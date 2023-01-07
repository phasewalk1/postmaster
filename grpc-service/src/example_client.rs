use postmaster::prelude::*;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::try_init().ok();
    let mut client = MessengerClient::connect("http://[::1]:50051").await?;

    let req = Request::new(MsgInTransit {
        sender: "prttyw1sh".into(),
        recipient: "Jane".into(),
        text: "Hello, Jane!".into(),
    });

    let response = client.send_msg(req).await?.into_inner();

    println!("RESPONSE={:?}", response);

    Ok(())
}
