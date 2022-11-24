use crate::schema::msg::dsl::*;
use crate::schema::*;
use diesel::prelude::*;

pub async fn get_msg(
    msg_request: crate::prelude::MsgRequest,
) -> Result<crate::prelude::Msg, diesel::result::Error> {
    let mut conn = crate::db::connection::unary_connection();
    let diesel_msg: QueryableMsg = msg
        .filter(id.eq(msg_request.message_id.parse::<i64>().unwrap()))
        .get_result(&mut conn)?;
    Ok(diesel_msg.into())
}

pub async fn get_msg_by_sender(
    sent_request: crate::prelude::SentMsgsRequest,
) -> Result<Vec<crate::prelude::Msg>, diesel::result::Error> {
    let mut conn = crate::db::connection::unary_connection();
    let diesel_msgs: Vec<QueryableMsg> = msg
        .filter(sender.eq(sent_request.client_id))
        .get_results(&mut conn)?;
    Ok(diesel_msgs.into_iter().map(|m| m.into()).collect())
}

pub async fn get_msg_by_recipient(
    receiver_request: crate::prelude::ReceivedMsgsRequest,
) -> Result<Vec<crate::prelude::Msg>, diesel::result::Error> {
    let mut conn = crate::db::connection::unary_connection();
    let diesel_msgs: Vec<QueryableMsg> = msg
        .filter(recipient.eq(receiver_request.client_id))
        .get_results(&mut conn)?;
    Ok(diesel_msgs.into_iter().map(|m| m.into()).collect())
}
