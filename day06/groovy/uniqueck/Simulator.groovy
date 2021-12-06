class Simulator {

    static void main(String[] args) {
        def countBy = new File("input.txt").readLines().collect { it.split(",") }.flatten().collect { it as int }.countBy { it }
        long[] initialStock = [0,0,0,0,0,0,0,0,0]
        countBy.each {initialStock[it.key] += it.value}
        println initialStock
        println "Star1: ${countLanternFishesAfterDays(initialStock, 80)}"
        println "Star2: ${countLanternFishesAfterDays(initialStock, 256)}"
    }

    static long countLanternFishesAfterDays(long[] initialStock, int afterDays, int currentDay = 1) {
        println "Day: ${currentDay} - ${initialStock}"
        long[] newFishes = [0,0,0,0,0,0,0,0,0]
        initialStock.eachWithIndex { long count, int index ->
            if (index > 0) {
                newFishes[index - 1] += count
            } else {
                newFishes[6] += count
                newFishes[8] += count
            }
        }
        if (currentDay < afterDays) {
            return countLanternFishesAfterDays(newFishes, afterDays, currentDay + 1)
        } else {
            return newFishes.sum()
        }
    }



}
