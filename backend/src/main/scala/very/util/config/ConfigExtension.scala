package very.util.config

import com.typesafe.config.{Config, ConfigList, ConfigMemorySize, ConfigObject}

import java.net.{URI, URL}
import java.time.Period
import java.time.temporal.TemporalAmount
import scala.concurrent.duration.{Duration, FiniteDuration}

//import scala.collection.JavaConverters._
import scala.jdk.CollectionConverters._

trait ConfigLoader[A] {
  self =>
  def load(config: Config, path: String = ""): A

  def map[B](f: A => B): ConfigLoader[B] = (config, path) => f(self.load(config, path))
}

object ConfigLoader {
  def apply[A](f: Config => String => A): ConfigLoader[A] = f(_)(_)

  implicit val stringLoader: ConfigLoader[String] = ConfigLoader(_.getString)
  implicit val seqStringLoader: ConfigLoader[Seq[String]] = ConfigLoader(_.getStringList).map(_.asScala.toSeq)

  implicit val intLoader: ConfigLoader[Int] = ConfigLoader(_.getInt)
  implicit val seqIntLoader: ConfigLoader[Seq[Int]] = ConfigLoader(_.getIntList).map(_.asScala.map(_.toInt).toSeq)

  implicit val booleanLoader: ConfigLoader[Boolean] = ConfigLoader(_.getBoolean)
  implicit val seqBooleanLoader: ConfigLoader[Seq[Boolean]] =
    ConfigLoader(_.getBooleanList).map(_.asScala.map(_.booleanValue).toSeq)

  implicit val finiteDurationLoader: ConfigLoader[FiniteDuration] =
    ConfigLoader(_.getDuration).map(javaDurationToScala)

  implicit val seqFiniteDurationLoader: ConfigLoader[Seq[FiniteDuration]] =
    ConfigLoader(_.getDurationList).map(_.asScala.map(javaDurationToScala).toSeq)

  implicit val durationLoader: ConfigLoader[Duration] = ConfigLoader { config =>
    path =>
      if (config.getIsNull(path)) Duration.Inf
      else if (config.getString(path) == "infinite") Duration.Inf
      else finiteDurationLoader.load(config, path)
  }

  // Note: this does not support null values but it added for convenience
  implicit val seqDurationLoader: ConfigLoader[Seq[Duration]] =
    seqFiniteDurationLoader.map(identity[Seq[Duration]])

  implicit val periodLoader: ConfigLoader[Period] = ConfigLoader(_.getPeriod)

  implicit val temporalLoader: ConfigLoader[TemporalAmount] = ConfigLoader(_.getTemporal)

  implicit val doubleLoader: ConfigLoader[Double] = ConfigLoader(_.getDouble)
  implicit val seqDoubleLoader: ConfigLoader[Seq[Double]] =
    ConfigLoader(_.getDoubleList).map(_.asScala.map(_.doubleValue).toSeq)

  implicit val numberLoader: ConfigLoader[Number] = ConfigLoader(_.getNumber)
  implicit val seqNumberLoader: ConfigLoader[Seq[Number]] = ConfigLoader(_.getNumberList).map(_.asScala.toSeq)

  implicit val longLoader: ConfigLoader[Long] = ConfigLoader(_.getLong)
  implicit val seqLongLoader: ConfigLoader[Seq[Long]] =
    ConfigLoader(_.getLongList).map(_.asScala.map(_.longValue).toSeq)

  implicit val bytesLoader: ConfigLoader[ConfigMemorySize] = ConfigLoader(_.getMemorySize)
  implicit val seqBytesLoader: ConfigLoader[Seq[ConfigMemorySize]] =
    ConfigLoader(_.getMemorySizeList).map(_.asScala.toSeq)

  implicit val configLoader: ConfigLoader[Config] = ConfigLoader(_.getConfig)
  implicit val configListLoader: ConfigLoader[ConfigList] = ConfigLoader(_.getList)
  implicit val configObjectLoader: ConfigLoader[ConfigObject] = ConfigLoader(_.getObject)
  implicit val seqConfigLoader: ConfigLoader[Seq[Config]] = ConfigLoader(_.getConfigList).map(_.asScala.toSeq)

  //implicit val configurationLoader: ConfigLoader[Configuration] = configLoader.map(Configuration(_))
  //implicit val seqConfigurationLoader: ConfigLoader[Seq[Configuration]] = seqConfigLoader.map(_.map(Configuration(_)))

  implicit val urlLoader: ConfigLoader[URL] = ConfigLoader(_.getString).map(new URL(_))
  implicit val uriLoader: ConfigLoader[URI] = ConfigLoader(_.getString).map(new URI(_))

  private def javaDurationToScala(javaDuration: java.time.Duration): FiniteDuration =
    Duration.fromNanos(javaDuration.toNanos)

}

extension (underlying: Config) {
  def getOptional[A](path: String)(using loader: ConfigLoader[A]): Option[A] = {
    if (underlying.hasPath(path)) Some(get[A](path)) else None
  }
  def get[A](path: String)(using loader: ConfigLoader[A]): A = {
    loader.load(underlying, path)
  }
}

