use common::{
    prelude::*,
    prostgen::{self, MsgInTransit, SendResponse},
    schema::QueryableMsg,
};
use diesel::prelude::*;
use futures::Stream;
use prostgen::messenger_server::{Messenger, MessengerServer};
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio_stream::{wrappers::ReceiverStream, StreamExt as TokioStreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Default, Debug)]
pub struct MessengerService {}

type ServerStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send>>;
type StreamResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Messenger for MessengerService {
    type GetAllStream = ServerStream;

    async fn send_msg(
        &self,
        request: Request<MsgInTransit>,
    ) -> Result<Response<SendResponse>, Status> {
        let msg: MsgInTransit = request.into_inner();
        println!("Got a message: {:?}", msg);

        let mut conn = common::db::establish_connection();
        let diesel_msg = common::schema::InsertableMsg::try_from(msg)
            .map_err(|_| Status::invalid_argument("Invalid timestamp"))?;
        let msg = diesel::insert_into(common::schema::msg::table)
            .values(&diesel_msg)
            .get_result::<common::schema::QueryableMsg>(&mut conn)
            .map_err(|_| Status::internal("Error inserting into database"))?;
        return Ok(Response::new(SendResponse {
            message_id: msg.id.to_string(),
            sent_at: chrono::Utc::now().naive_utc().to_string(),
        }));
    }

    async fn get_msg(&self, request: Request<MsgRequest>) -> Result<Response<Msg>, Status> {
        let msg_request: MsgRequest = request.into_inner();
        println!("Got a message request: {:?}", msg_request);

        let mut conn = common::db::establish_connection();
        let msg: QueryableMsg = common::schema::msg::table
            .filter(common::schema::msg::id.eq(msg_request.message_id.parse::<i64>().unwrap()))
            .get_result(&mut conn)
            .map_err(|_| Status::not_found("Message not found"))?;
        return Ok(Response::new(msg.into()));
    }

    async fn get_all(&self, request: Request<AllMsgsRequest>) -> StreamResult<ServerStream> {
        todo!()
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
