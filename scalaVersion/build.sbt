name := "scalaversion"

version := "1.0"

scalaVersion := "2.13.10"   

scalacOptions += "-deprecation"

import sbtassembly.AssemblyPlugin.autoImport._

// Percorso completo desiderato (relativo alla root del progetto)
assembly / assemblyOutputPath := baseDirectory.value / "scalaVersion.jar"

libraryDependencies ++= Seq(
  "org.scalanlp" %% "breeze" % "2.1.0",
  "org.scalanlp" %% "breeze-natives" % "2.1.0" )