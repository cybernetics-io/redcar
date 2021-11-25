use std::sync::Arc;
use std::pin::Pin;

use proto::service::kv_server::Kv;
use proto::service::{
    PutRequest, PutResponse, RangeRequest, RangeResponse, WatchCancel, WatchCreate, WatchRequest,
    WatchResponse, ObserveCancel, ObserveCreate, ObserveRequest, ObserveResponse,
};
use proto::txn::{Event, KeyValue};

use async_stream;
use futures::Stream;
use tonic::{IntoRequest, IntoStreamingRequest, Request, Streaming};
use tonic::Response;
use tonic::Status;
use index::index::Index;
use utils::Error;

use crate::config::Config;
use crate::kv::KV;
use std::collections::HashMap;
use std::hash::Hasher;
use std::time::Duration;
use futures_util::stream::StreamExt;
use futures_util::pin_mut;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tokio::sync::{Mutex, watch};
use std::sync::Mutex as Mut;
use proto::service::observe_request::ObserveType;
use proto::service::watch_request::WatchType;
use backend::{Backend, DB, Type};
use tokio::time::sleep;
use proto::service::observe_server::Observe;
use proto::service::watch_server::Watch;
use trigger::{Action, Trigger};
use trigger::Type as ActionType;
use trigger::edge::Edge;
use trigger::level::Level;

const CHANNEL_BUFFER_SIZE: usize = 512;

type WatchResponseStream = ReceiverStream<Result<WatchResponse, Status>>;
type ObserveResponseStream =
    Pin<Box<dyn Stream<Item = Result<ObserveResponse, Status>> + Send + Sync>>;

#[derive(Clone)]
pub struct Service {
    pub kvs: KVService,
    pub ws: WatchService,
    pub os: ObserveService,
}

impl Service {
    pub fn new(c: Config) -> Result<Service, Error> {
        let db = DB::new(&Type::LMDB, c.get_home());
        let kv = KV::new(Index::new(db), Trigger::new());
        let ws = WatchService {
            trigger: Arc::clone(&kv.trigger),
        };
        let os = ObserveService {
            trigger: Arc::clone(&kv.trigger),
        };
        let kvs = KVService {
            kv: Arc::new(Mutex::new(kv)),
        };
        Ok(Service { kvs, ws, os })
    }

    pub fn build(&mut self) {}

    pub fn clear(&mut self) {}
}

#[derive(Clone)]
pub struct KVService {
    pub kv: Arc<Mutex<KV>>,
}

#[tonic::async_trait]
impl Kv for KVService {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        let req = request.get_ref();
        self.kv
            .lock()
            .await
            .set(req.key.to_vec(), req.value.to_vec());
        Ok(Response::new(PutResponse { kv: None }))
    }
    async fn range(
        &self,
        request: Request<RangeRequest>,
    ) -> Result<Response<RangeResponse>, Status> {
        let req = request.get_ref();
        let val = self.kv.lock().await.get(req.key.as_slice());
        let mut kvs = Vec::new();
        kvs.push(KeyValue {
            key: req.key.to_vec(),
            create_revision: 0,
            mod_revision: 0,
            version: 0,
            value: val,
        });
        Ok(Response::new(RangeResponse {
            kvs,
            more: false,
            count: 0,
        }))
    }
}

#[derive(Clone)]
pub struct WatchService {
    trigger: Arc<Mut<Trigger>>,
}

#[tonic::async_trait]
impl Watch for WatchService {
    type WatchStream = WatchResponseStream;

    async fn watch(
        &self,
        request: Request<WatchRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let (mut tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        let req = request.get_ref();
        let create = match req.clone().watch_type {
            Some(req_type) => match req_type {
                WatchType::Create(create) => create,
                WatchType::Cancel(_) => {
                    todo!()
                }
            },
            None => {
                todo!()
            }
        };
        let (wtx, mut wrx) = watch::channel(Event {
            r#type: 0,
            kv: None,
        });
        self.trigger.lock().unwrap().register(
            create.key.clone(),
            Box::new(Edge {
                t: ActionType::Edge,
                sender: Arc::new(Mut::new(wtx)),
            }),
        );
        tokio::spawn(async move {
            while wrx.changed().await.is_ok() {
                let event = Event {
                    r#type: wrx.borrow().r#type,
                    kv: wrx.borrow().kv.clone(),
                };
                tx.send(Ok(WatchResponse {
                    watch_id: 88,
                    created: false,
                    canceled: false,
                    cancel_reason: "".to_string(),
                    events: vec![event],
                }))
                .await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[derive(Clone)]
pub struct ObserveService {
    trigger: Arc<Mut<Trigger>>
}

#[tonic::async_trait]
impl Observe for ObserveService {
    type ObserveStream = ObserveResponseStream;

    async fn observe(
        &self,
        request: Request<Streaming<ObserveRequest>>,
    ) -> Result<Response<Self::ObserveStream>, Status> {
        let mut stream = request.into_inner();

        //let output = async_stream::try_stream! {
        while let Some(observe) = stream.next().await {
            let observe = observe?;
            match observe.observe_type {
                Some(req_type) => {
                    match req_type {
                        ObserveType::Create(create) => {
                            //create
                            println!("find create request");
                            yield ObserveResponse{
                                observe_id: create.observe_id,
                                created: false,
                                canceled: false,
                                cancel_reason: "".to_string(),
                                events: vec![]
                            }
                        }
                        ObserveType::Cancel(cancel) => {
                            //cancel
                        }
                    };
                }
                None => {
                    println!("watch request error")
                }
            }
        }
        //};
        Ok(Response::new(Box::pin(output)))
    }
}
