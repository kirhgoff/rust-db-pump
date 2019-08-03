extern crate rust_db_pump;
extern crate dotenv;

use dotenv::dotenv;

use rust_db_pump::*;

fn main() {
    dotenv().ok();

    let create_table_template = include_str!("./create_table_template.sql");
    let delete_table_template = include_str!("./delete_table_template.sql");

    reset_table(SOURCE_DATABASE_URL, "source_table", create_table_template, delete_table_template);
    reset_table(TARGET_DATABASE_URL, "target_table", create_table_template, delete_table_template);
}

fn reset_table(database_url: &str, table_name: &str, create_template: &str, delete_template: &str) {
    let source_connection = create_connection(database_url);
    let delete_query = str::replace(delete_template, "$table_name", table_name);
    let _ = source_connection.execute(delete_query.as_str(), &[]); // table could be missing

    let create_query = str::replace(create_template, "$table_name", table_name);
    source_connection.execute(create_query.as_str(), &[]).unwrap();
}
