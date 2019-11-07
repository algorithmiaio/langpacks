name := "algorithm"
organization := "algorithmia"
version := "0.1"
scalaVersion := "2.13.1"

val repoUrl = System.getProperty("repo.url", "http://git.algorithmia.com")

resolvers += "Maven Central" at "http://repo1.maven.org/maven2/org/"

resolvers += "Typesafe Repository" at "http://repo.typesafe.com/typesafe/releases/"


libraryDependencies ++= Seq(
  "com.algorithmia" %% "algorithmia-scala" % "1.0.0",
  "org.scalatest" % "scalatest_2.11" % "3.0.1" % "test"
)

