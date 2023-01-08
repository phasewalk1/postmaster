#![forbid(unsafe_code)]
// Temporarily disable unused imports lint
#![deny(unused_crate_dependencies)]

use futures::Stream;
use postmaster::{
    db::pool::tonic::TONIC_POOL,
    prelude::*,
    prostgen::messenger_server::{Messenger, MessengerServer},
    schema::*,
};
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MessengerService {}

#[allow(dead_code)]
type ServerStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send>>;
#[allow(dead_code)]
type StreamResult<T> = Result<Response<T>, Status>;

fn decompose_request<T>(req: Request<T>) -> T
where
    T: prost::Message + Clone,
{
    let req = req.into_inner();
    log::info!("Got a request: {:#?}", req);
    return req;
}

// Temporarily disable unused parameters lint
#[allow(unused_variables)]
#[tonic::async_trait]
impl Messenger for MessengerService {
    async fn send_msg(
        &self,
        request: Request<MsgInTransit>,
    ) -> Result<Response<SendResponse>, Status> {
        let req = decompose_request(request);
        let new_msg: NewMsg<'_> = req.into();
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
        let req = decompose_request(request);

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

    async fn get_sent_msgs(
        &self,
        request: Request<SentMsgsRequest>,
    ) -> Result<Response<MultiMsgResponse>, Status> {
        let req = decompose_request(request);

        if let Ok(conn) = TONIC_POOL.try_connect() {
            if let Ok(msgs) = QueryableMsg::by_sender(req.sender.clone(), conn) {
                return Ok(Response::new(MultiMsgResponse { msgs }));
            } else {
                return Err(Status::not_found(format!(
                    "The msgs were not found: {:#?}",
                    req
                )));
            }
        } else {
            return Err(Status::internal(format!(
                "Failed to get a DB connection from the pool"
            )));
        }
    }

    async fn get_received_msgs(
        &self,
        request: Request<ReceivedMsgsRequest>,
    ) -> Result<Response<MultiMsgResponse>, Status> {
        let req = decompose_request(request);

        if let Ok(conn) = TONIC_POOL.try_connect() {
            if let Ok(msgs) = QueryableMsg::by_recipient(req.recipient.clone(), conn) {
                return Ok(Response::new(MultiMsgResponse { msgs }));
            } else {
                return Err(Status::not_found(format!(
                    "The msgs were not found: {:#?}",
                    req
                )));
            }
        } else {
            return Err(Status::internal(format!(
                "Failed to get a DB connection from the pool"
            )));
        }
    }

    async fn get_all(
        &self,
        request: Request<AllMsgsRequest>,
    ) -> Result<Response<MultiMsgResponse>, Status> {
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
