use std::borrow::Borrow;
use std::fmt::Error;
use std::future::Future;
use std::hash::Hasher;
use crate::config::Config;

use tonic::{Request, Response};
use tonic::transport::Channel;
use proto::service::kv_client::KvClient;
use proto::service::watch_client::WatchClient;
use proto::service::observe_client::ObserveClient;
use proto::service::{PutRequest, RangeRequest, RangeResponse};

#[derive(Clone)]
pub struct Client {
    kv_client: KvClient<Channel>,
}

impl Client {
    pub fn new(c: &Config) -> Self {
        let channel = Channel::from_static(c.host)
            .connect_lazy()
            .unwrap();
        Client{
            kv_client: KvClient::new(channel),
        }
    }

    pub async fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let req = Request::new(
            PutRequest{
                key,
                value,
                ignore_value: false
            }
        );
        self.kv_client.put(req)
            .await
            .unwrap();
    }

    pub async fn range(&mut self, key: Vec<u8>) -> Vec<Vec<u8>> {
        let req = Request::new(
            RangeRequest{
                key,
                limit: 0,
                sort_order: 0,
                sort_target: 0,
                keys_only: false,
                count_only: false
            }
        );
        let resp = self.kv_client.range(req)
            .await
            .unwrap();
        let kvs =  resp.get_ref().kvs.clone();
        let mut vv: Vec<Vec<u8>> = Vec::new();
        for kv in kvs.iter() {
            vv.push(kv.value.clone());
        }
        vv
    }
}
