"""
This module provides functions to generate the Fibonacci sequence up to a given number
and count the number of prime numbers in the sequence. It also includes a main function
to handle user input and print the results.

Functions:
- fibonacci_sequence(n: int) -> list:
    Generate the Fibonacci sequence up to the nth number.

- count_primes_in_sequence(sequence: list) -> int:
    Count the number of prime numbers in the given sequence.

- is_prime(num: int) -> bool:
    Helper function to check if a number is prime.

- main() -> None:
    Main function to handle user input and print the Fibonacci sequence.

The module also includes unit tests for the functions using the unittest module.
The tests cover various scenarios, including valid and invalid inputs, to ensure
the correctness of the implemented functions.

Usage:
1. Run the script directly to execute the main function and interact with the program.
2. Import the module and use the individual functions as needed.

Example:
>>> from fibonacci import fibonacci_sequence, count_primes_in_sequence
>>> sequence = fibonacci_sequence(10)
>>> print(sequence)
[0, 1, 1, 2, 3, 5, 8]
>>> prime_count = count_primes_in_sequence(sequence)
>>> print(prime_count)
3

Note:
- The Fibonacci sequence starts with 0 and 1, and each subsequent number is the sum
  of the two preceding numbers.
- A prime number is a positive integer greater than 1 that has no positive divisors
  other than 1 and itself.

Dependencies:
- unittest: A built-in Python module for writing and running unit tests.
- unittest.mock: A module for creating mock objects and patching functions during testing.
"""


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
        prime_count = count_primes_in_sequence(sequence)
        print(f"Fibonacci sequence up to {user_input}: {sequence}")
        print(f"Number of prime numbers in the sequence: {prime_count}")
    except ValueError as e:
        print(f"Invalid input: {e}")


import unittest
from unittest.mock import patch

from fibonacci import count_primes_in_sequence, fibonacci_sequence, main


class TestFibonacciSequence(unittest.TestCase):
    def test_fibonacci_sequence(self):
        self.assertEqual(fibonacci_sequence(0), [0])
        self.assertEqual(fibonacci_sequence(1), [0, 1, 1])
        self.assertEqual(fibonacci_sequence(10), [0, 1, 1, 2, 3, 5, 8])
        self.assertEqual(fibonacci_sequence(20), [0, 1, 1, 2, 3, 5, 8, 13])


class TestCountPrimesInSequence(unittest.TestCase):
    def test_count_primes_in_sequence(self):
        self.assertEqual(count_primes_in_sequence([]), 0)
        self.assertEqual(count_primes_in_sequence([0, 1, 1, 2, 3, 5, 8]), 3)
        self.assertEqual(count_primes_in_sequence([0, 1, 1, 2, 3, 5, 8, 13]), 4)


class TestMain(unittest.TestCase):
    @patch("builtins.input", side_effect=["10"])
    @patch("builtins.print")
    def test_main_valid_input(self, mock_print, mock_input):
        main()
        mock_print.assert_any_call("Fibonacci sequence up to 10: [0, 1, 1, 2, 3, 5, 8]")
        mock_print.assert_any_call("Number of prime numbers in the sequence: 3")

    @patch("builtins.input", side_effect=["-5"])
    @patch("builtins.print")
    def test_main_negative_input(self, mock_print, mock_input):
        main()
        mock_print.assert_called_once_with(
            "Invalid input: The number must be non-negative."
        )

    @patch("builtins.input", side_effect=["abc"])
    @patch("builtins.print")
    def test_main_invalid_input(self, mock_print, mock_input):
        main()
        mock_print.assert_called_once_with(
            "Invalid input: invalid literal for int() with base 10: 'abc'"
        )


if __name__ == "__main__":
    main()
