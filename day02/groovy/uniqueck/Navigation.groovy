class Navigation {

    static void main(String[] args) {
        // tag::readInput[]
        def lines = new File("input.txt").readLines()
        // end::readInput[]
        // tag::part1-solve[]
        println "Day2 part 1 solution: ${calculateFinalPositionValue(lines)}"
        // end::part1-solve[]
        // tag::part2-solve[]
        println "Day2 part 2 solution: ${calculateFinalPositionValueWithAIM(lines)}"
        // end::part2-solve[]

    }

    // tag::courseEnum[]
    enum Course {
        DOWN, FORWARD, UP
    }
    // end::courseEnum[]

    // tag::part1[]
    static int calculateFinalPositionValue(def plannedCourse = String[]) {
        int horizontalPosition = 0
        int depth = 0

        plannedCourse.each { String plannedStep ->
            def (direction, sValue) = plannedStep.tokenize(" ")
            int speed = Integer.valueOf(sValue)
            switch (Course.valueOf(direction.toUpperCase())) {
                case Course.FORWARD: {
                    horizontalPosition += speed
                    break
                }
                case Course.UP: {
                    depth -= speed
                    break
                }
                case Course.DOWN: {
                    depth += speed
                    break
                }
            }
        }
        return horizontalPosition * depth
    }
    // end::part1[]

    // tag::part2[]
    static int calculateFinalPositionValueWithAIM(def plannedCourse = String[]) {
        int horizontalPosition = 0
        int depth = 0
        int aim = 0

        plannedCourse.each { String plannedStep ->
            def (direction, sValue) = plannedStep.tokenize(" ")
            int speed = Integer.valueOf(sValue)
            switch (Course.valueOf(direction.toUpperCase())) {
                case Course.FORWARD: {
                    horizontalPosition += speed
                    depth += (aim * speed)
                    break
                }
                case Course.UP: {
                    aim -= speed
                    break
                }
                case Course.DOWN: {
                    aim += speed
                }
            }
        }

        println "${horizontalPosition}, ${depth}, ${aim}"


        return horizontalPosition * depth

    }
    // end::part2[]

}
