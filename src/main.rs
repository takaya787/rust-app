#[macro_use]
extern crate rocket;

extern crate app;
extern crate bcrypt;
extern crate diesel;

pub mod controllers;
pub mod helpers;

use bcrypt::verify;

use self::app::*;
// use self::diesel::prelude::*;
use self::models::*;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/delete_user")]
fn delete_user() -> &'static str {
    let conn = establish_connection();
    let result = helpers::users::delete_user(&conn, "sample");
    match result {
        Ok(user) => {
            println!("{:?}", user);
            "User deleted"
        }
        Err(e) => {
            println!("{:?}", e);
            "User not deleted"
        }
    }
}

#[launch]
fn rocket() -> _ {
    // helpers::fetch_microposts::fetch_microposts_each_user();
    // helpers::fetch_microposts::fetch_feed_relationship();
    rocket::build().mount(
        "/api",
        routes![
            world,
            delete_user,
            controllers::users::index,
            controllers::users::create,
        ],
    )
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
