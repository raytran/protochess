import React, { Component } from "react";
import {FormControl, Button, InputGroup}  from 'react-bootstrap';
import CopyToClipboard from 'react-copy-to-clipboard';
 
interface MyProps {
    value: string;
}
class ReadOnlyCopyTextBox extends Component<MyProps> {
  constructor(props:MyProps) {
      super(props);
  }


  render() {
    return (
      <div>
      <InputGroup className="mb-3">
        <FormControl
           readOnly
           value={this.props.value}
           aria-describedby="basic-addon2"
         />
      </InputGroup>
        <CopyToClipboard text={this.props.value}
          onCopy={() => this.setState({})}>
          <button>Copy to clipboard</button>
        </CopyToClipboard>
      </div>
    )
  }
}
 
export default ReadOnlyCopyTextBox;
