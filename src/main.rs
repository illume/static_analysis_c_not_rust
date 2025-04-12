extern crate sha1;
use sha1::{Digest, Sha1};
use std::sync::{Arc, Mutex};
use std::thread;

// security issue, using old deprecated crypto
fn deprecated_crypto_function(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// Performance. Why is the function being called twice?
fn cpu_intensive_function(data: &str) -> usize {
    // Sleep to simulate a CPU-intensive operation
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Simulate a CPU-intensive operation
    data.chars().filter(|&c| c.is_alphanumeric()).count()
}

fn check_directory(directory: &str) -> bool {
    if cpu_intensive_function(directory) > 0
        && directory.chars().nth(cpu_intensive_function(directory) - 1) != Some('\\')
    {
        return true;
    }
    false
}

// Using float in loops is not portable.
// Different amount of loops because different float
// hardware or optimizations.
fn count_with_float() {
    let mut counter = 0.0;

    while counter < 10.0 {
        println!("Counter: {}", counter);
        counter += 0.5;
    }
}

// Incrementing wrong value. Much more unlikely in rust, because mut and iterators.
fn increment_wrong_value(array: &mut [f32], mut n: usize) {
    let i = 0;
    while i != n {
        array[i] *= 2.0;
        n += 1;
    }
}

// non-basic infinite loop, meaning this code would not halt
fn infinite_loop() {
    let mut i = 0;
    let mut j = 0;
    while i < 10 {
        if j == 5 {
            break;
        }
        // Missing increment of 'i' under certain conditions
        if j % 2 == 0 {
            i += 1;
        }
        println!("i is {}, j is {}", i, j);
        j += 1;
    }
}

// non-basic concurrency issue, this will deadlock when run
// The deadlock happens because:
//   Thread 1 holds lock on data1 and waits for lock on data2.
//   Thread 2 holds lock on data2 and waits for lock on data1.
fn deadlock() {
    let data1 = Arc::new(Mutex::new(0));
    let data2 = Arc::new(Mutex::new(0));

    let data1_clone1 = Arc::clone(&data1);
    let data2_clone1 = Arc::clone(&data2);

    let handle1 = thread::spawn(move || {
        let mut num1 = data1_clone1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(50)); // Simulate some work
        let mut num2 = data2_clone1.lock().unwrap();
        *num1 += 1;
        *num2 += 1;
    });

    let data1_clone2 = Arc::clone(&data1);
    let data2_clone2 = Arc::clone(&data2);

    let handle2 = thread::spawn(move || {
        let mut num2 = data2_clone2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(50)); // Simulate some work
        let mut num1 = data1_clone2.lock().unwrap();
        *num2 += 1;
        *num1 += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!(
        "data1: {}, data2: {}",
        *data1.lock().unwrap(),
        *data2.lock().unwrap()
    );
}

fn main() {
    count_with_float();

    let data = b"example data";
    let hash = deprecated_crypto_function(data);
    println!("SHA-1 hash: {:?}", hash);

    let directory = "example\\path";
    if check_directory(directory) {
        println!("The directory does not end with a backslash.");
    } else {
        println!("The directory ends with a backslash.");
    }
    // let i = 7;
    // if i > 99 { // These three run forever...
    let mut array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let n = array.len();
    increment_wrong_value(&mut array, n);
    println!("{:?}", array);

    deadlock();
    infinite_loop();
    // }
}
