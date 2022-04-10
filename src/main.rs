use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

fn yielder_thread(id: &i32, condition: Arc<AtomicBool>) {
  println!("Thread {} started.", id);
  while condition.load(Ordering::SeqCst) == false {
    thread::yield_now();
  }
}

fn main() {
  let dumb_condition = Arc::new(AtomicBool::new(false));

  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  for i in 1..10 {
    let condition_clone = dumb_condition.clone();
    handles.push(thread::spawn(move || yielder_thread(&i, condition_clone)));
  }

  println!("Test started.");

  for h in handles {
    h.join().unwrap();
  }
}
