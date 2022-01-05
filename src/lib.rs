// Copyright 2021 Redcar Project Authors. Licensed under Apache-2.0.

pub mod os;
pub mod config;
pub mod daemon;
pub mod kv;
pub mod event;
pub mod keepalive;
pub mod service;
pub mod txn;
pub mod utils;

use std::fmt::{Display, Formatter};
use proto::service::health_check_response;

/// An enumeration of values representing gRPC service health.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ServingStatus {
    /// Unknown status
    Unknown,
    /// The service is currently up and serving requests.
    Serving,
    /// The service is currently down and not serving requests.
    NotServing,
}

impl Display for ServingStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServingStatus::Unknown => f.write_str("Unknown"),
            ServingStatus::Serving => f.write_str("Serving"),
            ServingStatus::NotServing => f.write_str("NotServing"),
        }
    }
}

impl From<ServingStatus> for health_check_response::ServingStatus {
    fn from(s: ServingStatus) -> Self {
        match s {
            ServingStatus::Unknown => health_check_response::ServingStatus::Unknown,
            ServingStatus::Serving => health_check_response::ServingStatus::Serving,
            ServingStatus::NotServing => health_check_response::ServingStatus::NotServing,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
