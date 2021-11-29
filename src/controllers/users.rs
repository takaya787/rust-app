use crate::{app, helpers};
use app::*;
use helpers::users::*;
use models::forms::*;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserIndex {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub gravator_url: String,
}

#[get("/users")]
pub fn index() -> ApiResponse {
  let connection = establish_connection();

  let result = get_all_users(&connection);

  let error_response = handle_diesel_error(&result);

  let response = match result {
    Ok(users) => {
      println!("{}", users.len());
      let users_index = users
        .iter()
        .map(|user| UserIndex {
          id: user.id,
          name: user.name.clone().unwrap(),
          email: user.email.clone().unwrap(),
          gravator_url: get_gravator_url(&user.email.as_ref().unwrap()),
        })
        .collect::<Vec<UserIndex>>();
      ApiResponse {
        status: Status::Ok,
        json: json!(users_index),
      }
    }
    Err(_err) => error_response.unwrap(),
  };
  response
}

#[post("/users", data = "<user_form>")]
pub fn create(user_form: Form<UserForm>) -> ApiResponse {
  let conn = establish_connection();
  // println!("{:?}", user_form);
  let result = helpers::users::create_user(&conn, user_form);
  let error_response = handle_diesel_error(&result);

  let response = match result {
    Ok(user) => {
      let json = json!(
        {
        "user":  {
          "id": user.id,
          "name": user.name.clone().unwrap(),
          "email": user.email.clone().unwrap(),
          "gravator_url": get_gravator_url(&user.email.as_ref().unwrap()),
        },
        "gravator_url": get_gravator_url(&user.email.as_ref().unwrap()),
        "token": String::from("token"),
      });
      ApiResponse {
        status: Status::Ok,
        json: json,
      }
    }
    Err(_error) => error_response.unwrap(),
  };
  response
}
