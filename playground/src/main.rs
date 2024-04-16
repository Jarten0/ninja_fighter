#![allow(unused)]

trait Trait {
    fn foo(&self) {
        println!("Foo'd")
    }
}

struct Bar;

impl Trait for Bar {
    fn foo(&self) {
        println!("Original Implementation")
    }
}

fn wrap_original_implementation(bar: &Box<impl Trait + ?Sized>) {
    bar.foo()
}

impl Trait for Box<dyn Trait> {
    fn foo(&self) {
        println!("Custom implementation");
        wrap_original_implementation(self);
    }
}

fn main() {
    let bar = Bar;

    test(Box::new(bar))
}

fn test(trait_object: Box<dyn Trait>) {
    test2(trait_object)
}

fn test2(trait_object: impl Trait) {
    trait_object.foo()
}
