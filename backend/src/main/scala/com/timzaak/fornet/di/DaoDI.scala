package com.timzaak.fornet.di

import com.timzaak.fornet.dao.{DB, NetworkDao, NodeDao}
//import org.json4s.Formats


trait DaoDI {
  //given formats: Formats = org.json4s.DefaultFormats + org.json4s.ext.JOffsetDateTimeSerializer

  object db extends DB
  given DB = db

  object networkDao extends NetworkDao

  object nodeDao extends NodeDao
}
