def depth = 0
def horizontal = 0
def aim = 0
new File("input.txt").eachLine { line ->
    switch (line.split(" ")[0]) {
        case "up":
            aim -= line.split(" ")[1] as Integer
            break;
        case "down":
            aim += line.split(" ")[1] as Integer
            break;
        case "forward":
            horizontal += line.split(" ")[1] as Integer
            depth += aim*(line.split(" ")[1] as Integer)
            break;
    }

}
println (depth * horizontal)
