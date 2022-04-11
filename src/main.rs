use clap::{Arg, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time;

// These are strings because they're command line arguments.
const DEFAULT_MODE: &str = "3";
const DEFAULT_THREAD_COUNT: &str = "1500";
const DEFAULT_SLEEP_INTERVAL: &str = "500";

fn yielder_thread(id: &u32, condition: Arc<AtomicBool>) {
  println!("Yielder thread {} started.", id);
  while condition.load(Ordering::SeqCst) == false {
    thread::yield_now();
  }
}

// This uses the same amount of CPU as with the
// atomic check.
fn yielder_thread_no_atomic(id: &u32) {
  println!("Yielder (no atomic check) thread {} started.", id);
  loop {
    thread::yield_now();
  }
}

fn sleeper_thread(id: &u32, condition: Arc<AtomicBool>, sleep_interval: time::Duration) {
  println!("Sleeper thread {} started.", id);
  while condition.load(Ordering::SeqCst) == false {
    thread::sleep(sleep_interval);
  }
}

fn main() {
  let dumb_condition = Arc::new(AtomicBool::new(false));
  /*let args: Vec<String> = env::args().collect();
  let mode: &str = if args.len() >= 2 { &args[1] } else { "0" };*/
  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  let matches = Command::new("many-threads-experiment")
    .version("0.1.0")
    .author("DkVZ <dk@dkvz.eu>")
    .about("Experiment with many idle threads of different types")
    .arg(Arg::new("mode").index(1).default_value(DEFAULT_MODE).help(
      "Which test to run\n \
          1 - Threads that yield immediately\n \
          2 - Same but with no check whatsoever\n \
          3 - Threads that go to sleep immediately\n",
    ))
    .arg(
      Arg::new("thread_count")
        .short('t')
        .takes_value(true)
        .default_value("12")
        .help("The amount of threads to create"),
    )
    .arg(
      Arg::new("sleep_interval")
        .short('s')
        .long("sleep")
        .takes_value(true)
        .default_value("500")
        .help("Sleep interval in milliseconds, when relevant"),
    )
    .get_matches();

  let mode = matches.value_of("mode").unwrap_or(DEFAULT_MODE);
  let thread_count: u32 = matches
    .value_of("thread_count")
    .unwrap_or(DEFAULT_THREAD_COUNT)
    .parse()
    .unwrap_or(DEFAULT_THREAD_COUNT.parse().unwrap());
  let sleep_interval: u64 = matches
    .value_of("sleep_interval")
    .unwrap_or(DEFAULT_SLEEP_INTERVAL)
    .parse()
    .unwrap_or(DEFAULT_SLEEP_INTERVAL.parse().unwrap());
  let sleep_interval = time::Duration::from_millis(sleep_interval);

  for i in 1..(thread_count + 1) {
    let condition_clone = dumb_condition.clone();
    match mode {
      "1" => handles.push(thread::spawn(move || yielder_thread(&i, condition_clone))),
      "2" => handles.push(thread::spawn(move || yielder_thread_no_atomic(&i))),
      "3" => handles.push(thread::spawn(move || {
        sleeper_thread(&i, condition_clone, sleep_interval)
      })),
      _ => println!("Please provide mode as argument, 1 to something"),
    }
  }

  for h in handles {
    h.join().unwrap();
  }
}
