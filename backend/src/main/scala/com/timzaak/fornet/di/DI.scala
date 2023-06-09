package com.timzaak.fornet.di

import com.timzaak.fornet.config.{AppConfig, AppConfigImpl}
import com.timzaak.fornet.controller.*
import com.timzaak.fornet.grpc.AuthGRPCController
import com.timzaak.fornet.keycloak.{KeycloakJWTSaaSAuthStrategy, KeycloakJWTSaaSCompatAuthStrategy}
import com.timzaak.fornet.mqtt.MqttCallbackController
import com.timzaak.fornet.mqtt.api.RMqttApiClient
import com.timzaak.fornet.pubsub.{MqttConnectionManager, NodeChangeNotifyService}
import com.timzaak.fornet.service.*
import very.util.keycloak.{JWKPublicKeyLocator, JWKTokenVerifier}
import very.util.web.auth.{AuthStrategy, AuthStrategyProvider, SingleUserAuthStrategy}
object DI extends DaoDI { di =>

  object appConfig extends AppConfigImpl(config)

  // connection Manager
  // object connectionManager extends ConnectionManager
  object connectionManager
    extends MqttConnectionManager(
      mqttApiClient = di.mqttApiClient
    )

  object nodeService
    extends NodeService(
      nodeDao = di.nodeDao
    )

  object nodeChangeNotifyService
    extends NodeChangeNotifyService(
      nodeDao = di.nodeDao,
      networkDao = di.networkDao,
      deviceDao = di.deviceDao,
      connectionManager = di.connectionManager,
      nodeService = di.nodeService,
    )

  // web controller auth

  import very.util.config.*

  given authStrategyProvider: AuthStrategyProvider[String] =
    AuthStrategyProvider(
      if (config.hasPath("auth.keycloak")) {
        // init keycloak,( keycloak server must start, this would get information from keycloak server)
        val keycloakUrl = config.get[String]("auth.keycloak.authServerUrl")
        val realm = config.get[String]("auth.keycloak.realm")
        val publicKeyLocator = JWKPublicKeyLocator
          .init(
            keycloakUrl,
            realm,
          )
          .get
        val verifier = JWKTokenVerifier(publicKeyLocator, keycloakUrl, realm)
        if (appConfig.enableSAAS) {
          List(KeycloakJWTSaaSAuthStrategy(verifier))
        } else {
          List(
            KeycloakJWTSaaSCompatAuthStrategy(
              verifier,
              config.getOptional[String]("auth.keycloak.adminRole"),
              config.getOptional[String]("auth.keycloak.clientRole"),
            )
          )
        }
      } else {
        List(
          SingleUserAuthStrategy(
            config.get[String]("auth.simple.token"),
            config.get[String]("auth.simple.userId")
          )
        )
      }
    )
  // web controller
  object appInfoController extends AppInfoController(appConfig = di.appConfig)
  object networkController
    extends NetworkController(
      networkDao = di.networkDao,
      appConfig = di.appConfig,
      nodeChangeNotifyService = di.nodeChangeNotifyService,
    )

  object nodeController
    extends NodeController(
      nodeDao = di.nodeDao,
      networkDao = di.networkDao,
      deviceDao = di.deviceDao,
      nodeChangeNotifyService = di.nodeChangeNotifyService,
      appConfig = di.appConfig,
    )

  object authController
    extends AuthController(
      networkDao = di.networkDao,
      appConfig = di.appConfig,
    )

  object nodeAuthService extends NodeAuthService

  object authGRPCController
    extends AuthGRPCController(
      nodeDao = di.nodeDao,
      networkDao = di.networkDao,
      deviceDao = di.deviceDao,
      nodeChangeNotifyService = di.nodeChangeNotifyService,
      config = di.config,
      appConfig = di.appConfig,
      nodeAuthService = di.nodeAuthService,
      authStrategyProvider = di.authStrategyProvider,
    )

  object mqttApiClient extends RMqttApiClient(config.get[String]("mqtt.apiUrl"))
  object mqttCallbackController
    extends MqttCallbackController(
      nodeDao = di.nodeDao,
      networkDao = di.networkDao,
      deviceDao = di.deviceDao,
      nodeService = di.nodeService,
      mqttConnectionManager = di.connectionManager
    )

}
