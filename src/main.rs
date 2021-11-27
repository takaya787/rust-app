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

#[get("/index")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/create_user")]
fn create_user() -> &'static str {
    let conn = establish_connection();
    let result = helpers::users::create_user(&conn, "sample", "sample@gmail.com", "password");
    match result {
        Ok(user) => {
            println!("{:?}", user);
            "User created"
        }
        Err(e) => {
            println!("{:?}", e);
            "User not created"
        }
    }
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
            index,
            world,
            create_user,
            delete_user,
            controllers::users::index
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
