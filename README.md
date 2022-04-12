# Experiments with threads and performances

## Experiment ideas
- A lot of sleeping threads that are set with a long duration;
- Sleeping threads with lower duration, that immediately go back to sleep;
- Threads that call "yield_now" immediately if a condition isn't met -> They didn't lie in the doc this uses a lot of CPU which makes sense since there's no blocking at all;
- Threads waiting for a [Condvar](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html) - This should have the least impact.

### Sleeper threads
- These generate a context switch every time they're wait period expires, or somewhere close to that which, I believe, is what's generating the load.
- From Process Explorer we can see these are in state "Wait:DelayExecution" which is **not** the usual waiting thread state.
- The state we want is called "Wait:UserRequest".

-> Could probably achieve this using Condvar as mentioned above, a blocking message channel or thread parking.

## Testing "protocol"
- Disable antimalware
- Stop unrequired services if I have a script for that ready on target machine
- Disable bluetooth
- Pause Windows Updates