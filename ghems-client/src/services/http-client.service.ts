import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from "axios"
import { ApiEndpoints } from "../constants/endpoints.constants"
import { ApiResponse } from "../models/api.model"
import { LocalStorageService } from "./local-storage.service"

export type RequestConfig = {
  quiet?: boolean,
  axios?: AxiosRequestConfig,
}

const createAxiosInstance = (defaultAxiosConfig: AxiosRequestConfig = {}): AxiosInstance => {
  const config = {
    baseURL: ApiEndpoints.root,
    headers: {},
  } as AxiosRequestConfig

  const token = LocalStorageService.getAuthToken()
  if (token) {
    config.headers!["Authorization"] = `Bearer ${token}`
  }

  return axios.create(config)
}

type Response<T, D = any> = AxiosResponse<ApiResponse<T>, D>

const handleError = (reject: (reason?: any) => void) => (error: any) => {
  // TODO: may display error message box or sumthing.
  reject(error)
}

const handleResponse = <T, D = any>(resolve: (value: ApiResponse<T> | PromiseLike<ApiResponse<T>>) => void) => (response: Response<T, D>) => {
  resolve(response.data)
}

const handleRequest = <T, D>(request: Promise<Response<T, D>>, onfinally?: VoidFunction): Promise<ApiResponse<T>> => {
  return new Promise((resolve, reject) =>
    request
      .then(handleResponse<T, D>(resolve))
      .catch(handleError(reject))
      .finally(onfinally)
  )
}

const get = <R>(url: string, params?: any, config: RequestConfig = {}): Promise<ApiResponse<R>> => {
  const axiosConfig = {
    ...config.axios,
    params,
  }

  return handleRequest(createAxiosInstance().get(url, axiosConfig))
}

const post = async <T, R>(url: string, data: T, config: RequestConfig = {}): Promise<ApiResponse<R>> => {
  const axiosConfig = {
    ...config.axios,
    data,
  }
  return handleRequest(createAxiosInstance().post(url, data, axiosConfig))
}

export const HttpClient = {
  get,
  post,
}
