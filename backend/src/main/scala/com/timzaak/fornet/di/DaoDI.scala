package com.timzaak.fornet.di

import com.timzaak.fornet.dao.{DB, NetworkDao, NodeDao}
import com.typesafe.config.{Config, ConfigFactory}
import org.hashids.Hashids

//import org.json4s.Formats


trait DaoDI {
  //given formats: Formats = org.json4s.DefaultFormats + org.json4s.ext.JOffsetDateTimeSerializer

  given config: Config = ConfigFactory.load()

  object hashId extends Hashids(config.getString("server.hashId"), 5)

  given Hashids = hashId


  object db extends DB
  given DB = db

  object networkDao extends NetworkDao

  object nodeDao extends NodeDao
}
