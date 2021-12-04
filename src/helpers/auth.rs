use crate::*;
use diesel::prelude::{PgConnection, QueryResult};
use models::indexes::*;
use models::tables::*;

pub fn get_user_microposts(
  conn: &PgConnection,
  current_user: &User,
) -> QueryResult<Vec<Micropost>> {
  let belonging_microposts = Micropost::belonging_to(current_user).load::<Micropost>(conn);

  belonging_microposts
}

pub fn convert_to_login_user_index(login_user: &User) -> LoginUserIndex {
  LoginUserIndex {
    id: login_user.id.clone(),
    name: login_user.name.clone().unwrap(),
    email: login_user.email.clone().unwrap(),
    created_at: login_user.created_at.clone().to_string(),
    activated: login_user.activated.clone().unwrap(),
    activated_at: login_user.activated_at.clone().unwrap().to_string(),
    gravator_url: get_gravator_url(&login_user.email.as_ref().unwrap()),
  }
}
