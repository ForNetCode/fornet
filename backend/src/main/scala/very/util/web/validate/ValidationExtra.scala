package very.util.web.validate

import zio.prelude.Validation
import inet.ipaddr.IPAddress.IPVersion
import inet.ipaddr.IPAddressString
import inet.ipaddr.ipv4.IPv4Address

import java.text.MessageFormat
import scala.util.{Success, Try}

trait ValidationExtra {this: org.scalatra.i18n.I18nSupport & org.scalatra.ScalatraBase =>

  def i18n(key:String)(arguments:AnyRef*): String = {
    MessageFormat.format(messages(key), arguments:_*)
  }


  def ipV4Range(n:String) = Validation.fromEither {
    Try(IPAddressString(n).toAddress(IPVersion.IPV4).asInstanceOf[IPv4Address]) match {
      case Success(address) if address.isPrivate && address.getPrefixLength != null  =>
        Right(address)
      case _ => Left(i18n("error.ipv4_address_range")(n))
    }
  }

  def privateIpValid(n:String) = Validation.fromEither {
    Try(IPAddressString(n).toAddress(IPVersion.IPV4).asInstanceOf[IPv4Address]) match {
      case Success(address) if address.isPrivate && address.getPrefixLength == null =>
        Right(address)
      case _ => Left(i18n("error.ipv4_address")(n))
    }
  }

}