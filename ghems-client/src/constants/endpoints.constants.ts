import { ApplicationSettings } from "../environments/environment";

const API_ROOT = ApplicationSettings.apiUrl;

export const ApiEndpoints = {
  root: API_ROOT,
  user: {
    get parent() { return ApiEndpoints },
    get root() { return `${this.parent.root}/user` },
    get login() { return `${this.root}/login` },
    get info() { return `${this.root}/info` }
  }
}
