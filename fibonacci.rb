# Generates the Fibonacci sequence up to the specified number of terms.
#
# Parameters:
#   - n: The number of terms to generate in the Fibonacci sequence.
#
# Returns:
#   - An array containing the Fibonacci sequence up to the specified number of terms.
#
# Behavior:
#   - If n is less than or equal to 0, an empty array is returned.
#   - If n is equal to 1, an array containing [0] is returned.
#   - If n is equal to 2, an array containing [0, 1] is returned.
#   - For n greater than 2, the function generates the Fibonacci sequence up to n terms
#     by iteratively adding the sum of the last two numbers to the sequence.
#
# Example:
#   fibonacci(5) # Returns [0, 1, 1, 2, 3]
#   fibonacci(10) # Returns [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

def fibonacci(n)
  return [] if n <= 0
  return [0] if n == 1
  return [0, 1] if n == 2

  fib = [0, 1]
  (2...n).each do |i|
    fib << fib[-1] + fib[-2]
  end
  fib
end

# Function to check if a number is prime
def prime?(num)
  return false if num <= 1
  (2..Math.sqrt(num)).each do |i|
    return false if num % i == 0
  end
  true
end

def count_primes(arr)
  arr.count { |num| prime?(num) }
end

require "chunky_png"

# Function to graph the Fibonacci sequence
def graph_fibonacci(sequence, filename)
  graph = ChunkyPNG::Image.new(sequence.length, sequence.max + 10, ChunkyPNG::Color::WHITE)

  sequence.each_with_index do |num, index|
    graph[index, sequence.max - num] = ChunkyPNG::Color::BLACK
  end

  graph.save(filename)
end

# Function to graph the prime numbers
def graph_primes(sequence, filename)
  primes = sequence.select { |num| prime?(num) }
  graph = ChunkyPNG::Image.new(sequence.length, 10, ChunkyPNG::Color::WHITE)

  primes.each do |prime|
    index = sequence.index(prime)
    graph[index, 5] = ChunkyPNG::Color::BLACK
  end

  graph.save(filename)
end

# Main function to run the script

def prime_distances(sequence)
  primes = sequence.select { |num| prime?(num) }
  primes_pi = primes.map { |prime| prime * Math::PI }
  distances = []

  primes_pi.each_cons(2) do |a, b|
    distances << b - a
  end

  [primes_pi, distances]
end

def main
  puts "Enter a number:"
  n = gets.to_i

  fibonacci_sequence = fibonacci(n)
  prime_count = count_primes(fibonacci_sequence)

  puts "Fibonacci sequence up to #{n} terms: #{fibonacci_sequence.join(", ")}"
  puts "Number of primes in the sequence: #{prime_count}"

  primes_pi, distances = prime_distances(fibonacci_sequence)
  puts "Primes multiplied by PI: #{primes_pi.join(", ")}"
  puts "Distances between consecutive primes multiplied by PI: #{distances.join(", ")}"
end

# Run the main function
main
