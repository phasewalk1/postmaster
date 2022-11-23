//! 'common' library
//! Used to organize pb files and share them between server and client
mod pb;

/// Prost generated code
pub use pb::messenger as prostgen;

/// Common protobuf schema
pub mod prelude {
    pub use super::prostgen::{
        AllMsgsRequest, Msg, MsgInTransit, MsgRequest, MsgResponse, ReceivedMsgsRequest,
        SentMsgsRequest,
    };
}

/// Diesel types that map to the protobuf schema
pub mod schema;

/// Db connection
pub mod db {
    use diesel::prelude::*;
    use diesel::PgConnection;

    pub fn establish_connection() -> PgConnection {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
