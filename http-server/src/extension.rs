use carrera::pool::Pool;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::State;

/// A wrapper around the connection pool that acts as a request guard
pub struct PoolGuard(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl From<r2d2::PooledConnection<ConnectionManager<PgConnection>>> for PoolGuard {
    fn from(conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self(conn)
    }
}

// Implement the request guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for PoolGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let outcome = request
            .guard::<&State<Pool>>()
            .await
            .map(|pool| pool.inner().clone());
        match outcome {
            Outcome::Success(pool) => match pool.get() {
                Ok(conn) => Outcome::Success(PoolGuard(conn)),
                Err(_) => Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
            },
            Outcome::Forward(_) => Outcome::Forward(()),
            Outcome::Failure((status, _)) => Outcome::Failure((status, ())),
        }
    }
}

// We need to be able to dereference the guard to get the underlying connection
impl std::ops::Deref for PoolGuard {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
