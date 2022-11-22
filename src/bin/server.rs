use common::{
    prelude::*,
    prostgen::{self, SendResponse},
};
use futures::{Stream, StreamExt};
use prostgen::messenger_server::{Messenger, MessengerServer};
use std::{pin::Pin, time::Duration};
use tokio_stream::{wrappers::ReceiverStream, StreamExt as TokioStreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Default, Debug)]
pub struct MessengerService {}

type ServerStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send>>;
type StreamResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Messenger for MessengerService {
    type GetAllStream = ServerStream;

    async fn send_msg(&self, request: Request<Msg>) -> Result<Response<SendResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = SendResponse {
            message_id: 1.to_string(),
        };
        Ok(Response::new(reply))
    }
    async fn get_msg(&self, request: Request<MsgRequest>) -> Result<Response<Msg>, Status> {
        println!("Got a request: {:?}", request);
        let reply = Msg {
            id: 1.to_string(),
            sender: "server".to_string(),
            recipient: "client".to_string(),
            sent_at: "now".to_string(),
            text: "Hello, world!".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn get_all(&self, request: Request<AllMsgsRequest>) -> StreamResult<ServerStream> {
        // create an infinite stream
        let rep = std::iter::repeat(Msg {
            id: 1.to_string(),
            sender: "server".to_string(),
            recipient: "client".to_string(),
            sent_at: "now".to_string(),
            text: "Hello, world!".to_string(),
        });
        let mut stream = Box::pin(tokio_stream::iter(rep).throttle(Duration::from_millis(200)));

        // handling disconnect functionality
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(msg) = TokioStreamExt::next(&mut stream).await {
                match tx.send(Result::<_, Status>::Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Client disconnected");
                        break;
                    }
                }
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as ServerStream))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {}
