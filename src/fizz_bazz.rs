pub fn run() {
  for i in 1..100 {
    println!("{} {}", i, print_fizz_bazz(i));
  }
}

fn print_fizz_bazz(i: u32) -> &'static str {
  if i % 15 == 0 {
    return "fizzbuzz";
  } else if i % 5 == 0 {
    return "buzz";
  } else if i % 3 == 0 {
    return "fizz";
  } else {
    return "";
  }
}

#[test]
fn fizz_bazz_test() {
  assert_eq!(print_fizz_bazz(1), "");
  assert_eq!(print_fizz_bazz(3), "fizz");
  assert_eq!(print_fizz_bazz(4), "");
  assert_eq!(print_fizz_bazz(5), "buzz");
  assert_eq!(print_fizz_bazz(15), "fizzbuzz");
}
