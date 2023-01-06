use super::PoolConn;
use crate::prelude::*;
use crate::schema::{NewMsg, QueryableMsg};
use crate::schema::{NewThread, QueryableThread};
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

impl<'t> NewThread<'t> {
    pub fn insert<T>(
        &self,
        mut conn: PoolConn<T>,
    ) -> Result<CreateThreadResponse, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let thread: QueryableThread = diesel::insert_into(crate::schema::thread::table)
            .values(self)
            .get_result(&mut conn)?;
        return Ok(thread.into());
    }

    /// Query thread by id and return the thread if it exists (even if it's empty)
    pub fn by_id<T>(
        // parsed as i64
        id: String,
        mut conn: PoolConn<T>,
    ) -> Result<ThreadResponse, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let thread: QueryableThread = crate::schema::thread::table
            .filter(crate::schema::thread::id.eq(id.parse::<i32>().unwrap()))
            .first(&mut conn)?;
        if let Some(msg_ids) = thread.messages {
            let msgs: Vec<QueryableMsg> = crate::schema::msg::table
                .filter(crate::schema::msg::id.eq_any(msg_ids))
                .load(&mut conn)?;
            return Ok(ThreadResponse {
                msgs: msgs.into_iter().map(|msg| msg.into()).collect(),
            });
        } else {
            return Ok(ThreadResponse { msgs: vec![] });
        }
    }

    pub fn by_participant<T>(
        sender: String,
        mut conn: PoolConn<T>,
    ) -> Result<Vec<ThreadResponse>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let threads: Vec<QueryableThread> = crate::schema::thread::table
            .filter(crate::schema::thread::peer1.eq(sender.clone()))
            .or_filter(crate::schema::thread::peer2.eq(sender))
            .load(&mut conn)?;
        return Ok(threads
            .into_iter()
            .map(|thread| {
                if let Some(msg_ids) = thread.messages {
                    let msgs: Vec<QueryableMsg> = crate::schema::msg::table
                        .filter(crate::schema::msg::id.eq_any(msg_ids))
                        .load(&mut conn)
                        .unwrap();
                    return ThreadResponse {
                        msgs: msgs.into_iter().map(|msg| msg.into()).collect(),
                    };
                } else {
                    return ThreadResponse { msgs: vec![] };
                }
            })
            .collect());
    }
}
