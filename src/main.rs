#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hi!"
    }
}

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

use other::world;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![hello, other::world])
        .launch();
}
