import { Link, Outlet } from "react-router-dom";
import "./LoginLayout.scss";

export const LoginLayout = () => {
  return <div className="login-page-layout">
    <Outlet />
  </div>
}
