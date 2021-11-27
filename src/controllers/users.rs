use crate::{app, helpers};

// use models::User;
use app::*;
use helpers::users::*;

#[derive(Debug)]
pub struct UserIndex {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub gravatar_url: String,
}

#[get("/users")]
pub fn index() -> &'static str {
  let connection = establish_connection();

  let results = get_all_users(&connection);

  match results {
    Ok(users) => {
      println!("{}", users.len());
      let users_index = users
        .iter()
        .map(|user| UserIndex {
          id: user.id,
          name: user.name.clone().unwrap(),
          email: user.email.clone().unwrap(),
          gravatar_url: gravator_for(user.clone()),
        })
        .collect::<Vec<UserIndex>>();
      println!("{:?}", users_index);
    }
    Err(error) => {
      println!("{}", error.to_string());
    }
  }

  "Users index"
}
