//! 'postmaster' library

/// Used to organize pb files and share them between server and client
mod pb;

/// Prost generated code
pub use pb::messenger as prostgen;

/// Diesel types that map to the protobuf schema
pub mod schema;

/// Database pooling and connection handlers
pub mod db;
pub use db::pool;

/// Common protobuf schema
pub mod prelude {
    pub use super::prostgen::{
        messenger_client::MessengerClient, messenger_server::Messenger as MessengerServer,
    };
    pub use super::prostgen::{
        AllMsgsRequest, Msg, MsgInTransit, MsgRequest, MsgResponse, MultiMsgResponse,
        ReceivedMsgsRequest, SendResponse, SentMsgsRequest,
    };
}
