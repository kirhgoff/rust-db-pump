extern crate rust_db_pump;

use postgres::{Connection, TlsMode};

fn main() {
//    let mut client = Client::connect("host=localhost port=5433 user=postgres password=postgres", NoTls).unwrap();
//
//    let mut reader = client
//        .copy_out("COPY source_table TO STDOUT", &[])
//        .unwrap();
//
//    let mut s = String::new();
//    reader.read_to_string(&mut s).unwrap();

    let database_url = "postgres://postgres:postgres@localhost:5433/bams1";
    let connection = Connection::connect(database_url, TlsMode::None).unwrap();
    let rows = connection.query("COPY source_table TO STDOUT", &[]).unwrap();
    println!("{:?}", &rows);
}