use crate::*;
use chrono::Utc;
use diesel::prelude::{PgConnection, QueryResult};

use models::forms::MicropostForm;
use models::tables::{Micropost, NewMicropost, User};

use rocket::form::Form;

pub fn get_all_microposts(conn: &PgConnection) -> QueryResult<Vec<Micropost>> {
  use crate::schema::microposts::dsl::*;

  microposts
    .limit(50)
    .order(id.desc())
    .load::<Micropost>(conn)
}

pub fn create_micropost(
  conn: &PgConnection,
  micropost_form: Form<MicropostForm>,
  current_user: &User,
) -> QueryResult<Micropost> {
  use crate::schema::microposts::dsl::*;
  let new_micropost = NewMicropost {
    content: Some(String::from(micropost_form.content)),
    user_id: current_user.id,
    created_at: Utc::now().naive_utc(),
    updated_at: Utc::now().naive_utc(),
  };

  diesel::insert_into(microposts)
    .values(&new_micropost)
    .get_result::<Micropost>(conn)
}
