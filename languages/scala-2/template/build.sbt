name := "algorithm"
organization := "algorithmia"
version := "0.1"
scalaVersion := "2.13.1"

resolvers += "Maven Central" at "http://repo1.maven.org/maven2/org/"

resolvers += "Typesafe Repository" at "http://repo.typesafe.com/typesafe/releases/"

enablePlugins(JavaAppPackaging)

libraryDependencies ++= Seq(
  "com.algorithmia" %% "algorithmia-scala" % "1.0.+",
  "org.scalatest" %% "scalatest" % "3.0.1" % Test
)

