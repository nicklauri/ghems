import { Button, PrimaryButton, SecondaryButton } from "../../components/Button/Button"
import { TextField } from "../../components/TextField/TextField"
import "./Controls.scss"

export const Controls = () => {
  return <div className="controls-page">
    <div className="area">
      <TextField label="Label" placeholder="Placeholder" />
    </div>
    <div className="area">
      <TextField prefixText="Prefix text" prefixSeparator suffixSeparator suffixText="Suffix text" />
    </div>
    <div className="area row vertical">
      <PrimaryButton text="Primary button" />
      <SecondaryButton text="Secondary button" />
      <Button text="Normal button" />
    </div>
  </div>
}