
export type ApiResponse<T> = {
  isSuccess: boolean,
  message: string | null,
  data: T,
}
