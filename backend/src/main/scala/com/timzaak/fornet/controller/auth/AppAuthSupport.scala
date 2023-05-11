package com.timzaak.fornet.controller.auth

import very.util.web.auth.AuthSupport
import very.util.web.auth.AuthStrategyProvider

type User = String
type AppAuthSupport = AuthSupport[User]
type AppAuthStrategyProvider = AuthStrategyProvider[User]
