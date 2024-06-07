def fibonacci_sequence(n):
    """Generate the Fibonacci sequence up to the nth number."""
    sequence = []
    a, b = 0, 1
    while a <= n:
        sequence.append(a)
        a, b = b, a + b
    return sequence


def count_primes_in_sequence(sequence):
    """Count the number of prime numbers in the given sequence."""

    def is_prime(num):
        if num < 2:
            return False
        for i in range(2, int(num**0.5) + 1):
            if num % i == 0:
                return False
        return True

    prime_count = sum(1 for num in sequence if is_prime(num))
    return prime_count


def main():
    """Main function to handle user input and print the Fibonacci sequence."""
    try:
        user_input = int(
            input("Enter a number to generate the Fibonacci sequence up to: ")
        )
        if user_input < 0:
            raise ValueError("The number must be non-negative.")
        sequence = fibonacci_sequence(user_input)
        print(f"Fibonacci sequence up to {user_input}: {sequence}")
    except ValueError as e:
        print(f"Invalid input: {e}")


if __name__ == "__main__":
    main()
