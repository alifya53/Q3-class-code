// use std::thread;
// // use futures::executor::block_on;
// use std::time::Duration;

// fn main(){
// hello();
// world();
// }
// fn world() {
//     thread::sleep(Duration::from_secs(2));
//     println!("world");
// }
// fn hello(){
//     thread::sleep(Duration::from_secs(2));
//     println!("Hello");
// }

// use std::thread;
// use futures::executor::block_on;
// use std::time::Duration;

// // `block_on` blocks the current thread until the provided future has run to
// // completion. Other executors provide more complex behavior, like scheduling
// // multiple futures onto the same thread.
// use futures::executor::block_on;

// async fn hello_world() {
//     println!("hello, world!");
// }

// fn main() {
//     let future = hello_world(); // Nothing is printed
//     block_on(future); // `future` is run and "hello, world!" is printed
// }



// use futures::executor::block_on;
// // use:std::thread;
// // use:std::time:Duration;
// async fn learn_song() -> Song {println!("Song learning!")}
// async fn sing_song(song: Song) {println!("Song learning!")}
// async fn dance() { println!("Dancing Time!") }

// async fn learn_and_sing() {
//     // Wait until the song has been learned before singing it.
//     // We use `.await` here rather than `block_on` to prevent blocking the
//     // thread, which makes it possible to `dance` at the same time.
//     let song = learn_song().await;
//     sing_song(song).await;
// }

// async fn async_main() {
//     let f1 = learn_and_sing();
//     let f2 = dance();

//     // `join!` is like `.await` but can wait for multiple futures concurrently.
//     // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
//     // future will take over the current thread. If `dance` becomes blocked,
//     // `learn_and_sing` can take back over. If both futures are blocked, then
//     // `async_main` is blocked and will yield to the executor.
//     futures::join!(f1, f2);
// }

// fn main() {
//     block_on(async_main());
// }


use std::thread;
// use futures::executor::block_on;
use std::time::Duration;
fn dance_1(){
    thread::sleep(Duration::from_secs(3));
    println!("I am Dancing on the song");  
}
fn main(){
fn dance(){
    thread::sleep(Duration::from_secs(5));
    println!("I am Dancing on the song on dance function"); 
}    
learn_song();
sing_song();
dance(); 
dance_1();

}
fn sing_song() {
    thread::sleep(Duration::from_secs(2));
    println!("I am singing the song");
}
fn learn_song(){
    thread::sleep(Duration::from_secs(2));
    println!("I am learning the song");
}
