package very.util.entity

case class Pagination(page: Int, pageSize: Int) {
  assert(pageSize <= 50)
  assert(page > 0)
  def offset: Int = (page - 1) * pageSize
  def limit: Int = pageSize

}
