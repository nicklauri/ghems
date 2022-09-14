import { useCallback } from 'react'
import { ChangeEventHandler, CSSProperties, FocusEventHandler, ForwardedRef, forwardRef, HTMLAttributes, HTMLInputTypeAttribute, InputHTMLAttributes, ReactNode, useImperativeHandle, useRef } from 'react'
import { useBoolean } from '../../hooks/useBoolean'
import { ret } from '../../utils/common'
import { ReactUtils } from '../../utils/react'
import { StringUtils } from '../../utils/string-utils'
import "./TextField.scss"

export type TextFieldProps = {
  id?: string,
  name?: string,
  type?: HTMLInputTypeAttribute,
  className?: string,
  label?: string,
  required?: boolean,
  readOnly?: boolean,
  autoComplete?: string,
  autoFocus?: boolean,
  maxLength?: number,
  placeholder?: string,

  inputRef?: any,

  norounded?: boolean,
  errorMessage?: string,
  hasErrorPlaceholder?: boolean,
  autoTrim?: boolean,

  value?: string,
  defaultValue?: string,
  onChange?: (value: string) => void,
  onFocus?: VoidFunction,
  onBlur?: VoidFunction,

  prefixText?: string,
  suffixText?: string,
  prefixStyles?: CSSProperties,
  suffixStyles?: CSSProperties,
  prefixSeparator?: boolean,
  suffixSeparator?: boolean,
  onRenderPrefix?: () => ReactNode,
  onRenderSuffix?: () => ReactNode,

  nameAsId?: string,

  labelProps?: HTMLAttributes<HTMLLabelElement>,
  inputProps?: InputHTMLAttributes<HTMLInputElement>,
  labelStyles?: CSSProperties,
  inputStyles?: CSSProperties,
  styles?: CSSProperties,
}

export const TextField = forwardRef((props: TextFieldProps, ref: ForwardedRef<HTMLInputElement | undefined>) => {
  const inputRef = useRef<HTMLInputElement>(null)
  const [isActive, { setTrue: setActive, setFalse: setInactive }] = useBoolean(false)

  useImperativeHandle(ref, () => inputRef.current || undefined)

  ReactUtils.assignRef(inputRef.current, props.inputRef)

  const activeClass = ret(isActive, "active")
  const requiredClass = ret(props.required, "required")
  const readOnlyClass = ret(props.readOnly, "readonly")
  const invalidClass = ret(props.errorMessage, "invalid")
  const noroundedClass = ret(props.norounded, "no-rounded")
  const hasErrorPlaceholderClass = ret(props.hasErrorPlaceholder, "has-error-placeholder")

  const className = StringUtils.join("ghems-text-field",
    props.className,
    activeClass,
    requiredClass,
    readOnlyClass,
    invalidClass,
    noroundedClass,
    hasErrorPlaceholderClass,
  )
  const labelClassName = "text-field-label"
  const inputClassName = "text-field-input"

  const prefixClassName = StringUtils.join("text-field-prefix",
    ret(props.prefixText, "text"),
    ret(props.prefixSeparator, "has-separator"),
  )

  const suffixClassName = StringUtils.join("text-field-suffix",
    ret(props.suffixText, "text"),
    ret(props.suffixSeparator, "has-separator"),
  )

  const name = !props.name && props.nameAsId ? props.id : props.name

  useImperativeHandle(ref, () => inputRef.current!)

  const onFocus: FocusEventHandler<HTMLInputElement> = useCallback((_ev) => {
    setActive()
    return props.onFocus?.()
  }, [props.onFocus])

  const onBlur: FocusEventHandler<HTMLInputElement> = useCallback((ev) => {
    setInactive()

    if (props.autoTrim && props.onChange) {
      const trimmed = ev.target.value.trim()

      if (trimmed.length !== ev.target.value.length) {
        props.onChange(trimmed)
      }
    }

    return props.onBlur?.()
  }, [props.onBlur, props.autoTrim, props.onChange])

  const onChange: ChangeEventHandler<HTMLInputElement> = useCallback((ev) => {
    props.onChange?.(ev.target.value)
  }, [props.onChange])

  const onContainerClick = useCallback(() => {
    inputRef.current?.focus()
  }, [inputRef.current])

  return <div className={className} onClick={onContainerClick}>
    {props.label &&
      <label
        {...props.labelProps}
        className={labelClassName}
        htmlFor={props.id}
        style={props.labelStyles}
      >
        {props.label}
      </label>
    }
    <div className="text-field-container" style={props.styles}>
      {(props.prefixText || props.onRenderPrefix) &&
        <div className={prefixClassName} style={props.prefixStyles}>
          {props.prefixText
            ? props.prefixText
            : props.onRenderPrefix?.()
          }
        </div>
      }
      <input
        {...props.inputProps}
        ref={inputRef}
        id={props.id}
        type={props.type}
        className={inputClassName}
        name={name}
        style={props.inputStyles}
        required={props.required}
        readOnly={props.readOnly}
        autoComplete={props.autoComplete}
        autoFocus={props.autoFocus}
        maxLength={props.maxLength}
        placeholder={props.placeholder}
        onChange={onChange}
        onFocus={onFocus}
        onBlur={onBlur}
        defaultValue={props.defaultValue}
        value={props.value}
      />

      {(props.suffixText || props.onRenderSuffix) &&
        <div className={suffixClassName} style={props.suffixStyles}>
          {props.suffixText
            ? props.suffixText
            : props.onRenderSuffix?.()
          }
        </div>
      }
    </div>

    {(props.errorMessage || props.hasErrorPlaceholder) &&
      <div className="text-field-error text-danger">
        {props.errorMessage}
      </div>
    }
  </div>
})
