use proconio::input;

fn main() {
  input!{
    a: i64,
    b: i64,
    c: i64
  }
  let m = gcd(a, gcd(b, c));
  
  let answer = a / m + b / m + c / m - 3;
  println!("{}", answer);
}

fn gcd(a: i64, b: i64) -> i64 {
  if b == 0 {
    return a;
  } else {
    return gcd(b, a % b);
  }
}