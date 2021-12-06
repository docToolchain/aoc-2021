import org.assertj.core.api.Assertions
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class SimulatorTest {

    @ParameterizedTest
    @CsvSource(value = [
            "1,             5",
            "2,             6",
            "3,             7",
            "4,             9",
            "5,            10",
            "80,         5934",
            "256, 26984457539"
    ])
    @DisplayName("after days {0} population has grown to {1}")
    void starOne(int afterDays, long expectedPopulation) {
        long[] initialStock = [0,1,1,2,1,0,0,0,0]

        Assertions.assertThat(Simulator.countLanternFishesAfterDays(initialStock, afterDays)).isEqualTo(expectedPopulation)
    }


}
