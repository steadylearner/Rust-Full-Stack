// FunctionalComponent example

import * as React from 'react';

interface Props {
    count: number;
}

const Count: React.FunctionComponent<Props> = (props) => {
    return <h1>{props.count}</h1>;
};

export default Count;