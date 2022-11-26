use carrera::pool::Pool;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::request::{self, FromRequest, Request};
use rocket::State;

/// A wrapper around the connection pool that acts as a request guard
pub struct PoolGuard(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PoolGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let outcome = request
            .guard::<&State<Pool>>()
            .await
            .map(|pool| pool.inner().clone());
        match outcome {
            rocket::outcome::Outcome::Success(pool) => match pool.get() {
                Ok(conn) => rocket::outcome::Outcome::Success(PoolGuard(conn)),
                Err(_) => rocket::outcome::Outcome::Failure((
                    rocket::http::Status::ServiceUnavailable,
                    (),
                )),
            },
            rocket::outcome::Outcome::Forward(_) => rocket::outcome::Outcome::Forward(()),
            rocket::outcome::Outcome::Failure((status, _)) => {
                rocket::outcome::Outcome::Failure((status, ()))
            }
        }
    }
}

impl std::ops::Deref for PoolGuard {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
