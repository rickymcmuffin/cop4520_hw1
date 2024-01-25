use std::collections::VecDeque;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

const LIMIT: u64 = 10_u64.pow(7);
const RUN_SINGLE: bool = true;

fn main() {
    let now = Instant::now();
    let (sum, num_primes, mut top_primes) = find_primes_parallel();
    let elapsed = now.elapsed().as_millis();
    top_primes.make_contiguous().sort();
    while top_primes.len() > 10 {
        top_primes.pop_front();
    }
    println!("{}, {}, {}", elapsed, num_primes, sum);
    println!("{:?}", top_primes);

    // runs same algorithm on one thread
    if RUN_SINGLE {
        let now = Instant::now();
        let (sum2, num_primes2, top_primes2) = find_primes_single();
        println!("{}, {}, {}", sum2, num_primes2, now.elapsed().as_millis());
        println!("{:?}", top_primes2);
    }
}

// finds primes using a single core approach
fn find_primes_single() -> (u64, u64, VecDeque<u64>) {
    let mut i: u64 = 2;
    let mut sum: u64 = 0;
    let mut num_primes: u64 = 0;
    let mut top_primes: VecDeque<u64> = VecDeque::new();

    while i < LIMIT {
        if is_prime(i) {
            sum += i;
            num_primes += 1;
            top_primes.push_back(i);
            if top_primes.len() > 10 {
                top_primes.pop_front();
            }
        }
        if i % 1000000 == 0 {
            print_progress(i);
        }
        i += 1;
    }

    print!("\r");
    return (sum, num_primes, top_primes);
}

// finds all prime numbers with 8 threads. returns sum, number of primes, and top 10 primes
fn find_primes_parallel() -> (u64, u64, VecDeque<u64>) {
    // add all threads
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // all atomic shared memory
    let counter = Arc::new(Mutex::new(2_u64));
    let sum = Arc::new(Mutex::new(0_u64));
    let num_primes = Arc::new(Mutex::new(0_u64));
    let primes_vec: VecDeque<u64> = VecDeque::new();
    let top_primes = Arc::new(Mutex::new(primes_vec));

    // add all threads to handles
    for _ in 0..8 {
        let counter = counter.clone();
        let sum = sum.clone();
        let num_primes = num_primes.clone();
        let top_primes = top_primes.clone();
        let handle = thread::spawn(move || prime_thread(counter, sum, num_primes, top_primes));

        handles.push(handle);
    }

    // join all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // get return variables
    let sum = sum.lock().unwrap();
    let num_primes = num_primes.lock().unwrap();
    let top_primes = top_primes.lock().unwrap().clone();

    print!("\r");
    return (*sum, *num_primes, top_primes);
}

// takes in all shared memory
fn prime_thread(
    counter: Arc<Mutex<u64>>,
    sum: Arc<Mutex<u64>>,
    num_primes: Arc<Mutex<u64>>,
    top_primes: Arc<Mutex<VecDeque<u64>>>,
) {
    let mut i: u64 = 0;

    while i < LIMIT {
        // lock counter
        let mut num = counter.lock().unwrap();
        i = *num;
        *num += 1;
        std::mem::drop(num); // unlock num

        // for printing progress
        if i % 1000000 == 0 {
            // println!("progress: {i}");
            print_progress(i);
        }

        // don't check any numbers above LIMIT
        if i > LIMIT {
            break;
        }

        // check if it is prime
        let prime = is_prime(i);
        if prime {
            // lock and add to number of primes
            let mut pris = num_primes.lock().unwrap();
            *pris += 1;
            std::mem::drop(pris);

            // lock and add to sum
            let mut sum = sum.lock().unwrap();
            *sum += i;
            std::mem::drop(sum);

            // lock push to top primes
            let mut tp = top_primes.lock().unwrap();
            tp.push_back(i);
            // if tp.len() > 10 {
            //     tp.pop_front();
            // }
            std::mem::drop(tp);
        }
    }
}

// returns true if n is prime. false otherwise
fn is_prime(n: u64) -> bool {
    // start on 2 as 1 is not prime
    let mut i = 2;

    // square root is largest number that won't already be checked
    let sqr = (n as f64).sqrt() as u64 + 1;

    while i < sqr {
        // if divisble, it is not prime
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    // not divisble by any numbers so it is prime
    return true;
}

// helper function to print the current progress
fn print_progress(n: u64) {
    let percent = n * 100 / LIMIT;
    print!("\rProgress: {percent}%");
    io::stdout().flush().unwrap();
}
