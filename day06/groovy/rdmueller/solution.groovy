#!/usr/bin/env groovy
// export JAVA_OPTS=-Xmx2048m
List<byte> input = new File("test.txt").text.split(",").collect{it as byte}
List<byte> nextInput
long sum = 0
input.eachWithIndex { it, j ->
    input2 = [it]
    80.times { day ->
        nextInput = []
        input2.each { byte counter ->
            counter--
            if (counter == -1) {
                counter = 6 as byte
                nextInput << (8 as byte)
            }
            nextInput << counter
        }
        input2 = nextInput
    }
    sum += input2.size()
}
println ">>> "+sum
