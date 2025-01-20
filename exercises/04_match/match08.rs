#[allow(dead_code)]
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // TODO: Remove the codes below, using `match` instead 
    if let Foo::Bar = a_to_remove {
        println!("match foo::bar")
    } else if let Foo::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
}
