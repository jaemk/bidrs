import React, { Component } from 'react';

import TextField from 'material-ui/TextField';
import RaisedButton from 'material-ui/RaisedButton';
import Paper from 'material-ui/Paper';
import Subheader from 'material-ui/Subheader';


class Login extends Component {
    constructor() {
        super();
        this.state = {
            email: "",
            password: "",
        };

        this.handleInput = this.handleInput.bind(this);
        this.logIn = this.logIn.bind(this);
    }
    componentWillReceiveProps(props) {
        if (props.error) {
            this.setState({ password: "" });
        }
    }
    logIn() {
        this.props.logIn(this.state.email, this.state.password);
    }
    handleInput(e, type) {
        this.props.resetError();
        let value = e.target.value;
        let state = {};
        state[type] = value;
        this.setState(state);
    }
    render() {
        return (
            <Paper>
                {this.props.message?
                    <Subheader> {this.props.message} </Subheader>
                    :
                    <Subheader> Log in to --- App --- </Subheader>
                }
                <TextField
                    floatingLabelText="Email"
                    errorText={this.props.error? 'Invalid credentials' : null}
                    value={this.state.email}
                    onChange={(e) => this.handleInput(e, 'email')}
                />
                {' '}
                <TextField
                    floatingLabelText="Password"
                    errorText={this.props.error? 'Invalid credentials' : null}
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
Login.propTypes = {
    logIn: React.PropTypes.func,
    error: React.PropTypes.bool,
    resetError: React.PropTypes.func,
};

export default Login;
