use crate::*;

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::{PgConnection, QueryResult};
use models::forms::UserForm;
use models::tables::{Micropost, NewUser, User};
use rocket::form::{Contextual, Form};
use rocket::http::Status;
use rocket::serde::json::json;

// Helper function to valdate a user form
pub fn handle_user_form_validation(user_form: Form<Contextual<'_, UserForm>>) -> ApiResponse {
  let err_item = user_form.context.errors().next().unwrap();
  let key: String = err_item.name.as_ref().unwrap().to_string();
  let value: String = err_item.kind.to_string();

  return ApiResponse {
    status: Status::UnprocessableEntity,
    json: json!({"errors":  {key: [value]}}),
  };
}

// GET /users
pub fn get_all_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
  use schema::users::dsl::*;

  let results = users.filter(activated.eq(true)).load::<User>(conn);

  results
}

// Post /users
impl NewUser {
  fn create(name: &str, email: String, password_digest: String) -> NewUser {
    NewUser {
      name: Some(String::from(name)),
      email: Some(email),
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
      password_digest: Some(password_digest),
      admin: false,
      activation_digest: None,
      activated: Some(true),
      activated_at: None,
      reset_digest: None,
      reset_sent_at: None,
    }
  }
}

pub fn create_user(conn: &PgConnection, userform: &UserForm) -> QueryResult<User> {
  use schema::users::dsl::*;

  let user_password_digest = hash(userform.password, DEFAULT_COST).expect("Error hashing password");

  let new_user = NewUser::create(
    userform.name,
    userform.email.to_lowercase(),
    String::from(user_password_digest),
  );

  diesel::insert_into(users)
    .values(&new_user)
    .get_result::<User>(conn)
}

// GET /users/:id
pub fn get_user_by_id(conn: &PgConnection, user_id: i64) -> QueryResult<User> {
  use schema::users::dsl::*;

  let result = users
    .filter(id.eq(user_id))
    .filter(activated.eq(true))
    .first::<User>(conn);

  result
}

pub fn get_microposts_by_user(conn: &PgConnection, user: &User) -> QueryResult<Vec<Micropost>> {
  use schema::microposts::dsl::*;

  let results = microposts
    .filter(user_id.eq(user.id))
    .load::<Micropost>(conn);

  results
}

//  DELETE /users/:id
pub fn delete_user(conn: &PgConnection, user_id: i64) -> QueryResult<User> {
  use schema::users::dsl::*;

  let deleted_user: QueryResult<User> =
    diesel::delete(users.filter(id.eq(user_id))).get_result(conn);

  deleted_user
}
