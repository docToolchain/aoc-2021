class BinaryDiagnostic {

    // tag::main[]
    static void main(String[] args) {
        // tag::part1[]
        println "Day3 power consumption rating: ${calculatePowerConsumption(new File("input.txt").readLines())}"
        // end::part1[]
        // tag::part2[]
        println "Day3 life support rating: ${calculateLifeSupportRating(new File("input.txt").readLines())}"
        // end::part2[]
    }
    // end::main[]

    // tag::calculatePowerConsumption[]
    static int calculatePowerConsumption(List<String> diagnosticReport) {

        def gammaBinary = ""
        def epsilonBinary = ""
        def tempValue = 0
        def length = diagnosticReport[0].length()
        int half = diagnosticReport.size() / 2

        for (int position = 0; position < length; position++) {
            tempValue = 0
            diagnosticReport.each { String it ->
                tempValue += Integer.parseInt(it[position])
            }
            gammaBinary += tempValue > half ? "1" : "0"
            epsilonBinary += tempValue > half ? "0" : "1"
        }
        println gammaBinary
        println epsilonBinary

        return Integer.parseInt(gammaBinary, 2) * Integer.parseInt(epsilonBinary, 2)
    }
    // end::calculatePowerConsumption[]

    // tag::calculateLifeSupportRating[]
    static int calculateLifeSupportRating(List<String> diagnosticReport) {
        // calculate oxygen generator rating
        String oxygenGeneratorRating = filterByCriteria(diagnosticReport, { int[] tempBitsCounted -> tempBitsCounted[0] > tempBitsCounted[1] ? "0" : "1" })
        String co2ScrubberRating = filterByCriteria(diagnosticReport, { int[] tempBitsCounted -> tempBitsCounted[0] <= tempBitsCounted[1] ? "0" : "1" })
        return Integer.parseInt(oxygenGeneratorRating, 2) * Integer.parseInt(co2ScrubberRating, 2)
    }

    static String filterByCriteria(List<String> diagnosticReport, Closure<String> filterBitClosure) {
        int position = 0
        List<String> tempList = diagnosticReport
        do {
            String filterBit = filterBitClosure.call(countBitsOnPosition(position, tempList))
            tempList = tempList.findAll { it[position] == filterBit }
            position++
        } while (tempList.size() > 1)
        return tempList[0]
    }

    static int[] countBitsOnPosition(int position, List<String> diagnosticReport) {
        def by = diagnosticReport.countBy { it[position] }
        return [by."0", by."1"]
    }
    // end::calculateLifeSupportRating[]
}
