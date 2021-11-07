use std::{
    convert::TryInto,
    panic,
    process::{Child, Command},
    thread,
    time::Duration,
};

use assert_cmd::cargo::cargo_bin;
use nix::{
    sys::signal::{kill, Signal::SIGTERM},
    unistd::Pid,
};

const PORT: u64 = 8000;

// we set no difficulty and waiting time to minimize testing duration
const DIFFICULTY: u64 = 0;
const TRANSACTION_WAITING_MS: u64 = 100;

pub fn get_server_url() -> String {
    format!("http://localhost:{}", PORT)
}

pub fn run_in_server_instance(f: fn() -> ()) {
    // start the blockchain in the background
    let mut cmd = Command::new(cargo_bin("rust_blockchain"))
        .env("PORT", PORT.to_string())
        .env("DIFFICULTY", DIFFICULTY.to_string())
        .env("TRANSACTION_WAITING_MS", TRANSACTION_WAITING_MS.to_string())
        .spawn()
        .unwrap();

    // allow time for the blockchain to start
    sleep_secs(1);

    // run the desired functionality while the server is running
    // even if the tests panics (due to an assert failing) we shutdown the server
    let result = panic::catch_unwind(|| f());

    // finish the blockchain instance so it does not become a zombie process
    let pid = Pid::from_raw(cmd.id().try_into().unwrap());
    kill(pid, SIGTERM).unwrap();

    // block the thread until the server has finished
    wait_for_termination(&mut cmd);

    // after we shutdown the server, we can assert the the test is ok
    assert!(result.is_ok());
}

fn wait_for_termination(child: &mut Child) {
    let max_waiting_in_secs = 5;

    // check every second if the child has finished
    for _ in 0..max_waiting_in_secs {
        match child.try_wait().unwrap() {
            // has finished, so we exit
            Some(_) => return,
            // hasn't finished, we wait another second
            None => sleep_secs(1),
        }
    }

    // at this point, we waited but the child didn't finish
    // so we forcefully kill it
    kill_process(child);
}

pub fn wait_for_mining() {
    let milis = TRANSACTION_WAITING_MS * 10;
    let wait_duration = Duration::from_millis(milis);
    thread::sleep(wait_duration);
}

fn sleep_secs(secs: u64) {
    let wait_duration = Duration::from_secs(secs);
    thread::sleep(wait_duration);
}

fn kill_process(child: &mut Child) {
    let _ = child.kill();
    child.wait().unwrap();
}
