import React, { Component } from 'react';

import TextField from 'material-ui/TextField';
import RaisedButton from 'material-ui/RaisedButton';
import Paper from 'material-ui/Paper';


class Login extends Component {
    constructor() {
        super();
        this.state = {
            username: "",
            password: "",
        };

        this.handleInput = this.handleInput.bind(this);
        this.logIn = this.logIn.bind(this);
    }
    logIn() {
        this.props.logIn(this.state.username, this.state.password);
    }
    handleInput(e, type) {
        let value = e.target.value;
        let state = {};
        state[type] = value;
        this.setState(state);
    }
    render() {
        return (
            <Paper>
                <TextField
                    floatingLabelText="Username"
                    value={this.state.username}
                    onChange={(e) => this.handleInput(e, 'username')}
                />
                {' '}
                <TextField
                    floatingLabelText="Password"
                    value={this.state.password}
                    type="password"
                    onChange={(e) => this.handleInput(e, 'password')}
                />
                <br/>
                <RaisedButton
                    label="Log In!"
                    primary={true}
                    onTouchTap={this.logIn}
                />
                <br/>
                <br/>
            </Paper>
        );
    }
}

export default Login;
