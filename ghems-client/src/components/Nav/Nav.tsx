
import { useMemo } from "react"
import { Link } from "react-router-dom"
import { RouteConstants, RouteLinkList } from "../../constants/routes.constants"
import "./Nav.scss"

export const Nav = () => {
  const linkItems = useMemo(() => {
    return RouteLinkList.map((routeItem, idx) =>
      <Link key={idx} to={routeItem.route} className="menu-item">
        {routeItem.name}
      </Link>
    )
  }, [])

  return <div className="ghems-nav">
    <div className="container">
      <div className="logo">
        <span className="text-color-animation">Ghems</span>
      </div>
      <div className="menu">
        {linkItems}
      </div>
    </div>
  </div>
}
