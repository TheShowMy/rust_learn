// Rayon
// 用于Rust的数据并行库
// ·它极其轻量，并且可以轻松地将顺序计算转换为并行计算
// ·同时，它还能保证数据竞争（datarace）的安全性

use rayon::prelude::*;

fn main() {
    let nums: Vec<u64> = (0..1_000_000).collect();
    let sum: u64 = nums.par_iter().sum();
    println!("The sum is: {}", sum);
}
