// Ok: Borrow a mutable object as immutable
// TODO: This code has no errors!
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

#[allow(unused_variables)]
#[allow(clippy::ptr_arg)]
fn borrow_object(s: &String) {}
