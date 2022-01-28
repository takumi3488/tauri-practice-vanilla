#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::convert::TryInto;

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn perfect(max: usize) -> Vec<usize> {
  let mut res: Vec<usize> = vec![];
  let primes = prime(2_usize.pow(max as u32) - 1);
  for i in 1..(max + 1) {
    let a = 2_usize.pow(i as u32) - 1;
    if primes.clone().into_iter().any(|p|p==a) {
      res.push(2_usize.pow((i - 1_usize).try_into().unwrap()) * a);
    }
  }
  return res;
}

fn prime(max: usize) -> Vec<usize> {
  let mut res: Vec<usize> = vec![2];
  for i in 2..(max + 1) {
    if res.clone().into_iter().all(|x| i % x != 0) {
      res.push(i)
    }
  }
  return res;
}

#[cfg(test)]
#[test]
fn prime_test() {
  let expected: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
  assert_eq!(prime(50), expected);
}

#[test]
fn perfect_test() {
  let expected: Vec<usize> = vec![6, 28, 496, 8128, 33550336];
  assert_eq!(perfect(13), expected);
}
