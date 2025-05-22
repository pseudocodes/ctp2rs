mod apimd;
use std::thread;

use apimd::*;

mod apitd;
use apitd::*;

fn run_two_loops() {
    let handle1 = thread::spawn(run_td); // 启动第 td 线程
    let handle2 = thread::spawn(run_md); // 启动第 md 线程

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Both loops are finished. Main thread exiting.");
}
fn main() {
    run_two_loops();
}
