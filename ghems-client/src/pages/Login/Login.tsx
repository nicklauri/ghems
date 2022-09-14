import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react"
import { Controller } from "react-hook-form"
import { useNavigate } from "react-router-dom"
import { BubblesBackground } from "../../components/BubblesBackground/BubblesBackground"
import { PrimaryButton, SubmitButton } from "../../components/Button/Button"
import { TextField } from "../../components/TextField/TextField"
import { RouteConstants } from "../../constants/routes.constants"
import { useBoolean } from "../../hooks/useBoolean"
import { CommonUtils } from "../../utils/common"
import { LoginHandlers } from "./Login.handlers"
import "./Login.scss"

const labelStyle = {
  width: 72,
}

export const LoginPage = () => {
  useLayoutEffect(() => {
    CommonUtils.setPageTitle("Login to Ghems")
  }, [])

  const navigate = useNavigate()
  const [errorMessage, setErrorMessage] = useState<string>()
  const [isLoading, { setTrue: setLoading, setFalse: setLoaded }] = useBoolean(false)
  const [isTouchedUsername, { setTrue: setTouchedUsername }] = useBoolean(false)
  const [isTouchedPassword, { setTrue: setTouchedPassword }] = useBoolean(false)
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")

  useEffect(() => {
    if ((isTouchedUsername && !username) || (isTouchedPassword && !password)) {
      setErrorMessage("Username and password is required")
    }
    else {
      setErrorMessage(undefined)
    }
  }, [isTouchedUsername, username, password])

  const login = useCallback(() => {
    if (errorMessage) {
      return
    }

    setLoading()
    LoginHandlers.login({ username, password },
      () => {
        console.log(`login successfully! redirecting`)
        navigate(RouteConstants.Root)
      },
      (data) => {
        setErrorMessage(`Login failed`)
      },
      setLoaded
    )
  }, [username, password])

  return <>
    <div className="login-page">
      <BubblesBackground />
      <div className="login-form">
        <div className="row row-title">
          <div className="title">
            <span className="text-color-animation">Ghems </span>
            login
          </div>
        </div>
        <div className="row">
          <TextField
            prefixText="Username"
            prefixSeparator
            id="username"
            autoComplete="off"
            autoFocus
            autoTrim
            readOnly={isLoading}
            prefixStyles={labelStyle}
            onChange={setUsername}
            onBlur={setTouchedUsername}
          />
        </div>
        <div className="row">
          <TextField
            prefixText="Password"
            prefixSeparator
            type="password"
            id="password"
            autoComplete="off"
            autoTrim
            readOnly={isLoading}
            prefixStyles={labelStyle}
            onChange={setPassword}
            onBlur={setTouchedPassword}
          />
        </div>
        <div className="row button">
          <PrimaryButton
            onClick={login}
            buttonStyles={{
              fontFamily: "Montserrat",
              fontWeight: "600",
            }}
          >
            {isLoading
              ? "Logging in..."
              : "Login"
            }
          </PrimaryButton>
        </div>
        <div className="row">
          <div className="error-message text-danger">
            {errorMessage || <>&nbsp;</>}
          </div>
        </div>
      </div>
    </div>
  </>
}
