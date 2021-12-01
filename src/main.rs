#[macro_use]
extern crate rocket;

extern crate app;
extern crate bcrypt;
extern crate diesel;

pub mod controllers;
pub mod helpers;

use self::app::*;
use bcrypt::verify;
// use self::diesel::prelude::*;
use self::models::tables::*;

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
    rocket::build().mount(
        "/api",
        routes![
            world,
            delete_user,
            controllers::users::index,
            controllers::users::create,
        ],
    )
    // .register(
    //     "/api",
    //     catchers![
    //         controllers::errors::not_found,
    //         controllers::errors::unprocessable_entity
    //     ],
    // )
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
