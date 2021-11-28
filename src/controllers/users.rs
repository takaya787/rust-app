use crate::{app, helpers, models};
use app::*;
use helpers::users::*;
use rocket::form::Form;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserIndex {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub gravatar_url: String,
}

#[get("/users")]
pub fn index() -> Result<Json<Vec<UserIndex>>, Json<String>> {
  let connection = establish_connection();

  let users = get_all_users(&connection);

  let users_json = match users {
    Ok(users) => {
      println!("{}", users.len());
      let users_index = users
        .iter()
        .map(|user| UserIndex {
          id: user.id,
          name: user.name.clone().unwrap(),
          email: user.email.clone().unwrap(),
          gravatar_url: get_gravator_url(&user.email.as_ref().unwrap()),
        })
        .collect::<Vec<UserIndex>>();
      Ok(Json(users_index))
    }
    Err(error) => {
      println!("{}", error.to_string());
      Err(Json(error.to_string()))
    }
  };

  users_json
}

#[post("/users", data = "<user_form>")]
pub fn create(user_form: Form<models::UserForm>) -> Result<Json<UserIndex>, Json<String>> {
  let conn = establish_connection();
  println!("{:?}", user_form);
  let result = helpers::users::create_user(&conn, user_form);
  let json = match result {
    Ok(user) => {
      let json = Json(UserIndex {
        id: user.id,
        name: user.name.clone().unwrap(),
        email: user.email.clone().unwrap(),
        gravatar_url: get_gravator_url(&user.email.as_ref().unwrap()),
      });
      Ok(json)
    }
    Err(error) => {
      println!("{:?}", error);
      Err(Json(error.to_string()))
    }
  };
  json
}
