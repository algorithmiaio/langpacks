package algorithmia.MLeap_demo

import com.algorithmia._
import ml.combust.mleap.runtime.frame.{DefaultLeapFrame, Row, Transformer}
import ml.combust.mleap.core.types._
import ml.combust.bundle.BundleFile
import ml.combust.bundle.dsl.Bundle
import ml.combust.mleap.runtime.MleapSupport._
import resource._

import scala.collection.JavaConverters._
import scala.collection.mutable
import scala.util.{Failure, Success}

class Algorithm extends AbstractAlgorithm[InputExample, String] {
  var loaded_state= new mutable.HashMap[String, Bundle[Transformer]]()
  val model_uri = "data://zeryx/databricks/simple-spark-pipeline.zip"
  val client: AlgorithmiaClient = Algorithmia.client()


  override def load = Try{
    if(!this.loaded_state.contains("model")) {
      val datafile_path =this.client.file(this.model_uri).getFile.getPath
      val real_path = s"jar:file:$datafile_path"
      Console.println(real_path)
      (for (bundleFile <- managed(BundleFile(real_path))) yield {
        bundleFile.loadMleapBundle() match {
          case Success(value) => value;
          case Failure(exception) => Failure(exception)
        }
      }).tried match {
        case Failure(exception) => Failure(exception)
        case Success(value) => this.loaded_state.put("model", value)
      }
    }
      else {
      Console.println("already loaded model")
    }
    Success()
  }

  override def apply(input: InputExample): String = {
    this.load()
    val schema = StructType(StructField("test_string", ScalarType.String),
      StructField("test_double", ScalarType.Double)).get
    val data = Seq(Row("hello", 0.6), Row("MLeap", 0.2))
    val frame = DefaultLeapFrame(schema, data)
    val bundle: Bundle[Transformer] = this.loaded_state.get("model").head
    val mleapPipeline = bundle.root
    val frame2 = mleapPipeline.transform(frame).get
    val data2 = frame2.dataset
    "Hello " + data2
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
