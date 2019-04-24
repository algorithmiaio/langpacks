package algorithmia.Algorithm

import com.algorithmia._
import com.algorithmia.algo._
import com.algorithmia.data._
import com.algorithmia.algorithmHandler._
import com.google.gson._


object Algorithm{
  def apply(input: String, context: Map[String, String]): String = {
    s"Hello $input your file is available here ${context.get("local_path")}"
  }

  def download_model():Map[String, String] = {
    val context: Map[String, String] = Map()
    val client = Algorithmia.client("YOUR_API_KEY")
    val local_path = client.file("data://demo/collection/somefile.json").getFile.getName
    context + "local_path" -> local_path
    context
  }

  def main(args: Array[String]): Unit = {
    AlgorithmHandler(apply)
      .setOnLoad(download_model)
      .run()
  }
}