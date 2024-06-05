fn fibonacci_and_primes(range: (u32, u32)) -> (Vec<u32>, u32) {
    // Calculate the Fibonacci sequence up to 10 numbers using iteration
    let mut fib_seq = vec![0, 1];
    for i in 2..10 {
        fib_seq.push(fib_seq[i - 1] + fib_seq[i - 2]);
    }

    // Count the number of primes within the given range using a modified Sieve of Eratosthenes
    let (start, end) = range;
    let mut prime_count = 0;

    let mut is_prime = vec![true; (end + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(end as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=end as usize).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    prime_count = is_prime[start as usize..=end as usize].iter().filter(|&&x| x).count() as u32;

    (fib_seq, prime_count)
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 || num == 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

let range = (1, 20);
let (fib_seq, prime_count) = fibonacci_and_primes(range);
println!("Fibonacci sequence: {:?}", fib_seq);
println!("Number of primes in the range {:?}: {}", range, prime_count);