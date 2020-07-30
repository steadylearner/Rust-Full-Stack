import React, {Component} from 'react';
import {render} from 'react-dom';
import io from 'socket.io-client';
import Row from './Row';

class FXComponent extends Component<{}, FXComponentState> {
  constructor(props:any) {
    super(props);
    this.state = { fxRates: [] };
  }

  componentDidMount() {
    io().on("data", (data:Array<FXRow>) => this.setState({ fxRates: data }));
  }

  render() {
    return (
      <table>
        <tbody>
          {this.state.fxRates.map((rate: FXRow) => (
            <Row key={rate.currencyPair} data={rate}></Row>)
          )}
        </tbody>
      </table>);
  }
}

render(
  <FXComponent></FXComponent>,
  document.getElementById("root")
);