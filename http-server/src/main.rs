#![feature(proc_macro_hygiene, decl_macro)]
#![feature(stmt_expr_attributes)]
#![allow(non_snake_case)]
#![forbid(unsafe_code)]
#![deny(unused_imports)]

#[macro_use]
extern crate rocket;
extern crate postmaster;

// Database connection pool
use postmaster::pool::rocket as pool;
// Common protobuf
use postmaster::prelude::*;
// Common ORM schemas and proto conversions
use postmaster::schema::*;
// Rocket exposes serde json
use rocket::serde::json::Json;

// Our PoolGuard implementation that wraps a r2d2 connection pool
mod extension;

////////////////////////////////////////////////////////////////////////////////
///////////////////////////////  ROUTES  ///////////////////////////////////////
/// Send a message
///  @params: message: Message
///     * data - proto::Msg
#[post("/", data = "<msg>")]
fn send(msg: Json<MsgInTransit>, conn: extension::PoolGuard) -> Json<SendResponse> {
    let new_msg: NewMsg<'_> = msg.into_inner().into();
    let res = new_msg.insert(conn.0).unwrap();
    return Json(res.into());
}

#[post("/", data = "<msg_request>")]
fn get(msg_request: Json<MsgRequest>, conn: extension::PoolGuard) -> Json<SendResponse> {
    let search_id = msg_request.into_inner().message_id;
    let res = QueryableMsg::by_id(search_id, conn.0).unwrap();
    return Json(res.into());
}

#[post("/", data = "<sender>")]
fn sent(sender: Json<SentMsgsRequest>, conn: extension::PoolGuard) -> Json<Vec<Msg>> {
    let sender = sender.into_inner().sender;
    let res = QueryableMsg::by_sender(sender, conn.0).unwrap();
    return Json(res.into());
}

#[post("/", data = "<recipient>")]
fn received(recipient: Json<ReceivedMsgsRequest>, conn: extension::PoolGuard) -> Json<Vec<Msg>> {
    let recipient = recipient.into_inner().recipient;
    let res = QueryableMsg::by_recipient(recipient, conn.0).unwrap();
    return Json(res.into());
}

#[launch]
fn rocket() -> _ {
    let DB_POOL = pool::init_pool();

    rocket::build()
        .manage(DB_POOL)
        .mount("/api/v1/send", routes![send])
        .mount("/api/v1/get", routes![get])
        .mount("/api/v1/sent", routes![sent])
        .mount("/api/v1/received", routes![received])
}
