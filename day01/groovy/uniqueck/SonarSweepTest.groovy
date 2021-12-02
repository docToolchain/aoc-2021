import org.junit.jupiter.api.Test

import static org.junit.jupiter.api.Assertions.assertEquals

class SonarSweepTest {

    // tag::part1[]
    @Test
    void countIfNextMeasurementIncreased() {
        assertEquals(7, SonarSweep.countIfNextMeasurementIncreased([199, 200,208,210,200,207,240,269,260,263]))
    }
    // end::part1[]

    // tag::part2[]
    @Test
    void countThreeMeasurementSlidingWindow() {
        assertEquals(5, SonarSweep.countThreeMeasurementSlidingWindow([199, 200,208,210,200,207,240,269,260,263]))
    }
    // end::part2[]

}
