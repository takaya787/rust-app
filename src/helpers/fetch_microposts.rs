use crate::diesel::prelude::*;
use crate::models::tables::{ActiveRelationship, Micropost, User};
use crate::*;

// return のtype Alias
pub type MicropostsTypeEachUser = std::vec::Vec<(
  (
    i64,
    std::option::Option<std::string::String>,
    std::option::Option<std::string::String>,
  ),
  std::vec::Vec<Micropost>,
)>;

pub fn fetch_microposts_each_user() -> MicropostsTypeEachUser {
  use schema::users::dsl::*;

  let connection = establish_connection();

  let result_users = users
    .filter(activated.eq(true))
    .limit(5)
    .load::<User>(&connection)
    .unwrap();

  let selected_users = users
    .limit(5)
    .select((id, name, email))
    .load::<(i64, Option<String>, Option<String>)>(&connection)
    .unwrap();

  let microposts = Micropost::belonging_to(&result_users)
    .limit(50)
    .load::<Micropost>(&connection)
    .unwrap()
    .grouped_by(&result_users);

  let data = selected_users
    .into_iter()
    .zip(microposts)
    .collect::<Vec<_>>();

  println!("microposts_data with each user = {:?}", data);
  data
}

pub fn fetch_feed_relationship() {
  use schema::*;
  let connection = establish_connection();

  fn get_first_user() -> User {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let result_user = users
      .filter(activated.eq(true))
      .first::<User>(&connection)
      .expect("User is not found");
    result_user
  }

  let first_user = get_first_user();

  let following_users_index = ActiveRelationship::belonging_to(&first_user)
    .select(relationships::dsl::followed_id)
    .load::<i64>(&connection)
    .unwrap()
    .into_iter()
    .collect::<Vec<i64>>();

  let following_users = users::table
    .filter(users::dsl::id.eq_any(following_users_index))
    .load::<User>(&connection)
    .expect("following_users is not found");

  let following_users_feed = Micropost::belonging_to(&following_users)
    .count()
    .get_result::<i64>(&connection);

  // let followed_users = Followed::belonging_to(&user)
  //   .limit(50)
  //   .load::<Following>(&connection)
  //   .unwrap();

  // println!("following_users: {:?}", following_users);
  println!("feed_microposts {:?}", following_users_feed);
}
