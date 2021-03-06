use chrono::NaiveDateTime;
use serde::Serialize;

// get /users　index
#[derive(Debug, Serialize)]
pub struct UserIndex {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub gravator_url: String,
}

#[derive(Debug, Serialize)]
pub struct MicropostIndex {
  pub id: i64,
  pub content: String,
  pub user_id: i64,
  pub created_at: String,
}

// get /auto_login
#[derive(Debug, Serialize)]
pub struct LoginUserIndex {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub created_at: String,
  pub activated: bool,
  pub activated_at: String,
  pub gravator_url: String,
}

#[derive(Debug, Serialize)]
pub struct LoginIndex {
  pub user: LoginUserIndex,
  pub microposts: Vec<MicropostIndex>,
}

//get /auto_feed
#[derive(Debug, Serialize)]
pub struct MicropostFeedIndex {
  pub id: i64,
  pub content: String,
  pub user_id: i64,
  pub created_at: String,
  pub gravator_url: String,
  pub name: String,
}

pub type MicropostFeedType = (
  i64,
  Option<String>,
  i64,
  NaiveDateTime,
  Option<String>,
  Option<String>,
);
