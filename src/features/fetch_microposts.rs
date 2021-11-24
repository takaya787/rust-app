use crate::app::*;
use crate::diesel::prelude::*;
use crate::models::{Followed, Following, Micropost, User};

// return のtype Alias
pub type MicropostsTypeEachUser = std::vec::Vec<(
  (
    i64,
    std::option::Option<std::string::String>,
    std::option::Option<std::string::String>,
  ),
  std::vec::Vec<app::models::Micropost>,
)>;

pub fn fetch_microposts_each_user() -> MicropostsTypeEachUser {
  use schema::*;
  let connection = establish_connection();

  let users = users::table.limit(5).load::<User>(&connection).unwrap();

  let selected_users = users::table
    .limit(5)
    .select((users::id, users::name, users::email))
    .load::<(i64, Option<String>, Option<String>)>(&connection)
    .unwrap();

  let microposts = Micropost::belonging_to(&users)
    .limit(50)
    .load::<Micropost>(&connection)
    .unwrap()
    .grouped_by(&users);

  let data = selected_users
    .into_iter()
    .zip(microposts)
    .collect::<Vec<_>>();

  // println!("microposts_data with each user = {:?}", data);
  data
}

pub fn fetch_feed_relationship() {
  use schema::*;
  let connection = establish_connection();

  let user = users::table.first::<User>(&connection).unwrap();
  let following_users = Following::belonging_to(&user)
    .limit(50)
    .load::<Following>(&connection)
    .unwrap();

  let followed_users = Followed::belonging_to(&user)
    .limit(50)
    .load::<Following>(&connection)
    .unwrap();

  println!("following_users: {:?}", following_users);
  println!("followed_users: {:?}", followed_users);
}
