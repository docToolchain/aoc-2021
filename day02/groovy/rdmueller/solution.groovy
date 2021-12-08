def depth = 0
def horizontal = 0
new File("input.txt").eachLine { line ->
    switch (line.split(" ")[0]) {
        case "up":
            depth -= line.split(" ")[1] as Integer
            break;
        case "down":
            depth += line.split(" ")[1] as Integer
            break;
        case "forward":
            horizontal += line.split(" ")[1] as Integer
            break;
    }

}
println (depth * horizontal)
