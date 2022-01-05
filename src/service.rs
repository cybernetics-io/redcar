use std::sync::Arc;
use std::pin::Pin;

use proto::service::health_check_response;
use proto::service::health_server::Health;
use proto::service::kv_server::Kv;
use proto::service::{PutRequest, PutResponse, RangeRequest, RangeResponse, WatchCancel, WatchCreate, WatchRequest, WatchResponse, ObserveCancel, ObserveCreate, ObserveRequest, ObserveResponse, HeartbeatRequest, StatusRequest, HealthCheckRequest, HealthCheckResponse};
use proto::txn::{KeyValue, Message};

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
use tokio::sync::{Mutex, watch, broadcast, RwLock};
use std::sync::Mutex as Mut;
use proto::service::observe_request::ObserveType;
use proto::service::watch_request::WatchType;
use backend::{Backend, DB, Type};
use tokio::time::sleep;
use proto::service::event_server::Event;
use proto::service::keepalive_server::Keepalive;
use trigger::{Action, Trigger};
use trigger::Type as ActionType;
use trigger::edge::Edge;
use trigger::level::Level;
use crate::ServingStatus;

const CHANNEL_BUFFER_SIZE: usize = 512;

type WatchResponseStream = ReceiverStream<Result<WatchResponse, Status>>;
type ObserveResponseStream = ReceiverStream<Result<ObserveResponse, Status>>;
type KeepaliveStream = ReceiverStream<Result<Message, Status>>;

#[derive(Clone)]
pub struct Service {
    pub kvs: KVService,
    pub evs: EventService,
    pub kas: KeepaliveService,
    pub hs: HealthService
}

impl Service {
    pub fn new(c: Config) -> Result<Service, Error> {
        let db = DB::new(&Type::LMDB, c.get_home());
        let kv = KV::new(Index::new(db), Trigger::new());
        let evs = EventService {
            trigger: Arc::clone(&kv.trigger),
        };
        let kvs = KVService {
            kv: Arc::new(Mutex::new(kv)),
        };
        let kas = KeepaliveService {};
        let hs = HealthService::new(Arc::new(RwLock::new(HashMap::new())));
        Ok(Service { kvs, evs, kas, hs })
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
pub struct EventService {
    trigger: Arc<Mut<Trigger>>,
}

#[tonic::async_trait]
impl Event for EventService {
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
        let mut wrx = self.trigger.lock().unwrap().edge(create.key.clone());
        tokio::spawn(async move {
            let wid = create.watch_id;
            while wrx.changed().await.is_ok() {
                /*
                let event = Event {
                    action: wrx.borrow().action,
                    kv: wrx.borrow().kv.clone(),
                };

                 */
                tx.send(Ok(WatchResponse {
                    watch_id: wid,
                    created: false,
                    canceled: false,
                    cancel_reason: "".to_string(),
                    //events: vec![event],
                    events: vec![],
                }))
                .await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type ObserveStream = ObserveResponseStream;

    async fn observe(
        &self,
        request: Request<Streaming<ObserveRequest>>,
    ) -> Result<Response<Self::ObserveStream>, Status> {
        let mut stream = request.into_inner();
        let (mut tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);

        tokio::spawn(async move {
            // listening on request stream
            while let Some(req) = stream.message().await.unwrap() {
                let create = match req.observe_type {
                    Some(req_type) => match req_type {
                        ObserveType::Create(create) => create,
                        ObserveType::Cancel(_) => {
                            panic!("aa")
                        }
                    },
                    None => {
                        todo!()
                    }
                };
                // sending data as soon it is available
                tx.send(Ok(ObserveResponse {
                    observe_id: 77,
                    created: false,
                    canceled: false,
                    cancel_reason: "".to_string(),
                    events: vec![],
                }))
                    .await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[derive(Clone)]
pub struct KeepaliveService {
}

#[tonic::async_trait]
impl Keepalive for KeepaliveService {
    type HeartbeatStream = KeepaliveStream;

    async fn heartbeat(&self, request: Request<Streaming<HeartbeatRequest>>) -> Result<Response<Self::HeartbeatStream>, Status> {
        todo!()
    }

    type StatusStream = KeepaliveStream;

    async fn status(&self, request: Request<StatusRequest>) -> Result<Response<Self::StatusStream>, Status> {
        todo!()
    }
}

type StatusPair = (watch::Sender<ServingStatus>, watch::Receiver<ServingStatus>);

#[derive(Clone)]
pub struct HealthService {
    statuses: Arc<RwLock<HashMap<String, StatusPair>>>,
}

impl HealthService {
    fn new(services: Arc<RwLock<HashMap<String, StatusPair>>>) -> Self {
        HealthService { statuses: services }
    }

    async fn service_health(&self, service_name: &str) -> Option<ServingStatus> {
        let reader = self.statuses.read().await;
        reader.get(service_name).map(|p| *p.1.borrow())
    }
}

#[tonic::async_trait]
impl Health for HealthService {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let service_name = request.get_ref().service.as_str();
        let status = self.service_health(service_name).await;

        match status {
            None => Err(Status::not_found("service not registered")),
            Some(status) => Ok(Response::new(HealthCheckResponse {
                status: health_check_response::ServingStatus::from(status) as i32,
            })),
        }
    }

    type WatchStream =
    Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + Sync + 'static>>;

    async fn watch(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let service_name = request.get_ref().service.as_str();
        let mut status_rx = match self.statuses.read().await.get(service_name) {
            None => return Err(Status::not_found("service not registered")),
            Some(pair) => pair.1.clone(),
        };

        let output = async_stream::try_stream! {
            // yield the current value
            let status = health_check_response::ServingStatus::from(*status_rx.borrow()) as i32;
            yield HealthCheckResponse { status };

            while let Ok(_) = status_rx.changed().await {
                let status = health_check_response::ServingStatus::from(*status_rx.borrow()) as i32;
                yield HealthCheckResponse { status };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::WatchStream))
    }
}
