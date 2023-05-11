package very.util.web

import jakarta.servlet.http.{HttpServletRequest, HttpServletResponse}
import org.eclipse.jetty.server.ResourceService

class SinglePageAppResourceService extends ResourceService {
  override def notFound(
    request: HttpServletRequest,
    response: HttpServletResponse
  ) = {
    if (request.getRequestURI.contains(".") && request.getMethod != "GET") {
      super.notFound(request, response)
    } else {
      response.sendRedirect("/")
    }
  }
}
