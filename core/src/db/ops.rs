use super::PoolConn;
use crate::prelude::*;
use crate::schema::{NewMsg, QueryableMsg};
use diesel::prelude::*;

impl<'m> NewMsg<'m> {
    pub fn insert<T>(&self, mut conn: PoolConn<T>) -> Result<SendResponse, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let msg: QueryableMsg = diesel::insert_into(crate::schema::msg::table)
            .values(self)
            .get_result(&mut conn)?;
        return Ok(msg.into());
    }
}

impl QueryableMsg {
    pub fn by_id<T>(
        // parsed as i64
        id: String,
        mut conn: PoolConn<T>,
    ) -> Result<Msg, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let msg: QueryableMsg = crate::schema::msg::table
            .filter(crate::schema::msg::id.eq(id.parse::<i32>().unwrap()))
            .first(&mut conn)?;
        return Ok(msg.into());
    }

    pub fn by_sender<T>(
        sender: String,
        mut conn: PoolConn<T>,
    ) -> Result<Vec<Msg>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let msgs: Vec<QueryableMsg> = crate::schema::msg::table
            .filter(crate::schema::msg::sender.eq(sender))
            .load(&mut conn)?;
        return Ok(msgs.into_iter().map(|msg| msg.into()).collect());
    }

    pub fn by_recipient<T>(
        recipient: String,
        mut conn: PoolConn<T>,
    ) -> Result<Vec<Msg>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let msgs: Vec<QueryableMsg> = crate::schema::msg::table
            .filter(crate::schema::msg::recipient.eq(recipient))
            .load(&mut conn)?;
        return Ok(msgs.into_iter().map(|msg| msg.into()).collect());
    }
}
