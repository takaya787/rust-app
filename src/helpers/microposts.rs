use crate::*;
use diesel::prelude::{PgConnection, QueryResult};
use models::tables::Micropost;

pub fn get_all_microposts(conn: &PgConnection) -> QueryResult<Vec<Micropost>> {
  use crate::schema::microposts::dsl::*;

  microposts
    .limit(50)
    .order(id.desc())
    .load::<Micropost>(conn)
}
