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



# Main function to run the script
def main
  puts "Enter a number:"
  n = gets.to_i

  fibonacci_sequence = fibonacci(n)
  prime_count = count_primes(fibonacci_sequence)

  puts "Fibonacci sequence up to #{n} terms: #{fibonacci_sequence.join(', ')}"
  puts "Number of primes in the sequence: #{prime_count}"


end

# Run the main function
main






require 'chunky_png'

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

