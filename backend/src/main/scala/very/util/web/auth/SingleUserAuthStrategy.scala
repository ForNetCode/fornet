package very.util.web.auth
import io.getquill.ast.CaseClass.Single.apply

//This is the easy way to auth
class SingleUserAuthStrategy[User](selfDefinedToken: String, user: User)
  extends AuthStrategy[User] {
  override def name: String = SingleUserAuthStrategy.name

  override def adminAuth(
    token: String
  ): Option[User] = {
    if (token == selfDefinedToken) {
      Some(user)
    } else { None }
  }

  //this would never Use
  override def clientAuth(token: String): Option[User] = Some(user)
}
object SingleUserAuthStrategy {
  val name:String = "ST"
}
