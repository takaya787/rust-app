use crate::*;
use helpers::auth::*;
use helpers::comman::*;
use models::indexes::*;
use rocket::get;
use rocket::serde::json::json;

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

      ApiResponse {
        status: Status::Ok,
        json: json!(login_index),
      }
    }
    Err(_) => {
      login_index = LoginIndex {
        user: login_user_index,
        microposts: vec![],
      };
      ApiResponse {
        status: Status::InternalServerError,
        json: json!(login_index),
      }
    }
  }
}

// #[post("/login")]
// pub fn login() {}
