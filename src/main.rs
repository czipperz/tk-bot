#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::Data;

#[post("/", data = "<request>")]
fn respond(request: Data) -> String {
    use std::io::Read;
    let mut text = String::new();
    request.open().read_to_string(&mut text).unwrap();
    text
}

fn main() {
    rocket::ignite().mount("/", routes![respond]).launch();
}
