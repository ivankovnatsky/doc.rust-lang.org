// Note that when the main thread of a Rust program completes, all spawned threads are shut down,
// whether or not they have finished running. The output from this program might be a little
// different every time, but it will look similar to the following:

// The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a
// different thread to run. The threads will probably take turns, but that isn’t guaranteed: it
// depends on how your operating system schedules the threads. In this run, the main thread printed
// first, even though the print statement from the spawned thread appears first in the code. And
// even though we told the spawned thread to print until i is 9, it only got to 5 before the main
// thread shut down.

// If you run this code and only see output from the main thread, or don’t see any overlap, try
// increasing the numbers in the ranges to create more opportunities for the operating system to
// switch between the threads.

// The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the
// main thread ending, but because there is no guarantee on the order in which threads run, we also
// can’t guarantee that the spawned thread will get to run at all!

// We can fix the problem of the spawned thread not running or ending prematurely by saving the return
// value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle
// is an owned value that, when we call the join method on it, will wait for its thread to finish.
// Listing 16-2 shows how to use the JoinHandle of the thread we created in Listing 16-1 and call join
// to make sure the spawned thread finishes before main exits:

// Calling join on the handle blocks the thread currently running until the thread represented by
// the handle terminates. Blocking a thread means that thread is prevented from performing work or
// exiting. Because we’ve put the call to join after the main thread’s for loop, running Listing
// 16-2 should produce output similar to this:

// The two threads continue alternating, but the main thread waits because of the call to
// handle.join() and does not end until the spawned thread is finished.

// But let’s see what happens when we instead move handle.join() before the for loop in main, like
// this:

// The main thread will wait for the spawned thread to finish and then run its for loop, so the
// output won’t be interleaved anymore, as shown here:

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
