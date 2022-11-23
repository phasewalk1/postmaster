//! SB-MESSENGER SERVER
//!
//! This is the gRPC server for the SB-Messenger service. It is responsible for handling client
//! requests and storing/retrieving data from the database. Some methods implement server-side
//! streaming, while others are unary. The server is implemented using the Tonic library, which
//! provides a gRPC server implementation on top of the Tokio runtime.
#![forbid(dead_code)]
#![forbid(unsafe_code)]
#![forbid(unused_imports)]
use common::{
    prelude::*,
    prostgen::{self, MsgInTransit, ReceivedMsgsRequest, SendResponse, SentMsgsRequest},
    schema::QueryableMsg,
};
use diesel::prelude::*;
use futures::Stream;
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
        // Console log the message
        let msg: MsgInTransit = request.into_inner();
        println!("Got a message: {:?}", msg);

        // Establish PG connection
        let mut conn = common::db::establish_connection();

        // Convert MsgInTransit to an InsertableMsg Diesel object
        let diesel_msg = common::schema::InsertableMsg::try_from(msg)
            .map_err(|_| Status::invalid_argument("Invalid timestamp"))?;

        // Insert the message into the database
        let msg = diesel::insert_into(common::schema::msg::table)
            .values(&diesel_msg)
            .get_result::<common::schema::QueryableMsg>(&mut conn)
            .map_err(|_| Status::internal("Error inserting into database"))?;

        // Return relevant info to the client
        return Ok(Response::new(SendResponse {
            message_id: msg.id.to_string(),
            sent_at: chrono::Utc::now().naive_utc().to_string(),
        }));
    }

    async fn get_msg(&self, request: Request<MsgRequest>) -> Result<Response<Msg>, Status> {
        // Console log request
        let msg_request: MsgRequest = request.into_inner();
        println!("Got a message request: {:?}", msg_request);

        // Establish PG connection
        let mut conn = common::db::establish_connection();

        // Query the database for the message
        let msg: QueryableMsg = common::schema::msg::table
            .filter(common::schema::msg::id.eq(msg_request.message_id.parse::<i64>().unwrap()))
            .get_result(&mut conn)
            // Returns a status error if the message is not found
            .map_err(|_| Status::not_found("Message not found"))?;

        // Return the message to the client
        return Ok(Response::new(msg.into()));
    }

    async fn get_all(&self, _request: Request<AllMsgsRequest>) -> StreamResult<ServerStream> {
        todo!()
    }

    async fn get_sent_msgs(
        &self,
        _request: Request<SentMsgsRequest>,
    ) -> StreamResult<ServerStream> {
        let sent_request: SentMsgsRequest = _request.into_inner();
        println!("Got a sent message request: {:?}", sent_request);

        // Establish PG connection
        let mut conn = common::db::establish_connection();

        // Query the database for the messages
        let msgs: Vec<QueryableMsg> = common::schema::msg::table
            .filter(common::schema::msg::sender.eq(sent_request.client_id))
            .get_results(&mut conn)
            // Returns a status error if the message is not found
            .map_err(|_| Status::not_found("Message not found"))?;

        // Return the message to the client
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            for msg in msgs {
                tx.send(Ok(msg.into())).await.unwrap();
            }
        });
        let stream = ReceiverStream::new(rx);
        return Ok(Response::new(Box::pin(stream) as ServerStream));
    }

    async fn get_received_msgs(
        &self,
        _request: Request<ReceivedMsgsRequest>,
    ) -> StreamResult<ServerStream> {
        let received_request: ReceivedMsgsRequest = _request.into_inner();
        println!("Got a received message request: {:?}", received_request);

        // Establish PG connection
        let mut conn = common::db::establish_connection();

        // Query the database for the messages
        let msgs_received: Vec<QueryableMsg> = common::schema::msg::table
            .filter(common::schema::msg::recipient.eq(received_request.client_id))
            .get_results(&mut conn)
            // Returns a status error if the message is not found
            .map_err(|_| Status::not_found("Message not found"))?;

        // Return the stream
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            for msg in msgs_received {
                tx.send(Ok(msg.into())).await.unwrap();
            }
        });
        let stream = ReceiverStream::new(rx);
        return Ok(Response::new(Box::pin(stream) as ServerStream));
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let messenger = MessengerService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(MessengerServer::new(messenger))
        .serve(addr)
        .await
        .unwrap();
}
