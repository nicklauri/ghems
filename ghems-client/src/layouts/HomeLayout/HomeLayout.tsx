import { Outlet } from "react-router-dom"
import { Nav } from "../../components/Nav/Nav"
import "./HomeLayout.scss"

export const HomeLayout = () => {
  return <div className="home-page-layout">
    <div className="container">
      <div className="header">
        <Nav />
      </div>
      <div className="content">
        <Outlet />
      </div>
    </div>
  </div>
}
