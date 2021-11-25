use super::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable, PartialEq)]
#[table_name = "users"]
pub struct User {
  pub id: i64,
  pub name: Option<String>,
  pub email: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub password_digest: Option<String>,
  pub admin: bool,
  pub activation_digest: Option<String>,
  pub activated: Option<bool>,
  pub activated_at: Option<NaiveDateTime>,
  pub reset_digest: Option<String>,
  pub reset_sent_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub name: Option<String>,
  pub email: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub password_digest: Option<String>,
  pub admin: bool,
  pub activation_digest: Option<String>,
  pub activated: Option<bool>,
  pub activated_at: Option<NaiveDateTime>,
  pub reset_digest: Option<String>,
  pub reset_sent_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(User)]
#[table_name = "microposts"]
pub struct Micropost {
  pub id: i64,
  pub content: Option<String>,
  pub user_id: i64,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "relationships"]
pub struct Relationship {
  pub id: i64,
  pub follower_id: Option<i32>,
  pub followed_id: Option<i32>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

// #[derive(Debug, Queryable, Identifiable, Associations)]
// #[belongs_to(User, foreign_key = "follower_id" as i64)]
// #[table_name = "relationships"]
// pub struct Following {
//   pub id: i64,
//   pub follower_id: Option<i32>,
//   pub followed_id: Option<i32>,
//   pub created_at: NaiveDateTime,
//   pub updated_at: NaiveDateTime,
// }

// #[derive(Debug, Queryable, Identifiable, Associations)]
// #[belongs_to(User, foreign_key = "followed_id" as i64)]
// #[table_name = "relationships"]
// pub struct Followed {
//   pub id: i64,
//   pub follower_id: Option<i32>,
//   pub followed_id: Option<i32>,
//   pub created_at: NaiveDateTime,
//   pub updated_at: NaiveDateTime,
// }
