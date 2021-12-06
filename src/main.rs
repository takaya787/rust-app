#[macro_use]
extern crate rocket;

use controllers::*;
use rails_demo::controllers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            users::index,
            users::create,
            users::show,
            users::delete,
            auth::auto_login,
            auth::login
        ],
    )
}
