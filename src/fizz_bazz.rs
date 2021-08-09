pub fn run() {
  for i in 1..100 {
    println!("{} {}", i, return_fizz_bazz(i));
  }
}

fn return_fizz_bazz(i: u32) -> &'static str {
  return if i % 15 == 0 {
    "fizzbuzz"
  } else if i % 5 == 0 {
    "buzz"
  } else if i % 3 == 0 {
    "fizz"
  } else {
    ""
  };
}

#[test]
fn fizz_bazz_test() {
  assert_eq!(return_fizz_bazz(1), "");
  assert_eq!(return_fizz_bazz(3), "fizz");
  assert_eq!(return_fizz_bazz(4), "");
  assert_eq!(return_fizz_bazz(5), "buzz");
  assert_eq!(return_fizz_bazz(15), "fizzbuzz");
}
