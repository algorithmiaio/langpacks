package com.algorithmia

import com.algorithmia.handler.AbstractAlgorithm

import scala.util.{Success, Try}

class Algorithm extends AbstractAlgorithm[String, String]{

  var someVariable: String = _

  override def apply(input: String): Try[String] = {
    Success(s"hello $input")
  }
  override def load = {
    someVariable = "loaded"
    Success()
  }
}

object Algorithm {
  val handler = Algorithmia.handler(new Algorithm)

  def main(args: Array[String]): Unit = {
    handler.serve()
  }
}
