#![feature(box_syntax)]
// #[macro_use]をglobalで設定するために、extern crateが必要
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use md5::compute;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::{json, Value};
use std::env;
use std::io::Cursor;

pub mod models;
pub mod schema;

// Controllerから返すレスポンスを定義
#[derive(Debug)]
pub struct ApiResponse {
  pub json: Value,
  pub status: Status,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiResponse {
  fn respond_to(self, _: &Request) -> response::Result<'o> {
    Response::build()
      .status(self.status)
      .header(ContentType::JSON)
      .streamed_body(Cursor::new(self.json.to_string()))
      .ok()
  }
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  // println!("{}", database_url);
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_gravator_url(email: &str) -> String {
  let digest = compute(email);
  let result = format!("https://secure.gravatar.com/avatar/{:x}", digest);
  result
}

pub fn handle_diesel_error<T>(result: &QueryResult<T>) -> Option<ApiResponse> {
  if let Err(e) = result {
    println!("{:?}", e);
    match e {
      Error::DatabaseError(kind, value) => Some(ApiResponse {
        status: Status::BadRequest,
        json: json!({
          "errors": {"email": vec!["has already been taken"]},"message": format!("{:?}",kind),"value": format!("{:?}",value)
        }),
      }),
      Error::NotFound => Some(ApiResponse {
        status: Status::NotFound,
        json: json!({
          "errors": {"email": vec!["is not found"]},
        }),
      }),
      _ => Some(ApiResponse {
        status: Status::InternalServerError,
        json: json!({
          "errors": "Internal server error happened"
        }),
      }),
    }
  } else {
    None
  }
}
