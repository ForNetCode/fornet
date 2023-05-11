package very.util.web.auth

import jakarta.servlet.http.HttpServletRequest

trait AuthStrategy[User] {
  def name:String
  def auth(token:String): Option[User]
}
