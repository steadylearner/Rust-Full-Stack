/* eslint-disable */

import React from 'react';
import { shallow } from 'enzyme';

class CheckboxWithLabel extends React.Component {
    state = {
        isChecked: false,
    };

    onChange = () => {
        this.setState({ isChecked: !this.state.isChecked });
    };

    render() {
        return (
            <label>
                <input
                    type="checkbox"
                    checked={this.state.isChecked}
                    onChange={this.onChange}
                />
                {this.state.isChecked ? this.props.labelOn : this.props.labelOff}
            </label>
        );
    }
}

describe('Unit test example with React and Enzyme.', () => {
    it('CheckboxWithLabel changes the text after click', () => {
        // Render a checkbox with label in the document
        const checkbox = shallow(<CheckboxWithLabel labelOn="On" labelOff="Off" />);

        expect(checkbox.text()).toEqual('Off');

        checkbox.find('input').simulate('change');

        expect(checkbox.text()).toEqual('On');
    });
});
