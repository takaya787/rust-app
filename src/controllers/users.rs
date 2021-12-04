use crate::*;
use helpers::users::*;
use models::forms::*;
use models::indexes::*;
use models::tables::User;

use rocket::form::{Contextual, Form};
use rocket::http::Status;
use rocket::serde::json::json;
use rocket::{delete, get, post};

#[get("/users")]
pub fn index() -> ApiResponse {
  let connection = establish_connection();

  let result = get_all_users(&connection);

  let error_response = handle_diesel_error(&result);

  if error_response.is_some() {
    return error_response.unwrap();
  }

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
pub fn create(user_form: Form<Contextual<'_, UserForm>>) -> ApiResponse {
  let conn = establish_connection();

  if user_form.value.as_ref().is_none() {
    return validate_user_form(user_form);
  }

  let result = create_user(&conn, user_form.value.as_ref().unwrap());

  let error_response = handle_diesel_error(&result);

  if error_response.is_some() {
    return error_response.unwrap();
  }

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
        }
      );
      ApiResponse {
        status: Status::Ok,
        json: json,
      }
    }
    Err(_) => error_response.unwrap(),
  };
  response
}

#[get("/users/<id>")]
pub fn show(id: i64) -> ApiResponse {
  let conn = establish_connection();

  let result = get_user_by_id(&conn, id);

  let error_response = handle_diesel_error(&result);

  if error_response.is_some() {
    return error_response.unwrap();
  }

  let response = match result {
    Ok(user) => {
      let json: Value;

      let microposts = get_microposts_by_user(&conn, &user);
      if microposts.is_err() {
        json = json!({
          "message": "User is deleted successfully",
        });
      } else {
        let microposts_data = microposts
          .unwrap()
          .iter()
          .map(|micropost| MicropostIndex {
            id: micropost.id,
            content: micropost.content.clone().unwrap(),
            user_id: micropost.user_id.clone(),
            created_at: micropost.created_at.clone().to_string(),
          })
          .collect::<Vec<MicropostIndex>>();
        json = json!({
          "id": user.id,
          "name": user.name.clone().unwrap(),
          "email": user.email.clone().unwrap(),
          "created_at": user.created_at.clone().to_string(),
          "gravator_url": get_gravator_url(&user.email.as_ref().unwrap()),
          "microposts": microposts_data
        });
      }

      ApiResponse {
        status: Status::Ok,
        json: json,
      }
    }
    Err(_) => error_response.unwrap(),
  };
  response
}

#[delete("/users/<id>")]
pub fn delete(id: i64, current_user: User) -> ApiResponse {
  let conn = establish_connection();
  if current_user.id != id {
    return ApiResponse {
      status: Status::Forbidden,
      json: json!({
        "error": "You are not authorized to delete this user"
      }),
    };
  }

  let result = delete_user(&conn, id);

  let error_response = handle_diesel_error(&result);

  if error_response.is_some() {
    return error_response.unwrap();
  }

  let response = match result {
    Ok(user) => {
      let json = json!({
        "id": user.id,
        "name": user.name.clone().unwrap(),
        "email": user.email.clone().unwrap(),
        "created_at": user.created_at.clone().to_string(),
        "gravator_url": get_gravator_url(&user.email.as_ref().unwrap()),
      });
      ApiResponse {
        status: Status::Ok,
        json: json,
      }
    }
    Err(_) => error_response.unwrap(),
  };
  response
}
