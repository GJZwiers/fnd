import React from "react";
import { Input } from "@nextui-org/react";

class TextInput extends React.Component {
  private textInput;
  constructor(props) {
    super(props);
    this.textInput = React.createRef();
    this.focusTextInput = this.focusTextInput.bind(this);
  }

  focusTextInput() {
    this.textInput.current.focus();
  }

  render() {
    return (
      <div>
        <Input
          aria-label="Default msg"
          ref={this.textInput}
          onClick={this.focusTextInput}
          type="text"
          size="xs"
        />
      </div>
    );
  }
}
export default TextInput;
