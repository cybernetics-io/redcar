// Copyright 2021 redcar Project Authors. Licensed under Apache-2.0.

use std::fs::{self};
use std::path::Path;
use std::thread;

use prost::Message;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::runtime::Builder;
use tokio_stream::wrappers;
use tonic::transport::Server;

use proto::service::kv_server::KvServer;
use proto::service::observe_server::ObserveServer;
use proto::service::watch_server::WatchServer;

use utils::Error;

use crate::config::Config;
use crate::service::Service;
use crate::os;

pub struct Daemon {
    config: Config,
    home: String,
    host: String,
    service: Option<Service>,
}

impl Daemon {
    pub fn new(c: Config) -> Result<Self, Error> {
        c.validate().unwrap_or_else(|_err| panic!("config error"));
        let home = c.get_home();
        if !Path::new(home.as_str()).exists() {
            fs::create_dir(home.as_str())
                .unwrap_or_else(|err| panic!("create file error {:?}", err))
        }
        Ok(Daemon {
            home,
            config: c.clone(),
            host: c.get_host(),
            service: None,
        })
    }

    pub fn run(&mut self) {
        let service = match Service::new(self.config.clone()) {
            Ok(s) => s,
            Err(_e_) => {
                panic!("new service failure")
            }
        };
        let tid = thread::current().id();
        for i in 0..self.config.get_thread_number() {
            let svc = service.clone();
            let host = self.host.clone();
            thread::spawn(move || {
                os::thread_affinity(&[i]);
                let tid = thread::current().id();
                let r = Builder::new_current_thread().enable_all().build().unwrap();
                r.block_on(start_server(i, host.as_str(), svc, 1024));
            });
        }
    }

    pub fn exit(self) {
        match self.service {
            Some(mut s) => {
                s.clear();
            }
            None => {
                println!("None")
            }
        }
    }
}

async fn start_server(num: usize, host: &str, service: Service, backlog: u32) {
    println!("listen on: {:?}", host);
    let addr = host.parse().unwrap();
    let socket = TcpSocket::new_v4().unwrap();
    socket.set_reuseport(true).unwrap();
    socket.bind(addr);
    let listener = socket.listen(backlog).unwrap();
    let stream = wrappers::TcpListenerStream::new(listener);
    let kv = KvServer::new(service.kvs);
    let watch = WatchServer::new(service.ws);
    let observe = ObserveServer::new(service.os);
    Server::builder()
        .add_service(kv)
        .add_service(watch)
        .add_service(observe)
        .serve_with_incoming(stream)
        .await
        .unwrap();
}
