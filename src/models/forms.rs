use rocket::form::FromForm;
use serde::Deserialize;

#[derive(Debug, Deserialize, FromForm)]
pub struct UserForm<'r> {
  pub name: &'r str,
  pub email: &'r str,
  #[field(validate = dbg_eq(self.password_confirmation))]
  #[field(validate = len(7..20))]
  pub password: &'r str,
  #[field(validate = dbg_eq(self.password))]
  pub password_confirmation: &'r str,
}
