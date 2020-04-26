// use std::thread;
// // use futures::executor::block_on;
// use std::time::Duration;

// fn main(){
// hello();
// world();
// }
// fn world() {
//     thread::sleep(Duration::from_secs(7));;
//     print!("world");
// }
// fn hello(){
//     thread::sleep(Duration::from_secs(2));
//     print!("Hello");
// }

use std::thread;
use futures::executor::block_on;
use std::time::Duration;

