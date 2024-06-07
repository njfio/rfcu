# Function to calculate Fibonacci sequence up to n terms
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

# Function to count the number of primes in an array
def count_primes(arr)
  arr.count { |num| prime?(num) }
end

# Function to graph the Fibonacci sequence
def graph_fibonacci(sequence)
  require 'matplotlib'
  x = (0...sequence.length).to_a
  y = sequence
  Matplotlib.plot(x, y)
  Matplotlib.xlabel('Index')
  Matplotlib.ylabel('Fibonacci Number')
  Matplotlib.title('Fibonacci Sequence')
  Matplotlib.show
end

# Function to graph the primes in the Fibonacci sequence
def graph_primes(sequence)
  require 'matplotlib'
  primes = sequence.select { |num| prime?(num) }
  x = sequence.map.with_index { |num, i| i if prime?(num) }.compact
  y = primes
  Matplotlib.scatter(x, y)
  Matplotlib.xlabel('Index')
  Matplotlib.ylabel('Prime Number')
  Matplotlib.title('Primes in Fibonacci Sequence')
  Matplotlib.show
end

# Main function to run the script
def main
  puts "Enter a number:"
  n = gets.to_i

  fibonacci_sequence = fibonacci(n)
  prime_count = count_primes(fibonacci_sequence)

  puts "Fibonacci sequence up to #{n} terms: #{fibonacci_sequence.join(', ')}"
  puts "Number of primes in the sequence: #{prime_count}"

  # Graph the Fibonacci sequence
  graph_fibonacci(fibonacci_sequence)

  # Graph the primes in the Fibonacci sequence
  graph_primes(fibonacci_sequence)
end

# Run the main function
main






# Function to graph the Fibonacci sequence
def graph_fibonacci(sequence)
  require 'matplotlib'
  x = (0...sequence.length).to_a
  y = sequence
  Matplotlib.plot(x, y)
  Matplotlib.xlabel('Index')
  Matplotlib.ylabel('Fibonacci Number')
  Matplotlib.title('Fibonacci Sequence')
  Matplotlib.show
end

# Function to graph the primes in the Fibonacci sequence
def graph_primes(sequence)
  require 'matplotlib'
  primes = sequence.select { |num| prime?(num) }
  x = sequence.map.with_index { |num, i| i if prime?(num) }.compact
  y = primes
  Matplotlib.scatter(x, y)
  Matplotlib.xlabel('Index')
  Matplotlib.ylabel('Prime Number')
  Matplotlib.title('Primes in Fibonacci Sequence')
  Matplotlib.show
end

