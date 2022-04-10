# Experiments with threads and performances

## Experiment ideas
- A lot of sleeping threads that are set with a long duration;
- Sleeping threads with lower duration, that immediately go back to sleep;
- Threads that call "yield_now" immediately if a condition isn't met -> They didn't lie in the doc this uses a lot of CPU;
- Threads waiting for a [Condvar](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html) - This should have the least impact.