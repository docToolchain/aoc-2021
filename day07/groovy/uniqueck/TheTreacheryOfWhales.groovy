class TheTreacheryOfWhales {

    static void main(String[] args) {
        println "The Treachery of Wahles Fuel: ${new TheTreacheryOfWhales().fuelPart1(new File("input.txt"))}"
        println "The Treachery of Wahles Fuel: ${new TheTreacheryOfWhales().fuelPart2(new File("input.txt"))}"
    }

    @SuppressWarnings('GrMethodMayBeStatic')
    int fuelPart1(File puzzleInput) {
        int[] input = puzzleInput.readLines().collect {it.split(",")}.flatten().collect {it as int}
        int[] fuel = new int[input.max()]
        for (int i = input.min(); i < input.max(); i++) {
            for (int j = 0; j < input.length; j++) {
                fuel[i] += Math.abs(input[j] - i)
            }
        }
        println fuel
        fuel.min()
    }

    @SuppressWarnings('GrMethodMayBeStatic')
    int fuelPart2(File puzzleInput) {
        int[] input = puzzleInput.readLines().collect {it.split(",")}.flatten().collect {it as int}
        int[] fuel = new int[input.max()]
        for (int i = input.min(); i < input.max(); i++) {
            for (int j = 0; j < input.length; j++) {
                fuel[i] += sumNumbersFrom1Till(Math.abs(input[j] - i))
            }
        }
        println fuel
        fuel.min()
    }

    @SuppressWarnings('GrMethodMayBeStatic')
    int sumNumbersFrom1Till(int number) {
        int sum = 0
        number.times {sum += (it + 1) }
        sum
    }

}
