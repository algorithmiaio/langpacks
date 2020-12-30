package com.algorithmia
import play.api.libs.json._

case class InputExample(rows: List[InputRow])

object InputExample{
  implicit val reads: Reads[InputExample] = Json.reads[InputExample]
}


case class InputRow(field_name: String, value: Double)

object InputRow{
  implicit val reads: Reads[InputRow] = Json.reads[InputRow]
}
