import { User } from "./user.model"

export type AuthModel = {
  username: string,
  password: string,
}

export type AuthResponse = {
  accessToken: string,
  schema: string, // It's always "Bearer" anyway,
  user: User,
  // refreshToken: string, // For future implementation!
}
