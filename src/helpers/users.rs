use crate::{models, schema};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use models::{NewUser, User};

pub fn create_user<'a>(
  conn: &PgConnection,
  name: &'a str,
  email: &'a str,
  password: &str,
) -> QueryResult<User> {
  use schema::users;

  let password_hash = hash(password, DEFAULT_COST).expect("Error hashing password");

  let new_user = NewUser {
    name: Some(String::from(name)),
    email: Some(String::from(email)),
    created_at: Utc::now().naive_utc(),
    updated_at: Utc::now().naive_utc(),
    password_digest: Some(String::from(password_hash)),
    admin: false,
    activation_digest: None,
    activated: Some(true),
    activated_at: None,
    reset_digest: None,
    reset_sent_at: None,
  };

  diesel::insert_into(users::table)
    .values(&new_user)
    .get_result::<User>(conn)
}

pub fn delete_user(conn: &PgConnection, user_name: &str) -> QueryResult<User> {
  use schema::users::dsl::*;

  let deleted_user: QueryResult<User> =
    diesel::delete(users.filter(name.eq(user_name))).get_result(conn);

  deleted_user
}
