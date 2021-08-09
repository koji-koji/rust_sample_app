mod constants;
mod fizz_bazz;

fn main() {
    println!("Hello, world!");
    println!("{}", internal_adder(2, constants::SampleConst));
    fizz_bazz::run();
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn my_test() {
    assert_eq!(internal_adder(2, 2), 4);
}
