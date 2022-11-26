#![feature(proc_macro_hygiene, decl_macro)]
#![feature(stmt_expr_attributes)]
#![allow(non_snake_case)]

#[rustfmt::skip]
#[macro_use] extern crate rocket;
#[rustfmt::skip]
#[macro_use] extern crate diesel;

extern crate carrera;
// Database connection pool
use carrera::pool;
// Common protobuf
use carrera::prelude::*;
// Common ORM schemas and proto conversions
use carrera::schema::*;
mod extension;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let DB_POOL = pool::init_pool();

    rocket::build().manage(DB_POOL).mount("/", routes![index])
}
