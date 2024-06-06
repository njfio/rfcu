"""
This program focuses on the Fibonacci sequence and prime numbers, providing a comprehensive set of functions to generate, analyze, and visualize the Fibonacci sequence and its relationship with prime numbers.

The main features of the program include:

1. Generating the Fibonacci sequence:
   - The `fibonacci_sequence` function generates the Fibonacci sequence up to a specified number of terms.
   - It uses a simple iterative approach to calculate each term of the sequence.

2. Checking for prime numbers:
   - The `is_prime` function determines whether a given number is prime or not.
   - It efficiently checks for divisibility by iterating up to the square root of the number.

3. Counting prime numbers in a sequence:
   - The `count_primes_in_sequence` function counts the number of prime numbers in a given sequence.
   - It iterates over each number in the sequence and uses the `is_prime` function to check for primality.

4. Calculating prime numbers in the Fibonacci sequence:
   - The `calculate_primes_in_fibonacci` function calculates the prime numbers present in the Fibonacci sequence up to a specified number of terms.
   - It generates the Fibonacci sequence using the `fibonacci_sequence` function and then filters out the prime numbers using the `is_prime` function.

5. Plotting the Fibonacci sequence:
   - The `plot_fibonacci_sequence` function plots the Fibonacci sequence as a line plot with markers.
   - It uses the matplotlib library to create a visual representation of the sequence, displaying the term numbers on the x-axis and the corresponding Fibonacci values on the y-axis.

6. Plotting the Fibonacci spiral:
   - The `plot_fibonacci_spiral` function plots the Fibonacci sequence as a spiral plot.
   - It calculates the angles and radii based on the sequence values and uses the matplotlib library to create a spiral representation of the sequence.

The program provides a user-friendly interface through the `main` function. It prompts the user to enter the desired number of Fibonacci terms and performs the following steps:

1. Generates the Fibonacci sequence up to the specified number of terms.
2. Counts the number of prime numbers in the generated sequence.
3. Calculates the specific prime numbers present in the sequence.
4. Displays the generated Fibonacci sequence, the count of prime numbers, and the prime numbers found in the sequence.
5. Plots the Fibonacci sequence as a line plot and a spiral plot using the matplotlib library.

The program includes error handling to ensure that the user enters a valid positive integer for the number of Fibonacci terms. It provides informative error messages and prompts the user to enter a valid input.

Overall, this program offers a comprehensive exploration of the Fibonacci sequence and its relationship with prime numbers. It provides functions to generate and analyze the sequence, as well as visualizations to aid in understanding the patterns and growth of the Fibonacci sequence.

The program utilizes the following libraries:
- `matplotlib.pyplot`: Used for creating visualizations of the Fibonacci sequence.
- `numpy`: Used for generating angles and radii for the Fibonacci spiral plot.

The program consists of the following functions:

1. `is_prime(num)`: Checks if a given number is prime.
   - Takes a number `num` as input and returns `True` if it is prime, `False` otherwise.
   - Efficiently checks for divisibility by iterating up to the square root of the number.

2. `fibonacci_sequence(n)`: Generates the Fibonacci sequence up to a specified number of terms.
   - Takes the number of terms `n` as input and returns a list containing the Fibonacci sequence up to `n` terms.
   - Uses a simple iterative approach to calculate each term of the sequence.

3. `count_primes_in_sequence(sequence)`: Counts the number of prime numbers in a given sequence.
   - Takes a sequence (list) as input and returns the count of prime numbers in that sequence.
   - Iterates over each number in the sequence and uses the `is_prime` function to check for primality.

4. `calculate_primes_in_fibonacci(n)`: Calculates the prime numbers in the Fibonacci sequence up to a specified number of terms.
   - Takes the number of terms `n` as input and returns a list containing the prime numbers found in the Fibonacci sequence up to `n` terms.
   - Generates the Fibonacci sequence using the `fibonacci_sequence` function and then filters out the prime numbers using the `is_prime` function.

5. `plot_fibonacci_sequence(n)`: Plots the Fibonacci sequence as a line plot with markers.
   - Takes the number of terms `n` as input and plots the Fibonacci sequence up to `n` terms.
   - Uses the matplotlib library to create a visual representation of the sequence.
   - Displays the term numbers on the x-axis and the corresponding Fibonacci values on the y-axis.

6. `plot_fibonacci_spiral(sequence)`: Plots the Fibonacci sequence as a spiral plot.
   - Takes the Fibonacci sequence as input and plots it as a spiral.
   - Calculates the angles and radii based on the sequence values and uses the matplotlib library to create a spiral representation of the sequence.

7. `main()`: The main function that executes the program and handles user input.
   - Prompts the user to enter the desired number of Fibonacci terms.
   - Generates the Fibonacci sequence, counts the prime numbers, and calculates the specific prime numbers in the sequence.
   - Displays the results, including the generated sequence, the count of prime numbers, and the prime numbers found.
   - Plots the Fibonacci sequence as a line plot and a spiral plot using the respective functions.
   - Includes error handling to ensure that the user enters a valid positive integer for the number of terms.

To run the program, simply execute the `main()` function. The program will prompt the user to enter the desired number of Fibonacci terms and will generate the corresponding results and visualizations.

Note: The program requires the `matplotlib` and `numpy` libraries to be installed for the visualization functionality to work properly.
"""


def is_prime(num):
    """
    Check if a number is a prime number.

    This function takes a number as input and determines whether it is a prime number or not.
    It returns True if the number is prime and False otherwise.

    Parameters:
    - num (int): The number to be checked for primality.

    Returns:
    - bool: True if the number is prime, False otherwise.

    Algorithm:
    1. If the number is less than or equal to 1, it is not prime, so return False.
    2. If the number is 2, it is prime, so return True.
    3. If the number is even and not 2, it is not prime, so return False.
    4. Iterate from 3 to the square root of the number (inclusive) with a step of 2:
       - If the number is divisible by the current value, it is not prime, so return False.
    5. If the loop completes without finding any divisors, the number is prime, so return True.

    Note:
    - The function uses an efficient approach by checking divisibility only up to the square root of the number.
    - It skips even numbers (except 2) since they are not prime.

    Example:
    >>> is_prime(7)
    True
    >>> is_prime(12)
    False
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


def animate_fibonacci_primes(n, interval=100):
    """Animate the Fibonacci sequence growth mixed with primes."""
    fig, ax = plt.subplots()
    ax.set_xlim(0, n)
    ax.set_ylim(0, fibonacci_sequence(n)[-1])
    ax.set_xlabel("Term")
    ax.set_ylabel("Value")
    ax.set_title("Fibonacci Sequence Growth with Primes")
    ax.grid(True)

    (fib_plot,) = ax.plot([], [], marker="o", linestyle="-", color="blue")
    (prime_plot,) = ax.plot(
        [], [], marker="o", linestyle="", color="red", markersize=10
    )

    def update(frame):
        fib_sequence = fibonacci_sequence(frame)
        fib_plot.set_data(list(range(1, frame + 1)), fib_sequence)

        primes = [i for i in range(1, frame + 1) if is_prime(fibonacci_sequence(i)[-1])]
        prime_fib_values = [fibonacci_sequence(i)[-1] for i in primes]
        prime_plot.set_data(primes, prime_fib_values)

        return fib_plot, prime_plot

    anim = FuncAnimation(fig, update, frames=n, interval=interval, blit=True)
    plt.show()


def main():
    """Main function to execute the program."""
    while True:
        try:
            n = int(input("Enter the number of Fibonacci terms: "))
            if n <= 0:
                print("Please enter a positive integer.")
                continue

            # ... existing code ...

            animate_fibonacci_primes(n, interval=100)
            break

        except ValueError:
            print("Invalid input. Please enter an integer.")


from matplotlib.animation import FuncAnimation


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
