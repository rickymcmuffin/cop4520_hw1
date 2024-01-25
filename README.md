# COP4520HW1

## Building

Make sure rust is installed. Then enter the folder and run:
```
cargo build
```

## Description

This uses multi threading to find prime numbers for 1 to 10^8. It works by giving the threads a shared, atomic, counter variable and having them each add to it to find the next prime. This ensures each thread has equal work to do.

One issue is the is_prime function is not very efficient. It could have been optimized by only having it test with previous primes but this is a multi-threading assignment so I decided to keep it simple and focus on the parallelism. 
