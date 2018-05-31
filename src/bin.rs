extern crate russh;
pub fn main() {
  russh::connect("localhost", 2222);
}