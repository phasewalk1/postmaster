use common::{prelude::*, prostgen};
use prostgen::messenger_client::MessengerClient;

async fn send_msg_test() {
    let mut client = MessengerClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let request = tonic::Request::new(MsgInTransit {
        sender: "test_sender".to_string(),
        recipient: "test_recipient".to_string(),
        text: "lalallaal this a test message".to_string(),
    });
    let response = client.send_msg(request).await.unwrap();
    println!("RESPONSE={:?}", response);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    send_msg_test().await;
}
