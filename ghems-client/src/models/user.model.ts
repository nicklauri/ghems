
export type User = UserNoPicture & {
  picture: string,
}

export type UserNoPicture = {
  id: string,
  username: string,
  email: string,
  firstName: string,
  lastName: string,
  bio: string,
}

export const EmptyUser: User = {
  id: "",
  username: "",
  email: "",
  firstName: "",
  lastName: "",
  bio: "",
  picture: "",
}
