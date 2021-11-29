require "benchmark"

#tag::say_hello[]
def say_hello(name = "World")
  "Hello, #{name}!"
end

#end::say_hello[]

#tag::run[]
if __FILE__ == $0
  b = Benchmark.measure { puts say_hello }
  puts "Solved in #{b.real}s"
end

#end::run[]
