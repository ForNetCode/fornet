package very.util.web.auth

class AuthStrategyProvider[User](strategies: List[AuthStrategy[User]]) {
  private val strategyMap = strategies.map(v => v.name -> v).toMap

  def getStrategy(strategyName: String): Option[AuthStrategy[User]] = {
    strategyMap.get(strategyName)
  }

}
