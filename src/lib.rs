//! 'common' library
//! Used to organize pb files and share them between server and client

mod pb;
/// Prost generated code
pub use pb::messenger as prostgen;

/// Common schema
pub mod prelude {
    pub use super::prostgen::{AllMsgsRequest, Msg, MsgRequest, MsgResponse};
}
