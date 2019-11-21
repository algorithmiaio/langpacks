name := "algorithm"
organization := "algorithmia"
version := "0.1"
scalaVersion := "2.13.1"

resolvers += "Typesafe Repository" at "https://repo.typesafe.com/typesafe/releases/"

enablePlugins(JavaAppPackaging)

libraryDependencies ++= Seq(
  "com.algorithmia" %% "algorithmia-scala" % "1.0.+",
  "org.scalatest" %% "scalatest" % "3.0.8" % Test
)

