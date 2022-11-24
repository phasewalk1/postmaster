use crate::schema::*;
use diesel::prelude::*;

pub async fn create_msg(
    new_msg: crate::prelude::MsgInTransit,
) -> Result<crate::prelude::Msg, diesel::result::Error> {
    use crate::schema::msg::dsl::*;

    let mut conn = crate::db::connection::unary_connection();
    let diesel_msg: InsertableMsg = new_msg.into();
    let saved_msg = diesel::insert_into(msg)
        .values(diesel_msg)
        .get_result::<QueryableMsg>(&mut conn)?;
    Ok(saved_msg.into())
}
