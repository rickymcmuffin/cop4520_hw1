use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

const LIMIT: u64 = 10_u64.pow(8);

fn main() {
    let now = Instant::now();
    let (sum, num_primes) = find_primes_parallel();
    println!("{:?}, {:?}, {}", sum, num_primes, now.elapsed().as_millis());

    let now = Instant::now();
    let (sum, num_primes) = find_primes_single();
    println!("{:?}, {:?}, {}", sum, num_primes, now.elapsed().as_millis());
}

fn find_primes_single() -> (u64, u64) {
    let mut i: u64 = 2;
    let mut sum: u64 = 0;
    let mut num_primes: u64 = 0;

    while i < LIMIT {
        if is_prime(i) {
            sum += i;
            num_primes += 1;
        }
        if i % 1000000 == 0 {
            print_progress(i);
        }
        i += 1;
    }

    return (sum, num_primes);
}

fn find_primes_parallel() -> (u64, u64) {
    let counter = Arc::new(Mutex::new(2_u64));
    let sum = Arc::new(Mutex::new(0_u64));
    let num_primes = Arc::new(Mutex::new(0_u64));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..8 {
        let counter = counter.clone();
        let sum = sum.clone();
        let num_primes = num_primes.clone();
        let handle = thread::spawn(move || prime_print(counter, sum, num_primes));

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // loop  {
    //     let num = counter.lock().unwrap();
    //     if *num > LIMIT{
    //         break;
    //     }
    // }
    let sum = sum.lock().unwrap();
    let num_primes = num_primes.lock().unwrap();

    return (*sum, *num_primes);
}

fn prime_print(counter: Arc<Mutex<u64>>, sum: Arc<Mutex<u64>>, num_primes: Arc<Mutex<u64>>) {
    let mut i: u64 = 0;

    while i < LIMIT {
        {
            let mut num = counter.lock().unwrap();
            i = *num;
            *num += 1;
        }
        if i % 1000000 == 0 {
            // println!("progress: {i}");
            print_progress(i);
        }
        if i > LIMIT {
            break;
        }

        let prime = is_prime(i);
        if prime {
            let mut pris = num_primes.lock().unwrap();
            *pris += 1;
            let mut sum = sum.lock().unwrap();
            *sum += i;
        }
    }
}

fn is_prime(n: u64) -> bool {
    let mut i = 2;

    let sqr = (n as f64).sqrt() as u64 + 1;

    while i < sqr {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn print_progress(n: u64) {
    let percent = n * 100 / LIMIT;
    print!("\rProgress: {percent}%");
    io::stdout().flush().unwrap();
}
