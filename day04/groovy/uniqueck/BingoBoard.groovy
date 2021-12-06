class BingoBoard {

    static class Field {
        int value
        boolean marked

        private Field(int value) {
            this.value = value
            this.marked = false
        }

        static Field of(int value) {
            return new Field(value)
        }

        String toString() {
            return (marked ? "X" : "${value}").padLeft(2, " ")
        }
    }

    List<Field> fields


    static BingoBoard init(String[] boardLayout) {
        def fields = boardLayout.collect { it.split(" ").findAll { it.trim() }.collect { Field.of(it.trim() as int) } }.flatten()
        return new BingoBoard(fields)
    }

    private BingoBoard(def fields) {
        this.fields = fields
    }

    boolean isWins() {
        // check rows
        def match = false
        int i = 0
        do {
            // check line
            match |= fields.subList(i * 5, 5 * i + 5).every { it.marked }
            // check row
            match |= fields.collate(5).every { it[i].marked }
            i++
        } while (!match && i < 5)
        return match
    }

    int sumOfAllUnmarkedNumbers() {
        def sum = fields.findAll { !it.marked }.collect { it.value as int }.sum(0)
        return sum as int
    }

    boolean markNumber(int value) {
        // search field with this number
        def field = fields.find { it.value == value }
        if (field) {
            field.marked = true
            return wins
        } else {
            return false
        }
    }

    String toString() {
        return fields.collect { it.toString() }.collate(5).collect {it.join(" ")}.join("\n")
    }


}
