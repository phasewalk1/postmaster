#[cfg(test)]
use postmaster::prelude::*;
use tonic::Request;

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::try_init().ok();
    let mut client = MessengerClient::connect("http://[::1]:50051").await?;

    let req = Request::new(MsgInTransit {
        sender: "prttyw1sh".into(),
        recipient: "Jane".into(),
        text: "Hello, Jane!".into(),
    });

    let status = client.send_msg(req).await;
    let res = status?.into_inner();
    log::debug!("RESPONSE={:?}", res);

    Ok(())
}
