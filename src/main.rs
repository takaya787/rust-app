#[macro_use]
extern crate rocket;

extern crate bcrypt;
extern crate diesel;
extern crate rails_demo;

pub mod controllers;
pub mod helpers;

use self::controllers::*;
use crate::rails_demo::*;
// use self::diesel::prelude::*;

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
        routes![world, delete_user, users::index, users::create, users::show,],
    )
}
