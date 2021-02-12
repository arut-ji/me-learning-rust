#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Form;
use rocket::*;

#[derive(FromForm)]
struct User {
    name: String,
    account: usize,
}

#[get("/item?<id>&<user..>")]
fn item(id: usize, user: Option<Form<User>>) {}

fn main() {
    rocket::ignite().mount("/", routes![item]).launch();
}
