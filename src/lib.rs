// #[macro_use]をglobalで設定するために、extern crateが必要
#[macro_use]
extern crate diesel;
extern crate dotenv;

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
use models::{NewUser, User};
pub mod schema;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  // println!("{}", database_url);
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, email: &'a str, password: &str) -> User {
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
    activated: Some(false),
    activated_at: None,
    reset_digest: None,
    reset_sent_at: None,
  };

  diesel::insert_into(users::table)
    .values(&new_user)
    .get_result(conn)
    .expect("Error saving new user")
}
