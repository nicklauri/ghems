import { Control, Controller, FieldValues } from "react-hook-form";
import { TextField, TextFieldProps } from "./TextField";

export type ControlledTextFieldProps = TextFieldProps & {
  controlProps: {
    name: string,
    control?: Control<FieldValues, any>,
  },
}

export const ControlledTextField = (props: ControlledTextFieldProps) =>
  <Controller
    {...props.controlProps}
    render={({
      field: { onChange, onBlur, value, name, ref },
      fieldState: { isTouched, isDirty, error },
      formState,
    }) => (
      <TextField
        onBlur={onBlur}
        onChange={onChange}
        inputRef={ref}
        value={value}
        errorMessage={error?.message}
        {...props}
      />
    )}
  />
