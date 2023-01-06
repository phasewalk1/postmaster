//! 'common' library
//! Used to organize pb files and share them between server and client
mod pb;

/// Prost generated code
pub use pb::messenger as prostgen;

/// Common protobuf schema
pub mod prelude {
    pub use super::prostgen::{
        AllMsgsRequest, CreateThreadRequest, CreateThreadResponse, Msg, MsgInTransit, MsgRequest,
        MsgResponse, ReceivedMsgsRequest, SendResponse, SentMsgsRequest, ThreadRequest,
        ThreadResponse,
    };
}

/// Diesel types that map to the protobuf schema
pub mod schema;

/// Database pooling and connection handlers
pub mod db;
pub use db::pool;
