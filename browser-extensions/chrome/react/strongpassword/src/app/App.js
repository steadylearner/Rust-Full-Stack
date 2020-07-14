import React from 'react';

import Link from '@material-ui/core/Link';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/LIstItem';

import PopupPage from '../popup';

import './App.css';

function App() {
    let url = window.location.href;

    if (url.includes('popup.html')) {
        return (
            <PopupPage />
        );
    }
    /*
    else if (url.includes('options.html')) {
        return (
            <OptionPage />
        );
    }
    */
    else {
        // it's for test, construct an index page with links
        return (
            <List>
                <ListItem>
                    <Link href="options.html">Options Page</Link>
                </ListItem>
                <ListItem>
                    <Link href="popup.html">Popup Page</Link>
                </ListItem>
            </List>
        )
    }
}

export default App;
