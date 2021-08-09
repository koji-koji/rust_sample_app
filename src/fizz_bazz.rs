pub fn run() {
  for i in 1..100 {
    println!("{} {}", i, return_fizz_bazz(i));
  }
  for i in 1..100 {
    println!("{} {}", i, return_fizz_bazz2(i));
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

fn return_fizz_bazz2(i: u32) -> std::string::String {
  let fizz_result = if (i % 3) == 0 { "fizz" } else { "" };
  let buzz_result = if (i % 5) == 0 { "buzz" } else { "" };
  let answer = format!("{}{}", fizz_result, buzz_result);
  answer
}

#[test]
fn fizz_bazz_test() {
  assert_eq!(return_fizz_bazz(1), "");
  assert_eq!(return_fizz_bazz(3), "fizz");
  assert_eq!(return_fizz_bazz(4), "");
  assert_eq!(return_fizz_bazz(5), "buzz");
  assert_eq!(return_fizz_bazz(15), "fizzbuzz");
}
#[test]
fn fizz_bazz_test2() {
  assert_eq!(return_fizz_bazz2(1), "");
  assert_eq!(return_fizz_bazz2(3), "fizz");
  assert_eq!(return_fizz_bazz2(4), "");
  assert_eq!(return_fizz_bazz2(5), "buzz");
  assert_eq!(return_fizz_bazz2(15), "fizzbuzz");
}
