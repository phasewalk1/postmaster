use crate::prelude::{Msg, MsgInTransit};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

// Diesel TABLE
diesel::table! {
    msg (id) {
        id -> Int8,
        sent_at -> Timestamp,
        sender -> Varchar,
        recipient -> Varchar,
        content -> Text,
    }
}

/// Diesel: InsertableMsg <----> Proto: MsgInTransit
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = msg)]
pub struct InsertableMsg<'a> {
    pub sent_at: chrono::NaiveDateTime,
    pub sender: &'a str,
    pub recipient: &'a str,
    pub content: &'a str,
}

/// Diesel: QueryableMsg <----> Proto: Msg
#[derive(Debug, Queryable, Serialize)]
pub struct QueryableMsg {
    pub id: i64,
    pub sent_at: chrono::NaiveDateTime,
    pub sender: String,
    pub recipient: String,
    pub text: String,
}

/// Transit Errors
#[derive(Debug, PartialEq)]
pub enum TransitErr {
    TimestampParseError,
}

impl From<MsgInTransit> for InsertableMsg<'_> {
    fn from(msg: MsgInTransit) -> Self {
        let sender = Box::leak(msg.sender.into_boxed_str());
        let recipient = Box::leak(msg.recipient.into_boxed_str());
        let content = Box::leak(msg.text.into_boxed_str());
        return InsertableMsg {
            sent_at: chrono::Utc::now().naive_utc(),
            sender,
            recipient,
            content,
        };
    }
}

// Proto <--> Diesel
impl From<QueryableMsg> for Msg {
    fn from(msg: QueryableMsg) -> Self {
        return Msg {
            id: msg.id.to_string(),
            sent_at: msg.sent_at.to_string(),
            sender: msg.sender,
            recipient: msg.recipient,
            text: msg.text,
        };
    }
}
