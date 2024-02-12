use std::thread;
use std::time::Duration;
use std::sync::mpsc;
fn main() {
  // let handle = thread::spawn(|| {
  //   for i in 1 .. 50 {
  //       println!("The number {} is from spawned thread", i,);
  //       thread::sleep(Duration::from_millis(1));   
  //   }
  // });
  // handle.join().unwrap();
  // for i in 1..5 {
  //   println!("The number {} is from main thread", i);
  //   thread::sleep(Duration::from_millis(1));
  // }
  let (tx, rx) = mpsc::channel();
  let v = vec![1, 2, 3, 4];
  

  let handle = thread::spawn(move|| {
    println!("The vector is {:?}", v);
    let msg1 = String::from("Hello");
    tx.send(msg1).unwrap();
  });

  handle.join().unwrap();
  println!("The message is {}", rx.recv().unwrap());
}
