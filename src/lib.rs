//! An asynchronously etcd client for Rust.
//!
//! etcd-rs supports etcd v3 API and async/await syntax.

pub use auth::{
    AuthDisableResponse, AuthEnableResponse, AuthOp, AuthRoleAddRequest, AuthRoleAddResponse,
    AuthRoleDeleteRequest, AuthRoleDeleteResponse, AuthRoleListResponse, AuthStatusRequest,
    AuthStatusResponse, AuthenticateRequest, AuthenticateResponse,
};
pub use cluster::{
    ClusterOp, Member, MemberAddRequest, MemberAddResponse, MemberListRequest, MemberListResponse,
    MemberRemoveRequest, MemberRemoveResponse, MemberUpdateRequest, MemberUpdateResponse,
};
pub use kv::{
    CompactRequest, CompactResponse, DeleteRequest, DeleteResponse, KeyRange, KeyValue, KeyValueOp,
    PutRequest, PutResponse, RangeRequest, RangeResponse, TxnCmp, TxnOp, TxnOpResponse, TxnRequest,
    TxnResponse,
};
pub use lease::{
    LeaseGrantRequest, LeaseGrantResponse, LeaseId, LeaseKeepAlive, LeaseKeepAliveRequest,
    LeaseKeepAliveResponse, LeaseOp, LeaseRevokeRequest, LeaseRevokeResponse,
    LeaseTimeToLiveRequest, LeaseTimeToLiveResponse,
};
pub use response_header::ResponseHeader;
pub use watch::{
    Event, EventType, WatchCancelRequest, WatchCanceler, WatchCreateRequest, WatchInbound, WatchOp,
    WatchResponse, WatchStream,
};

pub use client::{Client, ClientConfig, Endpoint};
pub use error::Error;

mod auth;
mod client;
mod cluster;
mod error;
mod kv;
mod lease;
mod lock;
mod proto;
mod response_header;
mod watch;

pub type Result<T> = std::result::Result<T, Error>;
