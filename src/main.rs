extern crate app;
extern crate diesel;

use self::app::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use app::schema::microposts::dsl::*;
    use app::schema::users::dsl::*;

    let connection = establish_connection();

    let result_users = users
        .filter(activated.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    let result_microposts = microposts
        .filter(user_id.eq(1))
        .limit(5)
        .load::<Micropost>(&connection)
        .expect("Error loading microposts");

    println!("Displaying users {:#?}", result_users);
    println!("Displaying models {:#?}", result_microposts);
}
