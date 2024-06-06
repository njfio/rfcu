"""
This script demonstrates various operations related to the Fibonacci sequence and prime numbers.

Functions:
- is_prime(num): Checks if a number is prime.
- fibonacci_sequence(n): Generates the Fibonacci sequence up to n terms.
- count_primes_in_sequence(sequence): Counts the number of prime numbers in a given sequence.
- calculate_primes_in_fibonacci(n): Calculates the prime numbers in the Fibonacci sequence up to n terms.
- plot_fibonacci_sequence(n): Plots the Fibonacci sequence up to n terms.
- plot_fibonacci_spiral(sequence): Plots the Fibonacci sequence as a spiral.
- main(): The main function that executes the program.

Usage:
1. Run the script.
2. Enter the number of Fibonacci terms you want to generate.
3. The script will display the following:
   - The Fibonacci sequence up to the specified number of terms.
   - The number of prime numbers in the sequence.
   - The prime numbers in the sequence.
4. The script will then plot two graphs:
   - A line plot of the Fibonacci sequence.
   - A spiral plot of the Fibonacci sequence.

Dependencies:
- matplotlib: Used for plotting graphs.
- numpy: Used for mathematical operations and generating arrays.

Note:
- The script assumes that the user will input a valid positive integer for the number of Fibonacci terms.
- If an invalid input is provided, the script will prompt the user to enter a valid integer.
- The script uses the `is_prime` function to check if a number is prime.
- The `count_primes_in_sequence` function counts the number of prime numbers in a given sequence.
- The `calculate_primes_in_fibonacci` function calculates the prime numbers in the Fibonacci sequence.
- The `plot_fibonacci_sequence` function plots the Fibonacci sequence as a line plot.
- The `plot_fibonacci_spiral` function plots the Fibonacci sequence as a spiral.

Example:
Enter the number of Fibonacci terms: 10
Fibonacci sequence up to 10 terms: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
Number of prime numbers in the sequence: 3
Prime numbers in the sequence: [2, 3, 5]
"""
"""This is my doc string
"""


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


"""
Count the number of prime numbers in a given sequence.

Parameters:
    sequence (list): A sequence of numbers.

Returns:
    int: The count of prime numbers in the sequence.

Description:
    This function takes a sequence of numbers as input and counts the number of prime numbers
    present in the sequence. It iterates over each number in the sequence and uses the `is_prime`
    function to check if the number is prime. If a number is prime, the count is incremented.
    Finally, the function returns the total count of prime numbers in the sequence.

Example:
    >>> sequence = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    >>> count_primes_in_sequence(sequence)
    4

Note:
    The function assumes the existence of an `is_prime` function that takes a number as input
    and returns `True` if the number is prime, and `False` otherwise.
"""


def count_primes_in_sequence(sequence):
    """Count prime numbers in a given sequence."""
    count = 0
    for num in sequence:
        if is_prime(num):
            count += 1
    return count


def calculate_primes_in_fibonacci(n):
    """Calculate prime numbers in the Fibonacci sequence up to n terms."""
    fib_sequence = fibonacci_sequence(n)
    primes = [num for num in fib_sequence if is_prime(num)]
    return primes


import matplotlib.pyplot as plt


def plot_fibonacci_sequence(n):
    """Plot the Fibonacci sequence up to n terms."""
    sequence = fibonacci_sequence(n)
    x = list(range(1, n + 1))
    y = sequence

    plt.figure(figsize=(10, 6))
    plt.plot(x, y, marker="o", linestyle="-", color="blue")
    plt.xlabel("Term")
    plt.ylabel("Value")
    plt.title("Fibonacci Sequence")
    plt.grid(True)
    plt.show()


import matplotlib.pyplot as plt
import numpy as np


def plot_fibonacci_spiral(sequence):
    """Plot the Fibonacci sequence as a spiral."""
    angles = np.linspace(0, np.pi * 2 * len(sequence), len(sequence))
    radius = np.sqrt(sequence)  # Use square root of sequence values as radius
    x = np.cos(angles) * radius
    y = np.sin(angles) * radius

    plt.figure(figsize=(8, 8))
    plt.plot(x, y, color="blue")
    plt.fill(x, y, color="blue", alpha=0.3)
    plt.axis("equal")
    plt.title("Fibonacci Spiral")
    plt.grid(False)
    plt.show()


def main():
    """Main function to execute the program."""
    while True:
        try:
            n = int(input("Enter the number of Fibonacci terms: "))
            if n <= 0:
                print("Please enter a positive integer.")
                continue

            fib_sequence = fibonacci_sequence(n)
            prime_count = count_primes_in_sequence(fib_sequence)
            prime_numbers = calculate_primes_in_fibonacci(n)

            print(f"Fibonacci sequence up to {n} terms: {fib_sequence}")
            print(f"Number of prime numbers in the sequence: {prime_count}")
            print(f"Prime numbers in the sequence: {prime_numbers}")

            plot_fibonacci_sequence(n)
            plot_fibonacci_spiral(fib_sequence)
            break

        except ValueError:
            print("Invalid input. Please enter an integer.")


if __name__ == "__main__":
    main()

import matplotlib.pyplot as plt
import numpy as np
