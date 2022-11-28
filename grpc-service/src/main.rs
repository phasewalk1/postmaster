#![forbid(unsafe_code)]
use carrera::db::pool::tonic::TONIC_POOL;
use carrera::prelude::*;
use carrera::prostgen::messenger_server::{Messenger, MessengerServer};
use carrera::schema::*;
use futures::{Stream, StreamExt};
use log::*;
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MessengerService {}

type ServerStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send>>;
type StreamResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Messenger for MessengerService {
    type GetSentMsgsStream = ServerStream;
    type GetReceivedMsgsStream = ServerStream;
    type GetAllStream = ServerStream;

    async fn send_msg(
        &self,
        request: Request<MsgInTransit>,
    ) -> Result<Response<SendResponse>, Status> {
        let new_msg: NewMsg<'_> = request.into_inner().into();
        log::info!("Received message: {:#?}", new_msg);

        let maybe_conn = TONIC_POOL.try_connect();
        match maybe_conn {
            Ok(conn) => {
                if let Ok(res) = new_msg.insert(conn) {
                    return Ok(Response::new(res));
                } else {
                    return Err(Status::internal(format!(
                        "Failed to send msg: {:?}",
                        new_msg
                    )));
                }
            }
            Err(e) => return Err(e),
        }
    }

    async fn get_msg(&self, request: Request<MsgRequest>) -> Result<Response<Msg>, Status> {
        let req = request.into_inner();
        log::info!("Got a request: {:#?}", req);

        if let Ok(conn) = TONIC_POOL.try_connect() {
            if let Ok(msg) = QueryableMsg::by_id(req.message_id.clone(), conn) {
                return Ok(Response::new(msg));
            } else {
                return Err(Status::not_found(format!(
                    "The msg was not found: {:#?}",
                    req
                )));
            }
        } else {
            return Err(Status::internal(format!(
                "Failed to get a DB connection from the pool"
            )));
        }
    }

    async fn get_sent_msgs(&self, request: Request<SentMsgsRequest>) -> StreamResult<ServerStream> {
        todo!();
    }

    async fn get_received_msgs(
        &self,
        request: Request<ReceivedMsgsRequest>,
    ) -> StreamResult<ServerStream> {
        todo!();
    }

    async fn get_all(&self, request: Request<AllMsgsRequest>) -> StreamResult<ServerStream> {
        todo!();
    }

    async fn create_thread(
        &self,
        request: Request<CreateThreadRequest>,
    ) -> Result<Response<CreateThreadResponse>, Status> {
        todo!();
    }

    async fn get_thread(
        &self,
        request: Request<ThreadRequest>,
    ) -> Result<Response<ThreadResponse>, Status> {
        todo!();
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let messenger = MessengerService::default();

    pretty_env_logger::try_init().ok();
    log::info!("Server listening on: {}", addr);

    Server::builder()
        .add_service(MessengerServer::new(messenger))
        .serve(addr)
        .await
        .unwrap()
}
