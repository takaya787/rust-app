use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
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

#[derive(Debug, Queryable)]
pub struct Micropost {
  pub id: i64,
  pub content: Option<String>,
  pub user_id: i64,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
