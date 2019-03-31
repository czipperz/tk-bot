#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn respond() {
}

fn main() {
    rocket::ignite().mount("/", routes![respond]).launch();
}
