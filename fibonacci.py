
def is_prime(num):
    """Check if a number is a prime number."""
    if num <= 1:
        return False
    if num == 2:
        return True
    if num % 2 == 0:
        return False
    for i in range(3, int(num**0.5) + 1, 2):
        if num % i == 0:
            return False
    return True

def fibonacci_sequence(n):
    """Generate Fibonacci sequence up to n terms."""
    sequence = []
    a, b = 0, 1
    while len(sequence) < n:
        sequence.append(a)
        a, b = b, a + b
    return sequence

def count_primes_in_sequence(sequence):
    """Count prime numbers in a given sequence."""
    count = 0
    for num in sequence:
        if is_prime(num):
            count += 1
    return count

def main():
    """Main function to execute the program."""
    try:
        n = int(input("Enter the number of Fibonacci terms: "))
        if n <= 0:
            print("Please enter a positive integer.")
            return
        
        fib_sequence = fibonacci_sequence(n)
        prime_count = count_primes_in_sequence(fib_sequence)
        
        print(f"Fibonacci sequence up to {n} terms: {fib_sequence}")
        print(f"Number of prime numbers in the sequence: {prime_count}")
        
    except ValueError:
        print("Invalid input. Please enter an integer.")

if __name__ == "__main__":
    main()


def calculate_primes_in_fibonacci(n):
    """Calculate prime numbers in the Fibonacci sequence up to n terms."""
    fib_sequence = fibonacci_sequence(n)
    primes = [num for num in fib_sequence if is_prime(num)]
    return primes
