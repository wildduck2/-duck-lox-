fn main() {
  let mut total = 0;
  for index in 0..10 {
    total += index;
  }

  if total > 20 {
    println!("high: {}", total);
  } else {
    println!("low");
  }
}
