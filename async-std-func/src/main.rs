extern crate async_std;

use async_std::task;

async fn say_hi() {
    println!("Hello world!");
}

fn main() {
    task::block_on(say_hi())
}
