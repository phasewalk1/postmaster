#![cfg(test)]
extern crate postmaster;

use postmaster::prelude::MsgInTransit;
use reqwest::{Client, Url};
use serde_json::json;

#[tokio::test]
async fn test_send_msg() {
    let msg = MsgInTransit {
        sender: "test_sender".to_string(),
        recipient: "test_recipient".to_string(),
        text: "lalallaal this a test message".to_string(),
    };

    let payload = json!(msg);
    let client = Client::new();

    let url = Url::parse("http://localhost:8000/api/v1/send").unwrap();
    let res = client
        .post(url)
        .body(serde_json::to_string(&payload).unwrap())
        .send()
        .await
        .unwrap();

    println!("Status: {}", res.status());
    assert!(res.status().is_success());
}
