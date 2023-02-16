#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/loaderio-8f747818022b458427567324f00e2493")]
fn verify() -> &'static str {
    "loaderio-8f747818022b458427567324f00e2493"
}

fn main() {
    rocket::ignite()
    .mount("/", routes![hello, verify])
    .launch();
}
