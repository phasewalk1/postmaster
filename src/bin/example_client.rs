use common::prelude::*;
use common::prostgen::messenger_client::MessengerClient;
use common::prostgen::SendResponse;
use futures::TryStreamExt;
use tonic::Streaming;

pub type Client = MessengerClient<tonic::transport::Channel>;

#[tokio::main(flavor = "current_thread")]
async fn main() {}
