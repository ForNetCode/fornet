val scala3Version = "3.2.1"

maintainer := "timzaak<zsy.evan#gmail.com>"

enablePlugins(ScalatraPlugin)

enablePlugins(JavaAppPackaging)

//enablePlugins(ScalikejdbcPlugin)

Compile / PB.targets := Seq(
  scalapb.gen() -> (Compile / sourceManaged).value / "scalapb"
)
Compile / PB.protoSources += file("../protobuf")

// zio-json default value needs this
ThisBuild / scalacOptions ++= Seq("-Yretain-trees")

import Dependencies._

lazy val app = project
  .in(file("."))
  .settings(
    version := "0.0.3",
    scalaVersion := scala3Version,
    libraryDependencies ++= grpc ++ persistence ++ logLib ++ webServer ++ configLib ++
      keycloakLib ++ httpClient ++
      Seq(
        "dev.zio" %% "zio-prelude" % "1.0.0-RC16", // for validate
        "com.github.seancfoley" % "ipaddress" % "5.4.0", // for ip parse
        "org.bouncycastle" % "bcprov-jdk18on" % "1.72", // for x25519,
        "org.hashids" % "hashids" % "1.0.3", // hashids
        // "org.keycloak" % "keycloak-servlet-filter-adapter" % "20.0.1", //keycloak
        "org.scalameta" %% "munit" % "0.7.29" % Test
      )
  ) //.enablePlugins(JlinkPlugin)
