//! SB-MESSENGER SERVER
//!
//! This is the gRPC server for the SB-Messenger service. It is responsible for handling client
//! requests and storing/retrieving data from the database. Some methods implement server-side
//! streaming, while others are unary. The server is implemented using the Tonic library, which
//! provides a gRPC server implementation on top of the Tokio runtime.
#![forbid(unsafe_code)]
use common::{
    db::ops::{create_msg::*, read_msg::*},
    prelude::*,
    prostgen::{self, MsgInTransit, ReceivedMsgsRequest, SendResponse, SentMsgsRequest},
};
use futures::{Stream, StreamExt};
use prostgen::messenger_server::{Messenger, MessengerServer};
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default, Debug)]
pub struct MessengerService {}

type ServerStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send>>;
type StreamResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Messenger for MessengerService {
    type GetAllStream = ServerStream;
    type GetSentMsgsStream = ServerStream;
    type GetReceivedMsgsStream = ServerStream;

    async fn send_msg(
        &self,
        request: Request<MsgInTransit>,
    ) -> Result<Response<SendResponse>, Status> {
        // Get the message from the request
        let msg = request.into_inner();
        log::info!("Received message: {:?}", msg);

        // Insert the message into the database
        let saved_msg = create_msg(msg)
            .await
            .map_err(|e| Status::internal(format!("Error saving message to database: {:?}", e)))?;

        // Return the saved message to the client
        Ok(Response::new(SendResponse {
            message_id: saved_msg.id,
            sent_at: saved_msg.sent_at.to_string(),
        }))
    }

    async fn get_msg(&self, request: Request<MsgRequest>) -> Result<Response<Msg>, Status> {
        // Decouple the request
        let msg_request = request.into_inner();
        log::info!(
            "Received request for message with id: {}",
            msg_request.message_id
        );
        // Get the message from the database
        let msg_by_id: Msg = get_msg(msg_request).await.map_err(|e| {
            Status::internal(format!("Error getting message from database: {:?}", e))
        })?;
        // Return the message to the client
        return Ok(Response::new(msg_by_id));
    }

    async fn get_all(&self, _request: Request<AllMsgsRequest>) -> StreamResult<ServerStream> {
        todo!()
    }

    async fn get_sent_msgs(&self, request: Request<SentMsgsRequest>) -> StreamResult<ServerStream> {
        // Decouple the request
        let sent_request = request.into_inner();
        log::info!(
            "Received request for messages sent by: {}",
            sent_request.client_id
        );
        // Get the messages from the database
        let msgs_by_sender: Vec<Msg> = get_msg_by_sender(sent_request).await.map_err(|e| {
            Status::internal(format!("Error getting messages from database: {:?}", e))
        })?;

        // Turn the messages iterator into a stream
        let mut stream = Box::pin(tokio_stream::iter(msgs_by_sender));
        // Return the stream to the client
        let (tx, rx) = tokio::sync::mpsc::channel(512);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Error sending message to client: {:?}", e);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as ServerStream
        ))
    }

    async fn get_received_msgs(
        &self,
        request: Request<ReceivedMsgsRequest>,
    ) -> StreamResult<ServerStream> {
        let received_request = request.into_inner();
        log::info!(
            "Received request for messages received by: {}",
            received_request.client_id
        );
        let msgs_by_recipient: Vec<Msg> =
            get_msg_by_recipient(received_request).await.map_err(|e| {
                Status::internal(format!("Error getting messages from database: {:?}", e))
            })?;
        let mut stream = Box::pin(tokio_stream::iter(msgs_by_recipient));

        let (tx, rx) = tokio::sync::mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Error sending message to client: {:?}", e);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as ServerStream
        ))
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let messenger = MessengerService::default();

    pretty_env_logger::try_init().ok();
    log::info!("Starting server on: {}", addr);

    Server::builder()
        .add_service(MessengerServer::new(messenger))
        .serve(addr)
        .await
        .unwrap();
}
