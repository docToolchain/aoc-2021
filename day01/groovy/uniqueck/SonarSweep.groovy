class SonarSweep {

    static void main(String[] args) {

        // tag::collect-data[]
        def collect = new File('input.txt').text.split('\n').collect { Integer::valueOf(it) }
        // end::collect-data[]
        // tag::part1[]
        println "Part1 solution: ${countIfNextMeasurementIncreased(collect)}"
        // end::part1[]
        // tag::part2[]
        println "Part2 solution: ${countThreeMeasurementSlidingWindow(collect)}"
        // end::part2[]
    }

    // tag::iterate-over-list[]
    static int countIfNextMeasurementIncreased(List<Integer> input) {
        int counter = 0
        for (int i = 0; i < input.size() - 1; i++) {
            counter += count(input[i], input[i+1])
        }
        counter
    }
    // end::iterate-over-list[]

    // tag::iterate-over-list-and-create-sub-lists[]
    static int countThreeMeasurementSlidingWindow(List<Integer> input) {
        int counter = 0
        for (int i = 0; i < input.size() - 3; i++) {
            int sum1 = input.subList(i, i +3).sum(0) as int
            int sum2 = input.subList(i+1, i +4).sum(0) as int
            counter += count(sum1, sum2)
        }
        counter
    }
    // end::iterate-over-list-and-create-sub-lists[]

    // tag::count[]
    static int count(int a, b) {
        b > a ? 1 : 0
    }
    // end::count[]

}
