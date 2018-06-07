extern crate russh;
extern crate pretty_env_logger;

pub fn main() {
  pretty_env_logger::init();
  russh::connect("localhost", 2222).unwrap();
}