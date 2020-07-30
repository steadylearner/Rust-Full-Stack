import React from 'react';
import chrome from '../components/chrome';

import { makeStyles } from '@material-ui/core/styles';
import Grid from '@material-ui/core/Grid';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import InputBase from '@material-ui/core/InputBase';
import IconButton from '@material-ui/core/IconButton';
import CopyIcon from '@material-ui/icons/FileCopyOutlined';

import Generator from '../components/generator';

const passwordStyles = makeStyles(theme => ({
    container: {
        marginTop: theme.spacing(4),
    },
}));

function Password(props) {
    let refPassword = React.useRef();

    const classes = passwordStyles();
    if (!props.value) {
        return null;
    }

    const onCopy = () => {
        refPassword.current.select();
        document.execCommand('copy');
    }

    return (
        <Grid container justify="center" className={classes.container}>
            <Typography color="primary"
                align="center" variant="button">
                Your {props.length} Characters Password
            </Typography>
            <Grid>
                <Grid container direction="row" >
                    <InputBase value={props.value}
                        inputProps={{ ref: refPassword, spellCheck: 'false' }}
                    />
                    <IconButton onClick={onCopy}>
                        <CopyIcon />
                    </IconButton>
                </Grid>
            </Grid>
        </Grid>
    )
}

const pageStyles = makeStyles(theme => ({
    root: {
        margin: theme.spacing(2),
        width: '20rem'
    },
    copyright: {
        marginTop: theme.spacing(2),
    }
}));

function PopupPage(props) {
    const classes = pageStyles();

    let [password, setPassword] = React.useState(null);

    const PASSWORD_LENGTH = 15;
    const onGenerate = () => {
        setPassword(Generator(PASSWORD_LENGTH));
    }

    const goWebsite = () => {
        chrome.tabs.create({
            url: 'https://10converters.com'
        })
    }

    return (
        <Grid container justify="center" className={classes.root}>
            <Button variant="contained" color="primary"
                onClick={onGenerate}>
                Generate Strong Password
            </Button>
            <Password length={PASSWORD_LENGTH} value={password} />
            <Typography className={classes.copyright} variant="overline" color="textSecondary" onClick={goWebsite}>
                Powered by 10Converters.com
            </Typography>
        </Grid>
    )
}

export default PopupPage;