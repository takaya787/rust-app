// #[macro_use]をglobalで設定するために、extern crateが必要
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use md5::compute;
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::{json, Value};
use serde::Deserialize;
use std::env;
use std::io::Cursor;

pub mod controllers;
pub mod helpers;
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

// User認証を行うRequestGuard
#[derive(Debug)]
pub struct UserAuthGuard {
  pub current_user: models::tables::User,
}

#[derive(Debug)]
pub enum UserAuthError {
  NotFoundToken,
  InvalidToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuthGuard {
  type Error = UserAuthError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let token: &str = match request.headers().get_one("Authorization") {
      Some(token) => token.split_whitespace().collect::<Vec<&str>>()[1],
      None => return Outcome::Failure((Status::Unauthorized, UserAuthError::NotFoundToken)),
    };

    #[derive(Debug, Deserialize)]
    struct Token {
      user_id: i64,
    }

    let decoded_token = decode::<Token>(
      &token,
      &DecodingKey::from_secret("s3cr3t".as_ref()),
      &Validation {
        validate_exp: false,
        ..Default::default()
      },
    );
    // println!(
    //   "decoded_token: {:?}",
    //   decoded_token.as_ref().unwrap().claims
    // );

    if let Err(_) = decoded_token {
      return Outcome::Failure((Status::Unauthorized, UserAuthError::InvalidToken));
    }

    let conn = establish_connection();
    let current_user = helpers::users::get_user_by_id(&conn, decoded_token.unwrap().claims.user_id);

    match current_user {
      Ok(user) => Outcome::Success(UserAuthGuard { current_user: user }),
      Err(_) => Outcome::Failure((Status::Unauthorized, UserAuthError::InvalidToken)),
    }
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
    match e {
      Error::DatabaseError(_, _) => Some(ApiResponse {
        status: Status::BadRequest,
        json: json!({
          "errors": {"email": vec!["has already been taken"]}
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
