// TODO: make the necessary variable mutable
// Mutability can be changed when ownership is transferred.
fn main() {
    let s = String::from("Hello ");

    let s1 = s;

    s1.push_str("World!");

    println!("Success!");
}
