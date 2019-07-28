extern crate diesel;
extern crate rust_db_pump;

use rust_db_pump::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::input_table::dsl::*;

    let connection = establish_connection();

    let results = input_table
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}