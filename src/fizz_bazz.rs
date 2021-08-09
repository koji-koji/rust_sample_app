pub fn run() {
  for i in 1..100 {
    if i % 15 == 0 {
      println!("{} fizzbuzz", i)
    }
    if i % 5 == 0 {
      println!("{} buzz", i)
    }
    if i % 3 == 0 {
      println!("{} fizz", i)
    }
  }
}

#[test]
fn fizz_bazz_test() {
  assert_eq!(run(2), 4);
}
