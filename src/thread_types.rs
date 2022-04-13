use crate::MutexCondvar;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time;

pub fn yielder_thread(id: &u32, condition: Arc<AtomicBool>) {
  println!("Yielder thread {} started.", id);
  while condition.load(Ordering::SeqCst) == false {
    thread::yield_now();
  }
}

// This uses the same amount of CPU as with the
// atomic check.
pub fn yielder_thread_no_atomic(id: &u32) {
  println!("Yielder (no atomic check) thread {} started.", id);
  loop {
    thread::yield_now();
  }
}

pub fn sleeper_thread(id: &u32, condition: Arc<AtomicBool>, sleep_interval: time::Duration) {
  println!(
    "Sleeper thread {} started - Sleep interval {}.",
    id,
    sleep_interval.as_millis()
  );
  while condition.load(Ordering::SeqCst) == false {
    thread::sleep(sleep_interval);
  }
}

pub fn parked_thread(id: &u32) {
  println!("Parked thread {} started.", id);
  thread::park();
}

pub fn condvar_thread(id: &u32, mutex_condvar: Arc<MutexCondvar>) {
  println!("Condvar thread {} started.", id);
  let (lock, cvar) = &*mutex_condvar;
  let started = lock.lock().unwrap();
  let _ = cvar.wait(started).unwrap();
  println!("Condvar thread {} ended.", id);
}
