pub mod ops;
pub mod pool;

use diesel::r2d2;

/// A generic connection pool
/// This type encapsulates behavior for both, the request guarded rocket pool and the lazy static gRPC pool
pub type PoolConn<T> = r2d2::PooledConnection<T>;
