use crate::*;
use rocket::http::Status;
use rocket::serde::json::json;

// Result<LoginUser, UserAuthError>専用のerror Handler
pub fn handle_auth_error(error: UserAuthError) -> ApiResponse {
  println!("{:?}", error);
  match error {
    UserAuthError::NotFoundToken => ApiResponse::new(
      Status::Unauthorized,
      json!({
        "error": "not found token",
        "message": "Please login"
      }),
    ),
    UserAuthError::NotFoundUser => ApiResponse::new(
      Status::Unauthorized,
      json!({
        "error": "not found user",
        "message": "Your account is not found"
      }),
    ),
    UserAuthError::InvalidToken => ApiResponse::new(
      Status::Unauthorized,
      json!({
        "error": "invalid token",
        "message": "Your token is invalid. \n Please login again."
      }),
    ),
  }
}

pub fn get_gravator_url(email: &str) -> String {
  let digest = compute(email);
  let result = format!("https://secure.gravatar.com/avatar/{:x}", digest);
  result
}
