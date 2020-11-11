
#[macro_use]
extern crate procedural_macro_derive;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
#[HelloWorldName = "the best FrenchToast"]
struct FrenchToast;

#[derive(HelloWorld)]
#[HelloWorldName = "the best Waffles"]
struct Waffles;

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}
