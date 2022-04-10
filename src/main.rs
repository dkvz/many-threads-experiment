use std::env;
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
  let thread_count = 24;
  let dumb_condition = Arc::new(AtomicBool::new(false));
  let args: Vec<String> = env::args().collect();
  let mode: &str = if args.len() >= 2 { &args[1] } else { "0" };
  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  for i in 1..thread_count {
    let condition_clone = dumb_condition.clone();
    match mode {
      "1" => handles.push(thread::spawn(move || yielder_thread(&i, condition_clone))),
      _ => println!("Please provide mode as argument, 1 to something"),
    }
  }

  for h in handles {
    h.join().unwrap();
  }
}
