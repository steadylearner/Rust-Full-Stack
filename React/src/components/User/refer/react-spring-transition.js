import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import { animated, useTransition } from 'react-spring';
import './styles.css';

function Test() {
    const [show, set] = useState(true)
    const transitions = useTransition(show, null, {
        from: { position: 'absolute', opacity: 0 },
        enter: { opacity: 1 },
        leave: { opacity: 0 },
    })

    return (transitions.map(({ item, key, props }) => item && <animated.div onClick={() => set(!show)} key={key} style={props}>
        transition
  </animated.div>)
    )
}

ReactDOM.render(<Test />, document.getElementById('root'))