import * as React from "react";

const code = "https://www.steadylearner.com/static/images/brand/code.png"

export default class FirstComponent extends React.Component<{}> {

  render() {
    return (
      <div>
        <h1>A Simple React Component Example with Typescript</h1>
        <div>
          <img src={code} />
        </div>
      </div>
    );
  }
}