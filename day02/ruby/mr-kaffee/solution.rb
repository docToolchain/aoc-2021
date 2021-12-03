require 'benchmark'

def read_input
  File.read 'input.txt'
end

#tag::calc_position[]
def calc_position(input, init, steps)
  input.split("\n")
       .map { |line| line.split(' ') }
       .inject(init) { |acc, cmd| steps[cmd.first].call acc, cmd.last.to_i }
end
#end::calc_position[]

#tag::calc_position_1[]
def calc_position_1(input)
  calc_position(input, [0, 0], {
    'up' => ->(acc, v) { [acc[0], acc[1] - v] },
    'down' => ->(acc, v) { [acc[0], acc[1] + v] },
    'forward' => ->(acc, v) { [acc[0] + v, acc[1]] }
  })
end
#end::calc_position_1[]

#tag::calc_position_2[]
def calc_position_2(input)
  calc_position(input, [0, 0, 0], {
    'up' => ->(acc, v) { [acc[0], acc[1], acc[2] - v] },
    'down' => ->(acc, v) { [acc[0], acc[1], acc[2] + v] },
    'forward' => ->(acc, v) { [acc[0] + v, acc[1] + acc[2] * v, acc[2]] }
  })
end
#end::calc_position_2[]

# tag::run[]
if __FILE__ == $PROGRAM_NAME
  exp_1 = 2_120_749
  sol_1 = 0
  b_1 = Benchmark.measure {
    acc = calc_position_1 read_input
    sol_1 = acc[0] * acc[1]
    raise "Wrong solution: #{sol_1} != #{exp_1}" unless exp_1 == sol_1
  }
  puts "Solved part 1 in #{b_1.real}s: #{sol_1}"

  exp_2 = 2_138_382_217
  sol_2 = 0
  b_2 = Benchmark.measure {
    acc = calc_position_2 read_input
    sol_2 = acc[0] * acc[1]
    raise "Wrong solution: #{sol_2} != #{exp_2}" unless exp_2 == sol_2
  }
  puts "Solved part 2 in #{b_2.real}s: #{sol_2}"
end
# end::run[]
