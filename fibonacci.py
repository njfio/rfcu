"""
This program focuses on the Fibonacci sequence and prime numbers. It provides functions to generate the Fibonacci sequence, check if a number is prime, count prime numbers in a sequence, and calculate prime numbers in the Fibonacci sequence.

The main functionality of the program includes:

1. Generating the Fibonacci sequence up to a specified number of terms.
2. Checking if a given number is prime using the `is_prime` function.
3. Counting the number of prime numbers in a given sequence using the `count_primes_in_sequence` function.
4. Calculating the prime numbers in the Fibonacci sequence up to a specified number of terms using the `calculate_primes_in_fibonacci` function.
5. Plotting the Fibonacci sequence using the `plot_fibonacci_sequence` function, which displays the sequence as a line plot with markers.
6. Plotting the Fibonacci sequence as a spiral using the `plot_fibonacci_spiral` function, which represents the sequence as a spiral plot.

The program prompts the user to enter the number of Fibonacci terms they want to generate. It then generates the Fibonacci sequence up to that number of terms, counts the prime numbers in the sequence, and calculates the specific prime numbers present in the sequence.

The program utilizes the matplotlib library to create visual representations of the Fibonacci sequence. It plots the sequence as a line plot and as a spiral plot, providing a visual understanding of the sequence's growth and pattern.

The main function (`main`) handles the user input, executes the relevant functions, and displays the results. It also includes error handling to ensure that the user enters a valid positive integer for the number of Fibonacci terms.

Overall, this program demonstrates the generation and analysis of the Fibonacci sequence, prime number calculations within the sequence, and visual representations of the sequence using matplotlib.
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
