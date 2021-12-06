use crate::*;
use helpers::auth::*;
use helpers::common::*;
use models::forms::LoginForm;
use models::indexes::*;
use rocket::form::Form;
use rocket::serde::json::json;
use rocket::{get, post};

#[post("/login", data = "<login_form>")]
pub fn login(login_form: Form<LoginForm<'_>>) -> ApiResponse {
  let login_user = get_login_user(&login_form.email);
  let error_response = handle_diesel_error(&login_user);

  if error_response.is_some() {
    return error_response.unwrap();
  }

  if let Ok(user) = login_user {
    let bcrypt_result =
      verify_user_password_digest(&login_form.password, user.password_digest.as_ref().unwrap());
    if bcrypt_result.is_ok() && bcrypt_result.unwrap() {
      return ApiResponse::new(
        Status::Ok,
        json!({
          "email": user.email
        }),
      );
    }
  }
  ApiResponse::new(
    Status::Unauthorized,
    json!({
      "message": "email or password is invalid"
    }),
  )
}

#[get("/auto_login")]
pub fn auto_login(key: Result<LoginUser, UserAuthError>) -> ApiResponse {
  let connection = establish_connection();

  let login_user = match key {
    Ok(user) => user,
    Err(err) => return handle_auth_error(err),
  };

  let login_index: LoginIndex;
  let login_user_index = convert_to_login_user_index(&login_user);
  let login_user_microposts = get_user_microposts(&connection, &login_user);

  match login_user_microposts {
    Ok(microposts) => {
      let microposts_index = microposts
        .into_iter()
        .map(|micropost| MicropostIndex {
          id: micropost.id,
          content: micropost.content.unwrap(),
          user_id: micropost.user_id,
          created_at: micropost.created_at.to_string(),
        })
        .collect::<Vec<_>>();
      login_index = LoginIndex {
        user: login_user_index,
        microposts: microposts_index,
      };
    }
    Err(_) => {
      login_index = LoginIndex {
        user: login_user_index,
        microposts: vec![],
      };
    }
  }
  ApiResponse::new(Status::Ok, json!(login_index))
}
