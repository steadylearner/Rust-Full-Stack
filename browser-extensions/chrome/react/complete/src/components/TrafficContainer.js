import React, { Component } from 'react';

export default class TrafficContainer extends Component {

    static renderNetworkTrafficData(requests) {
        if (requests) {
            return Object.keys(requests).map((key) => {
                const { url, requestDuration, status } = requests[key];
                return (<li>{`url ${url} took ${requestDuration}ms with status ${status}`}</li>);
            });
        }
        return '';
    }

    render() {
        return (
            <ul>
                {TrafficContainer.renderNetworkTrafficData(this.props.traffic.requests)}
            </ul>
        );
    }
}