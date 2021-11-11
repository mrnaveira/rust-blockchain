use anyhow::Result;
use crossbeam_utils::thread;
use std::time;

pub trait Runnable: Sync {
    fn run(&self) -> Result<()>;
}

pub fn run_in_parallel(runnables: Vec<&dyn Runnable>) {
    thread::scope(|s| {
        for runnable in runnables {
            s.spawn(move |_| {
                runnable.run().unwrap();
            });
        }
    })
    .unwrap();
}

// Suspend the execution of the thread by a particular amount of milliseconds
pub fn sleep_millis(millis: u64) {
    let wait_duration = time::Duration::from_millis(millis);
    std::thread::sleep(wait_duration);
}
