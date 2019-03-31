#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[post("/", data = "<request>")]
fn respond(request: String) -> String {
    request
}

fn main() {
    rocket::ignite().mount("/", routes![respond]).launch();
}
