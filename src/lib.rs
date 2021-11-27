// #[macro_use]をglobalで設定するために、extern crateが必要
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use md5::compute;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  // println!("{}", database_url);
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn gravator_for(user: &models::User) -> String {
  // process input message hasher.update(user_email);
  let digest = compute(user.email.as_ref().unwrap());
  let result = format!("https://secure.gravatar.com/avatar/{:x}", digest);
  println!("{:?}", result);
  result
}
