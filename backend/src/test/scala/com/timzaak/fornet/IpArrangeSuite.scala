package com.timzaak.fornet

//import com.google.common.net.InetAddresses

import inet.ipaddr.IPAddress.IPVersion
import inet.ipaddr.IPAddressString
import inet.ipaddr.ipv4.IPv4Address
import munit.FunSuite
import org.eclipse.jetty.util.InetAddressSet

class IpArrangeSuite extends FunSuite {
  test("ipRangePass") {
    val ipV4Range = "10.0.0.1/8"
    // InetAddresses.forString("")
    val a: IPv4Address = IPAddressString(ipV4Range)
      .toAddress(IPVersion.IPV4)
      .asInstanceOf[IPv4Address]
    println(a)

    println(
      s"${a.isSingleNetwork}, ${a.isSinglePrefixBlock}, ${a.getNetworkPrefixLength}"
    )
  }
  test("ipv4") {
    val ipV4 = "10.0.0.1"
    val a: IPv4Address =
      IPAddressString(ipV4).toAddress(IPVersion.IPV4).asInstanceOf[IPv4Address]
    println(ipV4)
    println(
      s"${a.isSingleNetwork}, ${a.isSinglePrefixBlock}, ${a.getNetworkPrefixLength}"
    )
  }

  test("generate ip") {
    val usedIPs = Seq("10.0.0.1")
      .map(ip =>
        IPAddressString(ip)
          .toAddress(IPVersion.IPV4)
          .asInstanceOf[IPv4Address]
          .intValue()
      )
      .toSet
    val ip = IPAddressString("10.0.0.1/24")
      .toAddress(IPVersion.IPV4)
      .toPrefixBlock()
      .asInstanceOf[IPv4Address]
    (ip.getLowerNonZeroHost.intValue() to ip.getUpper.intValue())
      .find(!usedIPs.contains(_))
      .foreach(v => println(IPv4Address(v)))

    println(ip)
  }

  test("ip contains") {
    val range = IPAddressString("10.0.0.1/24")
    val single = IPAddressString("10.0.0.2")
    println(range.prefixContains(single))
    println(range.contains(single))
  }

}
