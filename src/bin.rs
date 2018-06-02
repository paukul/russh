extern crate russh;
#[macro_use] extern crate env_logger;

pub fn main() {
  env_logger::init();
  russh::connect("localhost", 2222).unwrap();
}