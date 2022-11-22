use common::{prelude::*, prostgen};
use prostgen::messenger_client::MessengerClient;

#[cfg(test)]
async fn send_test() {
    let mut client = MessengerClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let request = tonic::Request::new(prostgen::Msg {
        id: "1".to_string(),
        sender: "client".to_string(),
        recipient: "server".to_string(),
        sent_at: "now".to_string(),
        text: "Hello, world!".to_string(),
    });
    let response = client.send_msg(request).await.unwrap();
    println!("RESPONSE={:?}", response);
}

#[allow(dead_code)]
async fn stream_test() {
    let mut client = MessengerClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let request = tonic::Request::new(prostgen::AllMsgsRequest {
        client_id: "1".to_string(),
    });
    let mut stream = client.get_all(request).await.unwrap().into_inner();
    while let Some(msg) = stream.message().await.unwrap() {
        println!("GOT={:?}", msg);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    stream_test().await;
}
