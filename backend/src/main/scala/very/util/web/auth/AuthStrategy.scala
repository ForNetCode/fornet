package very.util.web.auth

import jakarta.servlet.http.HttpServletRequest

trait AuthStrategy[User] {
  def name:String
  def adminAuth(token:String): Option[User]
  def clientAuth(token:String): Option[User]
}
