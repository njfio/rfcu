# Calculates the Fibonacci sequence up to n terms.
#
# The function takes an integer n as input and returns an array containing the Fibonacci sequence up to n terms.
# The Fibonacci sequence starts with 0 and 1, and each subsequent number is the sum of the two preceding numbers.
#
# Parameters:
#   n (Integer): The number of terms in the Fibonacci sequence to calculate.
#
# Returns:
#   Array: The Fibonacci sequence up to n terms.
#
# Example:
#   fib_seq = fibonacci(10)
#   puts fib_seq  # Output: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

# Checks if a given number is prime.
#
# The function takes a number as input and returns true if the number is prime, and false otherwise.
# A prime number is a number greater than 1 that has no positive divisors other than 1 and itself.
#
# Parameters:
#   num (Integer): The number to check for primality.
#
# Returns:
#   Boolean: true if the number is prime, false otherwise.
#
# Example:
#   is_prime = prime?(17)
#   puts is_prime  # Output: true

# The main function to run the script.
#
# This function prompts the user to enter a number, calculates the Fibonacci sequence up to that number of terms,
# counts the number of prime numbers in the sequence, and displays the results.
# It also graphs the Fibonacci sequence and the prime numbers in the sequence using the `graph_fibonacci` and
# `graph_primes` functions, respectively.
#
# Example:
#   Enter a number:
#   10
#   Fibonacci sequence up to 10 terms: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
#   Number of primes in the sequence: 4
#   Fibonacci sequence graphed and saved as 'fibonacci.png'.
#   Prime numbers in the sequence graphed and saved as 'primes.png'.

# Graphs the Fibonacci sequence.
#
# The function takes an array representing the Fibonacci sequence and a filename as input.
# It creates a PNG image using the ChunkyPNG library, where each number in the sequence is represented as a black pixel.
# The image is then saved with the specified filename.
#
# Parameters:
#   sequence (Array): The Fibonacci sequence to graph.
#   filename (String): The name of the file to save the graph as.
#
# Example:
#   fib_seq = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
#   graph_fibonacci(fib_seq, 'fibonacci.png')

# Graphs the prime numbers in the Fibonacci sequence.
#
# The function takes an array representing the Fibonacci sequence and a filename as input.
# It selects the prime numbers from the sequence and creates a PNG image using the ChunkyPNG library.
# Each prime number in the sequence is represented as a black pixel at its corresponding index.
# The image is then saved with the specified filename.
#
# Parameters:
#   sequence (Array): The Fibonacci sequence containing the numbers to check for primality.
#   filename (String): The name of the file to save the graph as.
#
# Example:
#   fib_seq = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
#   graph_primes(fib_seq, 'primes.png')



# Calculates the Fibonacci sequence up to n terms.
#
# The function takes an integer n as input and returns an array containing the Fibonacci sequence up to n terms.
# The Fibonacci sequence starts with 0 and 1, and each subsequent number is the sum of the two preceding numbers.
#
# Parameters:
#   n (Integer): The number of terms in the Fibonacci sequence to calculate.
#
# Returns:
#   Array: The Fibonacci sequence up to n terms.
#
# Example:
#   fib_seq = fibonacci(10)
#   puts fib_seq  # Output: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
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

# Counts the number of prime numbers in the given array.
#
# The function takes an array of numbers as input and returns the count of prime numbers in the array.
# It relies on the `prime?` helper function to determine if a number is prime.
#
# Parameters:
#   arr (Array): An array of numbers.
#
# Returns:
#   Integer: The count of prime numbers in the input array.
#
# Example:
#   arr = [2, 3, 4, 5, 6, 7, 8, 9, 10]
#   count = count_primes(arr)
#   puts count  # Output: 4
def count_primes(arr)
  arr.count { |num| prime?(num) }
end

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

# Main function to run the script


require 'gnuplot'

# Function to create a 3D plot of the Fibonacci sequence
def plot_fibonacci_3d(sequence, filename)
  Gnuplot.open do |gp|
    Gnuplot::SPlot.new(gp) do |plot|
      plot.title  "Fibonacci Sequence 3D Plot"
      plot.xlabel "Index"
      plot.ylabel "Fibonacci Number"
      plot.zlabel "Height"

      x = (0...sequence.length).to_a
      y = sequence
      z = x.map { |i| i / 10.0 }

      plot.data << Gnuplot::DataSet.new([x, y, z]) do |ds|
        ds.with = "lines"
        ds.linewidth = 2
      end
    end

    gp.terminal "png"
    gp.output filename
  end
end
# Plot the Fibonacci sequence in 3D
plot_fibonacci_3d(fibonacci_sequence, 'fibonacci_3d.png')
puts "Fibonacci sequence plotted in 3D and saved as 'fibonacci_3d.png'."

def main
  puts "Enter a number:"
  n = gets.to_i

  fibonacci_sequence = fibonacci(n)
  prime_count = count_primes(fibonacci_sequence)

  puts "Fibonacci sequence up to #{n} terms: #{fibonacci_sequence.join(', ')}"
  puts "Number of primes in the sequence: #{prime_count}"

  # Graph the Fibonacci sequence
  graph_fibonacci(fibonacci_sequence, 'fibonacci.png')
  puts "Fibonacci sequence graphed and saved as 'fibonacci.png'."

  # Graph the prime numbers in the sequence
  graph_primes(fibonacci_sequence, 'primes.png')
  puts "Prime numbers in the sequence graphed and saved as 'primes.png'."
end

# Run the main function
main



