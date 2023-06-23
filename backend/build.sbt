val scala3Version = "3.3.0"

maintainer := "timzaak<zsy.evan#gmail.com>"

enablePlugins(ScalatraPlugin)

enablePlugins(JavaAppPackaging)

//enablePlugins(ScalikejdbcPlugin)

Compile / PB.targets := Seq(
  scalapb.gen() -> (Compile / sourceManaged).value / "scalapb"
)
Compile / PB.protoSources += file("../protobuf")

// zio-json default value needs this
//ThisBuild / scalacOptions ++= Seq("-Yretain-trees")

lazy val webSugar = RootProject(file("../third/web-sugar"))

lazy val app = project
  .in(file("."))
  .settings(
    version := "0.0.3",
    scalaVersion := scala3Version,
    libraryDependencies ++=
      Seq(
        "org.eclipse.jetty" % "jetty-webapp" % "11.0.15" % "container;compile",
        "org.bouncycastle" % "bcprov-jdk18on" % "1.72", // for x25519,
        "org.scalameta" %% "munit" % "0.7.29" % Test
      )
  )
  .enablePlugins(ScalatraPlugin)
  .enablePlugins(JavaAppPackaging)
  .dependsOn(webSugar) //.enablePlugins(JlinkPlugin)
