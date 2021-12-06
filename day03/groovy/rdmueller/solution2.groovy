def input = new File("input.txt").text.split("[\n\r]+")

def calcStats = {  in ->
    def stats = []
    12.times { pos ->
        stats[pos] = [0: 0, 1: 0]
        in.each { line ->
            def digit = line[pos] as Integer
            stats[pos][digit]++
        }
    }
    return stats
}
def binToInt = { bin ->
    bin = bin.trim()
    def sum = 0
    bin.eachWithIndex { digit, i ->
        sum = sum * 2 + (digit as Integer)
    }
    return sum
}
12.times { pos->
    def stats = calcStats(input)
    def newInput = []
    input.each { line ->
        if (stats[pos][0]>stats[pos][1]) {
            if (line[pos]=='0') {
                newInput << line
            }
        } else {
            if (line[pos]=='1') {
                newInput << line
            }
        }
    }
    if (newInput.size()>0) {
        input = newInput
    }
}
println binToInt(input[0])
def a = binToInt(input[0])
input = new File("input.txt").text.split("[\n\r]+")
12.times { pos->
    def stats = calcStats(input)
    def newInput = []
    input.each { line ->
        if (stats[pos][0]>stats[pos][1]) {
            if (line[pos]=='1') {
                newInput << line
            }
        } else {
            if (line[pos]=='0') {
                newInput << line
            }
        }
    }
    if (newInput.size()>0) {
        input = newInput
    }
}
println input
println binToInt(input[0])
println a*binToInt(input[0])