use std::borrow::Borrow;
use crate::kv::{Kv};
use crate::config::Config;

use tonic::Request;
use tonic::transport::Channel;
use proto::service::kv_client::KvClient;
use proto::service::watch_client::WatchClient;
use proto::service::observe_client::ObserveClient;
use proto::service::{PutRequest, RangeRequest};

#[derive(Clone)]
pub struct Client {
    kv_client: KvClient<Channel>,
    watch_client: WatchClient<Channel>,
    observe_client: ObserveClient<Channel>
}

impl Client {
    pub fn new(c: &Config) -> Self {
        async {
            let channel = Channel::from_static(c.host)
                .connect()
                .await
                .unwrap();
            Client{
                kv_client: KvClient::new(channel),
                watch_client: WatchClient::new(channel.clone()),
                observe_client: ObserveClient::new(channel.clone())
            }
        }
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let req = Request::new(
           PutRequest{
               key,
               value,
               ignore_value: false
           }
        );
        self.kv_client.put(req)
    }

    pub fn range(&mut self, key: Vec<u8>) -> Vec<u8> {
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
        let resp = self.kv_client.range(req);
    }

    pub fn watch() {

    }
}
