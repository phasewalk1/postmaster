#![cfg(test)]
#![allow(dead_code)]
use carrera::prostgen::CreateThreadResponse;
use carrera::schema::{QueryableMsg, QueryableThread};

#[tokio::test]
async fn test_threads() {
	pretty_env_logger::try_init().ok();
	
    let msg: QueryableMsg = QueryableMsg {
        id: 1,
        sender: "test_sender".to_owned(),
        recipient: "test_recipient".to_owned(),
        body: "lalallaal this a test message".to_owned(),
        timestamp: chrono::Utc::now().naive_utc(),
    };
    log::info!("test_recipient received a msg: {:?}", msg);

    let reply: QueryableMsg = QueryableMsg {
        id: 2,
        sender: "test_recipient".to_owned(),
        recipient: "test_sender".to_owned(),
        body: "lalallaal this a test thread".to_owned(),
        timestamp: chrono::Utc::now().naive_utc(),
    };
    log::info!("you received a reply from test_recipient: {:?}", reply);

    let thread = QueryableThread {
        id: 1,
        peer1: "test_sender".to_owned(),
        peer2: "test_recipient".to_owned(),
        messages: Some(vec![msg.id, reply.id]),
    };
    log::info!("thread created: {:?}", thread);

    let thread_response: CreateThreadResponse = thread.into();
    log::info!("thread_response to client: {:?}", thread_response);
}
