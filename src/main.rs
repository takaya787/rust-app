#[macro_use]
extern crate rocket;

extern crate app;
extern crate bcrypt;
extern crate diesel;

use bcrypt::verify;

use self::app::*;
// use self::diesel::prelude::*;
use self::models::*;

pub mod features;
// fn main() {

// println!("Displaying users {:#?}", result_users);

// println!("Displaying models {:#?}", result_microposts);

// let new_user = create_user(
//     &connection,
//     "sample_name",
//     "railexample@gmail.com",
//     "password",
// );
// print_user(&new_user);

// delete_user(&connection, &new_user.name.unwrap());
// }

#[get("/index")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}
#[launch]
fn rocket() -> _ {
    // features::fetch_microposts::fetch_microposts_each_user();
    // features::fetch_microposts::fetch_feed_relationship();
    rocket::build().mount("/hello", routes![index, world])
}

fn _print_user(user: &User) {
    println!("user_name: {}", user.name.as_ref().unwrap());
    let user_password: String = String::from(user.password_digest.as_ref().unwrap());
    println!("password_salt: {}", &user_password[0..30]);
    println!("password_hash: {}", &user_password[30..]);
    // println!("password_digest: {}", user_password);
    println!("------------------------------------------------");
    println!("valid: {}", verify("password", &user_password).unwrap());
    println!("------------------------------------------------");
}
