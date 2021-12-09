
class SevenSegmentSearch {

    static void main(String[] args) {
        println "SevenSegmentSearch star1: ${star1(new File("input.txt"))}"
        println "SevenSegmentSearch star2: ${star2(new File("input.txt"))}"
    }

    // tag::part2[]
    static int star2(File input) {

        int sum = 0
        input.readLines().each { String line ->
            def (uniqueSignalPatterns, fourDigitOutputValue) = line.tokenize("|")

            println "UniqueSignalPatterns: ${uniqueSignalPatterns}"
            println "FourDigitOutputValue: ${fourDigitOutputValue}"

            def signalsForDigit = [:] as Map<Integer, String>
            def notDeterminedSignals = [] as List<String>
            uniqueSignalPatterns.trim().split(" ").collect { it.split("").sort().join() }.groupBy { it.size() }.each {

                switch (it.key) {
                    case 2: {
                        signalsForDigit[1] = it.value[0]
                        break
                    }
                    case 4: {
                        signalsForDigit[4] = it.value[0]
                        break
                    }
                    case 3: {
                        signalsForDigit[7] = it.value[0]
                        break
                    }
                    case 7: {
                        signalsForDigit[8] = it.value[0]
                        break
                    }
                    default: {
                        notDeterminedSignals += it.value
                    }
                }
            }

            println "determined signals after unique signals - start"
            println "determined signals: $signalsForDigit"
            println "not determined signals: $notDeterminedSignals"
            println "determined signals after unique signals - end"

            while (notDeterminedSignals) {
                notDeterminedSignals.each { String notDetermined ->
                    switch (notDetermined.size()) {
                        case 5: {
                            // check if already determined 1 is part of pattern, so we found the 3
                            if (notDetermined.contains(signalsForDigit[1][0]) && notDetermined.contains(signalsForDigit[1][1])) {
                                signalsForDigit[3] = notDetermined
                            } else
                            // if already determined 6 and 9, we can determined together with the 1 the last to remaining
                            if (signalsForDigit[6] && signalsForDigit[9] && signalsForDigit[1]) {

                                // check which bit from 1 is not part of 6
                                println signalsForDigit[6]
                                println signalsForDigit[1][0]
                                println signalsForDigit[1][1]
                                String signalForDigit6 = signalsForDigit[6]
                                String bit = signalForDigit6.contains(signalsForDigit[1][0]) ? signalsForDigit[1][1] : signalsForDigit[1][0]
                                println "Bit ${bit} is not part of 6"
                                // remove these bit from 9 and compare it with the current undetermined signal, if it succeed we found the 5
                                if (signalsForDigit[9].replace(bit, "") == notDetermined) {
                                    signalsForDigit[5] = notDetermined
                                } else {
                                    signalsForDigit[2] = notDetermined
                                }
                            }

                            break;
                        }
                        case 6: {
                            // can be 0,9,6
                            // check if already determined 1 is not part of pattern, so we found the 6
                            if (!(notDetermined.contains(signalsForDigit[1][0]) && notDetermined.contains(signalsForDigit[1][1]))) {
                                signalsForDigit[6] = notDetermined
                            } else
                            // if 3 is already determined, we can determine 9
                            if (signalsForDigit[3]
                                    && notDetermined.contains(signalsForDigit[3][0])
                                    && notDetermined.contains(signalsForDigit[3][1])
                                    && notDetermined.contains(signalsForDigit[3][2])
                                    && notDetermined.contains(signalsForDigit[3][3])
                                    && notDetermined.contains(signalsForDigit[3][4])
                            ) {
                                signalsForDigit[9] = notDetermined
                            } else
                            // if 9 and 6 is already determined, the last 6 signal digit is 0
                            if (signalsForDigit[6] && signalsForDigit[9]) {
                                signalsForDigit[0] = notDetermined
                            }
                        }
                    }
                }
                // remove all determined signals from unknown signals
                notDeterminedSignals.removeAll(signalsForDigit.values())
                println "determined signals: $signalsForDigit"
                println "not determined signals: $notDeterminedSignals"
            }


            String tempValue = ""
            fourDigitOutputValue.trim().split(" ").collect {it.split("").sort().join()}.each {String outputPart ->
                println "Search for $outputPart"
                tempValue += signalsForDigit.find {it.value == outputPart}.key
            }
            println "Decoded Output Signal $tempValue"
            sum += tempValue as Integer


        }

        return sum
    }
    // end::part2[]


    // tag::part1[]
    static int star1(File input) {
        int counter = 0
        input.readLines().each { String line ->
            println line
            def (uniqueSignalPatterns, fourDigitOutputValue) = line.tokenize("|")
            List<String> uniqueSignalPatternsList = uniqueSignalPatterns.tokenize(" ")
            println uniqueSignalPatternsList
            List<String> fourDigitOutputValueList = fourDigitOutputValue.tokenize(" ")
            println fourDigitOutputValueList

            fourDigitOutputValueList.findAll { [2, 3, 4, 7].contains(it.size()) }.each {
                counter += createCombinations(it).findAll {
                    uniqueSignalPatternsList.contains(it)
                }.size()
            }
        }
        return counter
    }
    // end::part1[]

    static List<String> createCombinations(String input) {
        def combinations = []
        input.toCharArray().toList().eachPermutation { combinations << it }
        combinations.collect { it.join() } as List<String>
    }

}
