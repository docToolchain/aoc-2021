import java.io.File
import kotlin.math.*

// tag::read_input[]
fun read(): String {
    var result: String = ""
    File("day2100_puzzle_input.txt").forEachLine {
        result = it
    }
    return result
}
// end::read_input[]

// tag::output[]
//main (args: Arrary<String>) {
    var solution_1 = read()
    println(" I like $solution_1")
//}
// end::output[]