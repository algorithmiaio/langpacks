//
// Algorithmia algorithm build file
//

name := "__ALGO__"

organization := "algorithmia"

// Allow version to be overwritten with "-DalgoVersion=XXX"
version := System.getProperty("algo.version", "1.0-SNAPSHOT")

scalaVersion := "2.11.7"

mainClass in Compile := Some("algorithmia.Main")

val repoUrl = System.getProperty("repo.url", "http://git.algorithmia.com")

resolvers += "Maven Central" at "http://repo1.maven.org/maven2/org/"

resolvers += "Typesafe Repository" at "http://repo.typesafe.com/typesafe/releases/"

libraryDependencies ++= Seq(
  "com.algorithmia" % "algorithmia-client" % "1.0.+",
  "com.algorithmia" % "algorithmia-extras" % "1.0.+",
  "com.google.code.gson" % "gson" % "2.5"
)

retrieveManaged := true

// Don't convert name to lowercase
normalizedName := name.value
