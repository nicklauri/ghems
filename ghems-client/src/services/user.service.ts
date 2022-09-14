import { ApiEndpoints } from "../constants/endpoints.constants"
import { ApiResponse } from "../models/api.model"
import { AuthModel, AuthResponse } from "../models/auth.model"
import { User } from "../models/user.model"
import { HttpClient } from "./http-client.service"
import { LocalStorageService } from "./local-storage.service"

const login = async (data: AuthModel): Promise<ApiResponse<AuthResponse>> => {
  return HttpClient.post(ApiEndpoints.user.login, data)
}

const getUserInfo = async (): Promise<ApiResponse<User>> => {
  return HttpClient.get(ApiEndpoints.user.info)
}

const hasToken = (): boolean => !!LocalStorageService.getItem(LocalStorageService.Key.AuthToken)

export const UserService = {
  login,
  getUserInfo,
  hasToken,
}
