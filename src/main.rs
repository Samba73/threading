use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
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
  // let (tx, rx) = mpsc::channel();
  // let tx1 = tx.clone();  
  
  // let v = vec![
  //   String::from("First"), 
  //   String::from("Second"),
  //   String::from("Third"),
  //   String::from("Fourth")];
  

  // let handle = thread::spawn(move|| {
  //   println!("First thread started");
  //   println!("The vector is {:?}", v);
  //   let msg1 = String::from("Hello");
  //   tx.send(msg1).unwrap();
  //   // ownership will be transferred and msg1 will be transferred, hence the ownership is transferred to the other thread(receiver)
  //   // println!("The message sent is {}", msg1);

  //   for n in v {
  //     tx.send(n).unwrap();
  //     thread::sleep(Duration::from_secs(1));
  //   }
  // });

  // thread::spawn(move || {
  //   println!("Second thread started");
  //   let msg = vec! [
  //     String::from("another thread started"),
  //     String::from("and transmitting data"),
  //     String::from("!")
  //   ];
  //   for m in msg {
  //     tx1.send(m).unwrap();
  //     thread::sleep(Duration::from_secs(1));
  //   }
  // });

  // // handle.join().unwrap();
  // println!("The message received is {}", rx.recv().unwrap());

  // for r in rx {
  //   println!("Received = {}", r);
  // }

  let counter = Arc::new(Mutex::new(0));

  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut counter = counter.lock().unwrap();
      *counter += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  println!("The count is {}", *counter.lock().unwrap());
}
