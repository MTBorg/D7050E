fn main() -> i32 {
  let a = 0;
  let c = 0;
  while a < 3 {
    if a == 1 {
      let c = b;
    }
    let b = 4;
    a = a + 1;
  }
  return c;
}
