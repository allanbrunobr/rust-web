#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index () -> &'static str{
    "Hello Rocket"
}

#[get("/hi")]
fn hi () -> &'static str{
    "Hi Rocket, updating!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/v1", routes![index, hi])
}
