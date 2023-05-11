package com.timzaak.fornet

import com.timzaak.fornet.di.DI
import com.timzaak.fornet.protobuf.auth.AuthGrpc
import com.typesafe.config.Config
import io.grpc.netty.NettyServerBuilder
import jakarta.servlet.DispatcherType
import org.eclipse.jetty.server.{Server, ServerConnector}
import org.eclipse.jetty.servlet.{DefaultServlet, ServletHolder}
import org.eclipse.jetty.util.ssl.SslContextFactory
import org.eclipse.jetty.webapp.WebAppContext
import org.scalatra.servlet.ScalatraListener
import very.util.config.*
import very.util.persistence.pgMigrate
import very.util.web.SinglePageAppResourceService

import java.io.File
import java.util
import java.util.concurrent.TimeUnit
import scala.concurrent.ExecutionContext

@main def serverRun: Unit = {
  pgMigrate(DI.config.getConfig("database.dataSource"))
  val gServer = grpcServer(DI.config)

  webServer(DI.config)
  sys.addShutdownHook {
    gServer.shutdown()
  }
  gServer.awaitTermination()
}

def grpcServer(config: Config) = {
  val builder = NettyServerBuilder
    .forPort(config.getOptional[Int]("server.grpc.port").getOrElse(9000))
    .addService(
      AuthGrpc.bindService(DI.authGRPCController, ExecutionContext.global)
    )
    // .intercept(DI.authServiceInterceptor)
    .keepAliveTime(500, TimeUnit.SECONDS)

  config.getOptional[String]("server.grpc.ssl.certChain") zip config
    .getOptional[String]("server.grpc.ssl.privateKey") match {
    case Some(certChain, privateKey) =>
      builder.useTransportSecurity(File(certChain), File(privateKey))
    case _ =>
  }

  builder.build().start()
}

def webServer(config: Config) = {
  val port: Int = config.getOptional[Int]("server.web.port").getOrElse(8080)

  val server = Server(port)

  // TODO: add tls/ssl
  // https://www.ibm.com/docs/pt/rational-change/5.3.0?topic=813-configuring-jetty-run-in-httpsssl-mode
  // val ssl = SslContextFactory.Server()
  // ssl.setKeyStorePath("")
  // ssl.setKeyStorePassword()
  // val connector = ServerConnector(server,ssl)

  val context = WebAppContext()
  // context setContextPath "/"

  context.addEventListener(new ScalatraListener)

  config.getOptional[String]("server.web.staticWeb") match {
    case Some(staticWebPath) =>
      context.setResourceBase(staticWebPath)
      val holder = ServletHolder(DefaultServlet(SinglePageAppResourceService()))
      context.addServlet(holder, "/")
    // context.addServlet(classOf[DefaultServlet], "/")
    case _ => // do nothing
  }

  context.setInitParameter(
    ScalatraListener.LifeCycleKey,
    "com.timzaak.fornet.ScalatraBootstrap"
  )
  // val keyCloakFilter = KeycloakOIDCFilter()

  // context.addFilter(classOf[org.keycloak])
  server.setHandler(context)
  server.setStopAtShutdown(true)
  server.start()
  server
}
