fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
fn main() -> i32 {
  return add(add(add(1,1),1), add(1,add(1,1)));
}
