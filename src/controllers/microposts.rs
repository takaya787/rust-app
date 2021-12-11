use crate::*;
// use diesel::prelude::{PgConnection, QueryResult};
// use helpers::common::*;
use helpers::microposts::*;
// use models::tables::*;

use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::{delete, get, post};
use serde::Serialize;

#[get("/microposts")]
pub fn index() -> ApiResponse {
  let connection = establish_connection();
  let all_microposts = get_all_microposts(&connection);

  #[derive(Debug, Serialize)]
  struct MicropostIndex {
    id: i64,
    content: String,
    user_id: i64,
    created_at: String,
    updated_at: String,
  }

  match all_microposts {
    Ok(microposts) => {
      let vector = microposts
        .into_iter()
        .map(|micropost| MicropostIndex {
          id: micropost.id,
          content: micropost.content.unwrap(),
          user_id: micropost.user_id,
          created_at: micropost.created_at.to_string(),
          updated_at: micropost.updated_at.to_string(),
        })
        .collect::<Vec<_>>();
      ApiResponse::new(Status::Ok, json!(vector))
    }
    Err(e) => ApiResponse::new(Status::NotFound, json!({"error": e.to_string()})),
  }
}

#[post("/microposts")]
pub fn create() -> ApiResponse {
  // let connection = establish_connection();
  ApiResponse::new(Status::Created, json!({"microposts": "create"}))
}
