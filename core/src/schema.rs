use crate::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = msg)]
pub struct NewMsg<'m> {
    pub sender: &'m str,
    pub recipient: &'m str,
    pub body: &'m str,
}

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = msg)]
pub struct QueryableMsg {
    pub id: i32,
    pub sender: String,
    pub recipient: String,
    pub body: String,
    pub timestamp: chrono::NaiveDateTime,
}

impl From<MsgInTransit> for NewMsg<'_> {
    fn from(msg: MsgInTransit) -> Self {
        // make sure to box each message field before leaking the memory,
        // otherwise the memory won't be deallocated when the function returns
        let sender: &mut str = Box::leak(msg.sender.into_boxed_str());
        let recipient: &mut str = Box::leak(msg.recipient.into_boxed_str());
        let body: &mut str = Box::leak(msg.text.into_boxed_str());
        return NewMsg {
            sender,
            recipient,
            body,
        };
    }
}

impl From<QueryableMsg> for Msg {
    fn from(msg: QueryableMsg) -> Self {
        Self {
            id: msg.id.to_string(),
            sender: msg.sender,
            recipient: msg.recipient,
            text: msg.body,
            sent_at: msg.timestamp.to_string(),
        }
    }
}

impl From<QueryableMsg> for SendResponse {
    fn from(msg: QueryableMsg) -> Self {
        Self {
            message_id: msg.id.to_string(),
            sent_at: msg.timestamp.to_string(),
        }
    }
}

impl From<Msg> for SendResponse {
    fn from(msg: Msg) -> Self {
        Self {
            message_id: msg.id,
            sent_at: msg.sent_at,
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = thread)]
pub struct NewThread<'t> {
    pub id: i32,
    pub peer1: &'t str,
    pub peer2: &'t str,
    pub messages: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = thread)]
pub struct QueryableThread {
    pub id: i32,
    pub peer1: String,
    pub peer2: String,
    pub messages: Option<Vec<i32>>,
}

impl From<QueryableThread> for CreateThreadResponse {
    fn from(thread: QueryableThread) -> Self {
        Self {
            thread_id: thread.id.to_string(),
        }
    }
}

table! {
    msg (id) {
        id -> Int4,
        sender -> Varchar,
        recipient -> Varchar,
        body -> Text,
        sent_at -> Timestamp,
    }
}

table! {
    thread (id) {
        id -> Int4,
        peer1 -> Varchar,
        peer2 -> Varchar,
        messages -> Nullable<Array<Int4>>,
    }
}

allow_tables_to_appear_in_same_query!(msg, thread,);
