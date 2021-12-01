// tag::read_input[]
def input = []
new File("input.txt").eachLine { line ->
    input << (line as Integer)
}
// end::read_input[]

// tag::sliding_windows[]
def windows = [0:10000000000, 1:0,2:0]
def result = []
input.eachWithIndex { line, i ->
    result << windows[i % 3]
    windows[i%3] = 0
    windows[0] += line
    windows[1] += line
    windows[2] += line
}
result << windows[input.size % 3]
result = result [3..-1]
// end::sliding_windows[]

// tag::part1[]
def sum = 0
def prevNum = 1000000000
result.each {
    if (it > prevNum) {
        sum++
    }
    prevNum = it
}
println sum
// end::part1[]
