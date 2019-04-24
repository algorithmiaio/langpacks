package algorithmia.Algorithm

import org.scalatest._

class __ALGO__Spec extends FlatSpec with Matchers {
  "Initial __ALGO__ algorithm" should "return Hello plus input" in {
    val algorithm = new __ALGO__()
    "Hello Bob" shouldEqual algorithm.apply("Bob")
  }
}
