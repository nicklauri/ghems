import { CSSProperties, ForwardedRef, forwardRef, HTMLAttributes, ReactNode, useImperativeHandle, useRef } from "react"
import { ret } from "../../utils/common"
import { StringUtils } from "../../utils/string-utils"
import { Icons } from "../Icons/Icons"
import "./Button.scss"

export enum ButtonType {
  Primary = "primary",
  Secondary = "secondary",
  Normal = "normal",
}

export type ButtonProps = {
  className?: string,
  type?: ButtonType,
  decorated?: boolean,
  disabled?: boolean,
  width?: string,
  loading?: boolean,
  text?: string,
  submit?: boolean,

  // TODO:
  onRenderPrefix?: () => ReactNode,
  onRenderSuffix?: () => ReactNode,

  onClick?: () => void,

  buttonProps?: HTMLAttributes<HTMLButtonElement>,
  buttonStyles?: CSSProperties,
  styles?: CSSProperties,
  children?: ReactNode,
}

export const PrimaryButton = (props: ButtonProps) => <GhemButton {...props} type={ButtonType.Primary} />
export const SecondaryButton = (props: ButtonProps) => <GhemButton {...props} type={ButtonType.Secondary} />
export const Button = (props: ButtonProps) => <GhemButton {...props} type={ButtonType.Normal} />

export const SubmitButton = (props: ButtonProps) =>
  <GhemButton {...props}
    type={ButtonType.Primary}
    buttonProps={{ type: "submit" } as ButtonProps["buttonProps"]}
  />

export const GhemButton = forwardRef((props: ButtonProps, ref: ForwardedRef<HTMLButtonElement>) => {
  const buttonTypeClass = props.type as string
  const decoratedClass = ret(props.decorated, "decorated")
  const disabledClass = ret(props.disabled, "disabled")
  const className = StringUtils.join("ghems-button",
    props.className,
    buttonTypeClass,
    decoratedClass,
    disabledClass,
  )

  const innerRef = useRef(null)

  useImperativeHandle(ref, () => innerRef.current!)

  const buttonStyles = { ...props.buttonStyles } ?? {}
  if (props.width) {
    buttonStyles.width = props.width
  }

  return <div
    className={className}
    style={props.styles}
  >
    <button {...props.buttonProps}
      ref={ref}
      className={undefined}
      style={props.buttonStyles}
      onClick={props.onClick}
    >
      {props.loading &&
        <></>
      }
      {props.text ? props.text : props.children}
    </button>
  </div>
})
