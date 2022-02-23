extern crate macro3_derive;
use macro3_derive::GusMacro;

trait GusMacro {
    fn hello_macro();
}

#[derive(GusMacro)]
struct Estructura;

fn main() {
    Estructura::hello_macro();
  
}
