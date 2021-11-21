extern crate app;
extern crate bcrypt;
extern crate diesel;

use bcrypt::{hash, verify, DEFAULT_COST};

use self::app::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use app::schema::microposts::dsl::*;
    use app::schema::users::dsl::*;

    let connection = establish_connection();

    let _result_users = users
        .filter(activated.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    let _result_microposts = microposts
        .filter(user_id.eq(1))
        .limit(5)
        .load::<Micropost>(&connection)
        .expect("Error loading microposts");

    // println!("Displaying users {:#?}", result_users);

    // println!("Displaying models {:#?}", result_microposts);

    let new_user = create_user(
        &connection,
        "sample_name",
        "railexample@gmail.com",
        "password",
    );
    print_user(new_user);
}

fn print_user(user: User) {
    println!("user_name: {}", user.name.unwrap());
    let user_password: String = String::from(user.password_digest.unwrap());
    println!("password_salt: {}", &user_password[0..30]);
    println!("password_hash: {}", &user_password[30..]);
    println!("password_digest: {}", user_password);
    println!(
        "password to hash: {}",
        hash("password", DEFAULT_COST).unwrap()
    );
    println!("valid: {}", verify("password", &user_password).unwrap());
    println!("------------------------------------------------")
}
