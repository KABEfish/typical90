// -*- coding:utf-8-unix -*-

use proconio::input;

// typical90 - Large LCM
// https://atcoder.jp/contests/typical90/tasks/typical90_al

fn main() {
  input! {
    a: i64,
    b: i64
  }

  let max = 10_i64.pow(18);
  let c = a / gcd(a, b);

  if b <= (max / c) {
    print!("{}", c * b);
  } else {
    print!("Large");
  }
}

fn gcd(a: i64, b: i64) -> i64 {
  if b == 0 {
    return a;
  } else {
    return gcd(b, a % b);
  }
}
