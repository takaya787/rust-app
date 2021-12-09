use rocket::form::FromForm;
use serde::Deserialize;

#[derive(Debug, Deserialize, FromForm)]
pub struct UserForm<'r> {
  #[field(validate = len(7..20))]
  pub name: &'r str,
  pub email: &'r str,
  #[field(validate = len(7..20))]
  pub password: &'r str,
  #[field(validate = dbg_eq(self.password))]
  pub password_confirmation: &'r str,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct LoginForm<'r> {
  pub email: &'r str,
  pub password: &'r str,
}
