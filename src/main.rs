use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time;

// Basically the test parameters are here:
const THREAD_COUNT: i32 = 5000;
//const SLEEP_INTERVAL: time::Duration = time::Duration::from_secs(1);
const SLEEP_INTERVAL: time::Duration = time::Duration::from_millis(500);

fn yielder_thread(id: &i32, condition: Arc<AtomicBool>) {
  println!("Yielder thread {} started.", id);
  while condition.load(Ordering::SeqCst) == false {
    thread::yield_now();
  }
}

// This uses the same amount of CPU as with the
// atomic check.
fn yielder_thread_no_atomic(id: &i32) {
  println!("Yielder (no atomic check) thread {} started.", id);
  loop {
    thread::yield_now();
  }
}

fn sleeper_thread(id: &i32, condition: Arc<AtomicBool>) {
  println!("Sleeper thread {} started.", id);
  while condition.load(Ordering::SeqCst) == false {
    thread::sleep(SLEEP_INTERVAL);
  }
}

fn main() {
  let dumb_condition = Arc::new(AtomicBool::new(false));
  let args: Vec<String> = env::args().collect();
  let mode: &str = if args.len() >= 2 { &args[1] } else { "0" };
  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  for i in 1..THREAD_COUNT {
    let condition_clone = dumb_condition.clone();
    match mode {
      "1" => handles.push(thread::spawn(move || yielder_thread(&i, condition_clone))),
      "2" => handles.push(thread::spawn(move || yielder_thread_no_atomic(&i))),
      "3" => handles.push(thread::spawn(move || sleeper_thread(&i, condition_clone))),
      _ => println!("Please provide mode as argument, 1 to something"),
    }
  }

  for h in handles {
    h.join().unwrap();
  }
}
