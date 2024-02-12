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
  let v = vec![
    String::from("First"), 
    String::from("Second"),
    String::from("Third"),
    String::from("Fourth")];
  

  let handle = thread::spawn(move|| {
    println!("The vector is {:?}", v);
    let msg1 = String::from("Hello");
    tx.send(msg1).unwrap();
    // ownership will be transferred and msg1 will be transferred, hence the ownership is transferred to the other thread(receiver)
    // println!("The message sent is {}", msg1);

    for n in v {
      tx.send(n).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  handle.join().unwrap();
  println!("The message received is {}", rx.recv().unwrap());

  for r in rx {
    println!("Received = {}", r);
  }
}
