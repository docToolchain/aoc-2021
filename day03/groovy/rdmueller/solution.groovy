def input = new File("input.txt").text

def stats = []
12.times { pos ->
    stats[pos] = [0:0,1:0]
    input.eachLine { line ->
        def digit = line[pos] as Integer
        stats[pos][digit]++
    }
}
println stats
def gamma = 0
def epsilon = 0
def value = 1
12.times { i ->
    def pos = 11-i
    println stats[pos]
    if (stats[pos][0]>stats[pos][1]) {
        gamma += value
        println "gamma"
    } else {
        epsilon += value
        println "epsilon"
    }
    value *= 2
}
println gamma
println epsilon
println gamma*epsilon