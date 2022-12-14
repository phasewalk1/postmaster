#![cfg(test)]
#![allow(dead_code)]
#[allow(unused_imports)]
use log::{error, info};
use postmaster::prelude::*;
use postmaster::schema::{NewMsg, QueryableMsg};

// Connect the client
async fn instantiate_client() -> MessengerClient<tonic::transport::Channel> {
    let client = MessengerClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    return client;
}

// Send a single msg (unary)
async fn send_msg_test() {
    let mut client = instantiate_client().await;
    let request = tonic::Request::new(MsgInTransit {
        sender: "test_sender".to_string(),
        recipient: "test_recipient".to_string(),
        text: "lalallaal this a test message".to_string(),
    });
    let response = client.send_msg(request).await.unwrap();
    dbg!("send_msg() RESPONSE={}", response);
}

// Receive a batch of sent msgs (server streaming)
async fn get_all_sent() {
    let mut client = instantiate_client().await;
    let request = tonic::Request::new(SentMsgsRequest {
        sender: "test_sender".to_string(),
    });

    let msgs = client
        .get_sent_msgs(request)
        .await
        .unwrap()
        .into_inner()
        .msgs;
    for msg in msgs {
        dbg!("get_sent_msgs() RESPONSE={}", msg);
    }
}

// Receive a batch of received msgs (server streaming)
async fn get_all_rec() {
    let mut client = instantiate_client().await;
    let request = tonic::Request::new(ReceivedMsgsRequest {
        recipient: "test_recipient".to_string(),
    });

    let msgs = client
        .get_received_msgs(request)
        .await
        .unwrap()
        .into_inner()
        .msgs;
    for msg in msgs {
        dbg!("get_received_msgs() RESPONSE={}", msg);
    }
}

#[tokio::test]
async fn diesel_proto_impl() {
    pretty_env_logger::try_init().ok();
    // ----------------------------------------
    // Convert Proto types to Diesel types
    // ----------------------------------------
    let proto_msg = MsgInTransit {
        sender: "John Doe".to_string(),
        recipient: "Jane Doe".to_string(),
        text: "Hello, world!".to_string(),
    };
    info!("(PROTO) MsgInTransit={:?}", &proto_msg);

    let diesel_msg: NewMsg = proto_msg.into();
    info!("(DIESEL) InsertableMsg={:?}", diesel_msg);

    // ----------------------------------------
    // Convert Diesel types to Proto types
    // ----------------------------------------
    let diesel_msg = QueryableMsg {
        id: 1,
        sender: "John Doe".to_string(),
        recipient: "Jane Doe".to_string(),
        body: "Hello, world!".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
    };
    info!("(DIESEL) QueryableMsg={:?}", &diesel_msg);

    let proto_full_msg: Msg = diesel_msg.into();
    info!("(PROTO) Msg={:?}", proto_full_msg);
}
