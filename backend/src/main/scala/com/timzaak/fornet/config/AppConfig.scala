package com.timzaak.fornet.config

import com.typesafe.config.Config

trait AppConfig {
  def enableSAAS:Boolean
}

class AppConfigImpl(config:Config) extends AppConfig {
  import very.util.config.*

  override val enableSAAS: Boolean = config.getOptional[Boolean]("server.saas").getOrElse(false)
}
