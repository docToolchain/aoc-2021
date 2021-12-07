import org.junit.jupiter.api.Test

class TheTreacheryOfWhalesTest {

    @Test
    void fuel() {
        assert 37 == new TheTreacheryOfWhales().fuelPart1(new File("example.txt"))
        assert 168 == new TheTreacheryOfWhales().fuelPart2(new File("example.txt"))
    }

    @Test
    void sumNumberFrom1Till() {
        assert 1 == new TheTreacheryOfWhales().sumNumbersFrom1Till(1)
        assert 3 == new TheTreacheryOfWhales().sumNumbersFrom1Till(2)
        assert 6 == new TheTreacheryOfWhales().sumNumbersFrom1Till(3)
        assert 10 == new TheTreacheryOfWhales().sumNumbersFrom1Till(4)
    }
}
