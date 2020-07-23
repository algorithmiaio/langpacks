package com.algorithmia

import com.algorithmia.handler.AbstractAlgorithm

import ml.combust.bundle.BundleFile
import ml.combust.bundle.dsl.Bundle
import ml.combust.mleap.core.types._
import ml.combust.mleap.runtime.MleapSupport._
import ml.combust.mleap.runtime.frame.{DefaultLeapFrame, Row, Transformer}
import resource._

import scala.collection.mutable
import scala.util.{Failure, Success, Try}

class Algorithm extends AbstractAlgorithm[InputExample, String] {
  var loaded_state = new mutable.HashMap[String, Bundle[Transformer]]()
  val model_uri = "data://zeryx/databricks/simple-spark-pipeline.zip"
  val client: AlgorithmiaClient = Algorithmia.client("simWKYsJU/gOwvomS2k2kyvvlvy1")


  override def load(): Try[Unit] = {
    if (!this.loaded_state.contains("model")) {
      val datafile_path = this.client.file(this.model_uri).getFile.getPath
      val real_path = s"jar:file:$datafile_path"
      Console.println(real_path)
      val bundleFile: Try[Bundle[Transformer]] = BundleFile(real_path)
        .loadMleapBundle()
      bundleFile match {
        case Failure(exception) => return Failure(exception)
        case Success(value) => this.loaded_state.put("model", value)
      }
    }
    else {
      Console.println("already loaded model")
    }
    Console.println("model loaded")
    Success()
  }


  override def apply(input: InputExample): Try[String] = {
    this.load()
    val schema = StructType(StructField("test_string", ScalarType.String),
      StructField("test_double", ScalarType.Double)).get
    val data = Seq(Row("hello", 0.6), Row("MLeap", 0.2))
    val frame = DefaultLeapFrame(schema, data)
    val bundle: Bundle[Transformer] = this.loaded_state.get("model").head
    val mleapPipeline = bundle.root
    val frame2 = mleapPipeline.transform(frame).get
    val data2 = frame2.dataset
    Success("Hello " + data2)
  }
}


object Algorithm {
  val handler = Algorithmia.handler(new Algorithm)

  def main(args: Array[String]): Unit = {
    handler.serve()
  }

  //object  MLeap_demo{
  //  def main(args: Array[String]): Unit = {
  //    val rows: java.util.List[InputRow] = List[InputRow](InputRow(field_name = "hello", value = 0.25)).asJava
  //    val input = InputExample(rows)
  //    val leap_demo = new MLeap_demo()
  //    val res = leap_demo.apply(input)
  //    val res2 = leap_demo.apply(input)
  //    Console.println(res)
  //  }
}

