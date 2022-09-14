import { AxiosResponse } from "axios";
import { AuthModel, AuthResponse } from "../../models/auth.model";
import { UserService, LocalStorageService } from "../../services";
import { AppService } from "../../services/app.service";

const saveLoginInfo = (info: AuthResponse) => {
  AppService.login(info.user)
  LocalStorageService.setAuthToken(info.accessToken)
}

const login = async (info: AuthModel,
  onsuccess: (response: AuthResponse) => void,
  onerror: (reason: any) => void,
  onfinally?: VoidFunction,
) => {
  try {
    const result = await UserService.login(info)

    if (!result.isSuccess) {
      onerror(result.message)
      return
    }

    saveLoginInfo(result.data)

    onsuccess(result.data)
  }
  catch (error) {
    const response = (error as any).response
    onerror(response.data.message)
  }
  finally {
    onfinally?.()
  }
}

export const LoginHandlers = {
  login,
  saveLoginInfo,
}
