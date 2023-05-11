package very.util.persistence

import com.typesafe.config.Config
import org.flywaydb.core.Flyway

//ConfigFactory.load().getConfig("database.dataSource")
def pgMigrate(config:Config) =
  val url = config.getString("url")
  val user = config.getString("user")
  val password = config.getString("password")
  val flyway = Flyway.configure().dataSource(url, user, password).load()
  flyway.migrate()
