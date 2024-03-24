#[macro_use]
extern crate diesel;

mod application;
mod domain;
mod infrastructure;
mod schema;
mod utils;
mod handler;

fn main () {
    let args = handler::args();
    let _h = handler::handler(args);
}
