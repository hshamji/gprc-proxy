#[derive(Debug)]
struct Test {
    this: String,
}

impl Test {
    fn new() -> Self {
        Test {
            this: String::from("this is new")
        }
    }
}
fn main() {
    println!("Hello, world!");
    let a = Test::new();
    println!("Output is {:?}", a);
}
