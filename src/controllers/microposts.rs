use crate::*;
use helpers::common::*;
use helpers::microposts::*;
use models::forms::*;

use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::json;
use rocket::{get, post};

use serde::Serialize;

#[derive(Debug, Serialize)]
struct MicropostIndex {
  id: i64,
  content: String,
  user_id: i64,
  created_at: String,
  updated_at: String,
}

#[get("/microposts")]
pub fn index() -> ApiResponse {
  let connection = establish_connection();
  let all_microposts = get_all_microposts(&connection);

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

#[post("/microposts", data = "<micropost_form>")]
pub fn create(
  micropost_form: Form<MicropostForm>,
  key: Result<LoginUser, UserAuthError>,
) -> ApiResponse {
  let login_user = match key {
    Ok(user) => user,
    Err(err) => return handle_auth_error(err),
  };

  let conn = establish_connection();

  let result = create_micropost(&conn, micropost_form, &login_user);

  match result {
    Ok(micropost) => {
      let micropost_index = MicropostIndex {
        id: micropost.id,
        content: micropost.content.unwrap(),
        user_id: micropost.user_id,
        created_at: micropost.created_at.to_string(),
        updated_at: micropost.updated_at.to_string(),
      };
      ApiResponse::new(
        Status::Created,
        json!({ "micropost": micropost_index, "message": "micropost is successfully created" }),
      )
    }
    Err(e) => ApiResponse::new(Status::UnprocessableEntity, json!({"error": e.to_string()})),
  }
}
