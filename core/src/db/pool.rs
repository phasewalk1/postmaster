use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use lazy_static::lazy_static;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::State;
use std::env::var as env;
use std::rc::Rc;

// A lazily initialized database url, needs to be set at runtime
lazy_static! {
    pub static ref DATABASE_URL: String =
        { env("DATABASE_URL").expect("DATABASE_URL must be set") };
}

/// A Postgres connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initialize a connection pool
pub fn init_pool() -> Pool {
    let mgr = ConnectionManager::<PgConnection>::new(&*DATABASE_URL);
    return Pool::builder()
        .max_size(10)
        .build(mgr)
        .expect("Failed to create pool.");
}

/// Raw pool state
#[derive(Debug)]
pub struct PoolState {
    state: Rc<diesel::r2d2::State>,
}

impl PoolState {
    /// Read the underlying state
    pub fn read(&self) -> &diesel::r2d2::State {
        // unwrap the RC
        return Rc::<diesel::r2d2::State>::as_ref(&self.state);
    }
}

impl From<Pool> for PoolState {
    fn from(pool: Pool) -> Self {
        Self {
            state: Rc::new(pool.state()),
        }
    }
}
