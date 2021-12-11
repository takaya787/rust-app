use crate::*;
use bcrypt::verify;
use diesel::prelude::{PgConnection, QueryResult};
use helpers::common::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use models::indexes::*;
use models::tables::*;
use serde::{Deserialize, Serialize};

pub fn get_login_user(user_email: &str) -> QueryResult<User> {
  use schema::users::dsl::*;

  let conn = establish_connection();
  let login_user = users
    .filter(email.eq(user_email.to_string()))
    .first::<User>(&conn);
  login_user
}

pub fn create_user_token(user_id: i64) -> String {
  #[derive(Serialize, Deserialize)]
  struct UserToken {
    user_id: i64,
  }
  let user_token = UserToken { user_id };
  encode(
    &Header::default(),
    &user_token,
    &EncodingKey::from_secret("s3cr3t".as_ref()),
  )
  .unwrap()
}

pub fn verify_user_password_digest(password: &str, hash: &str) -> bool {
  let result = verify(password.as_bytes(), hash);
  match result {
    Ok(true) => true,
    _ => false,
  }
}

pub fn get_user_microposts(
  conn: &PgConnection,
  current_user: &User,
) -> QueryResult<Vec<Micropost>> {
  use crate::schema::microposts::dsl::*;
  let belonging_microposts = Micropost::belonging_to(current_user)
    .order(id.desc())
    .load::<Micropost>(conn);

  belonging_microposts
}

pub fn convert_to_user_index(user: &User) -> UserIndex {
  UserIndex {
    id: user.id,
    email: user.email.clone().unwrap(),
    name: user.name.clone().unwrap(),
    gravator_url: get_gravator_url(&user.email.as_ref().unwrap()),
  }
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

// GET "/auto_feed" 用のhelpers
pub fn get_following_users_indexes(conn: &PgConnection, current_user: &User) -> Vec<i64> {
  use schema::relationships::dsl::*;

  let following_users_index = ActiveRelationship::belonging_to(current_user)
    .select(followed_id)
    .load::<i64>(conn)
    .unwrap()
    .into_iter()
    .collect::<Vec<i64>>();

  following_users_index
}

pub fn get_microposts_feed_from_users_indexes(
  conn: &PgConnection,
  indexes: Vec<i64>,
  offset: i64,
  limit: i64,
) -> QueryResult<Vec<MicropostFeedType>> {
  use schema::{microposts, users};

  let microposts_feed = users::table
    .inner_join(microposts::table)
    .filter(users::dsl::id.eq_any(indexes))
    .select((
      microposts::dsl::id,
      microposts::dsl::content,
      microposts::dsl::user_id,
      microposts::dsl::created_at,
      users::dsl::email,
      users::dsl::name,
    ))
    .order(microposts::dsl::id.desc())
    .offset(offset)
    .limit(limit)
    .load::<MicropostFeedType>(conn);

  microposts_feed
}

pub fn convert_microposts_feed_to_index(
  microposts_feed: Vec<MicropostFeedType>,
) -> Vec<MicropostFeedIndex> {
  microposts_feed
    .into_iter()
    .map(|(id, content, user_id, created_at, email, name)| {
      let content = String::from(content.as_ref().unwrap());
      let created_at = created_at.to_string();
      let gravator_url = get_gravator_url(email.as_ref().unwrap());
      let name = String::from(name.as_ref().unwrap());
      MicropostFeedIndex {
        id,
        content,
        user_id,
        created_at,
        gravator_url,
        name,
      }
    })
    .collect::<Vec<_>>()
}
