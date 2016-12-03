import React, { Component } from 'react';
//import logo from './logo.svg';
import './App.css';

import axios from 'axios';
import { Map } from 'immutable';

import injectTapEventPlugin from 'react-tap-event-plugin';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import getMuiTheme from 'material-ui/styles/getMuiTheme';

import Snackbar from 'material-ui/Snackbar';

import Login from './components/Login';
import Navigation from './components/Navigation';

injectTapEventPlugin();

const DEFAULT_LANDING = "/about";

const muiThemer = (path) => {
    return getMuiTheme({
    });
}

class App extends Component {
    constructor() {
        super();
        this.state = {
            authenticated: false,
            authError: false,
            token: "",
            path: "/",

            axiosConfig: Map({
                headers: Map({
                    'Authorization': "",
                    'X-CSRF-TOKEN': "",
                    'Content-Type': 'application/json',
                }),
            }),
        };

        this.logIn = this.logIn.bind(this);
        this.relogOnUnauthorized = this.relogOnUnauthorized.bind(this);
        this.apiGet = this.apiGet.bind(this);
        this.selectPath = this.selectPath.bind(this);
        this.injectChildrenWithProps = this.injectChildrenWithProps.bind(this);
    }

    componentWillMount() {
        console.log('will mount');
        if (this.state.path === "/") {
            this.selectPath(DEFAULT_LANDING);
        }
    }

    // Should be called on all api error responses to
    // redirect to the login page if it's an auth error
    relogOnUnauthorized(error) {
        if (error.response.status === 401) {
            this.setState({
                authenticated: false,
                authError: true,
            });
        }
    }

    // Wrapper around axios.get that applies 'handler' to a
    // successful response and the optional 'errHandler' to
    // any error responses
    apiGet(url, handler, errHandler) {
        axios.get(url, this.state.axiosConfig.toJS())
            .then((resp) => {
                handler(resp);
            })
            .catch((err) => {
                this.relogOnUnauthorized(err);
                if (errHandler) {
                    errHandler(err);
                }
            });
    }

    // Log in user
    // TODO: Make this a general apiPost
    logIn(email, password) {
        //console.log("log in! " + email + ', ' + password);
        let data = {
            email: email,
            password: password,
        };
        axios.post('/login', data, this.state.axiosConfig.toJS())
            .then((resp) => {
                let token = resp.data.token;
                let config = this.state.axiosConfig;
                config = config.setIn(['headers', 'Authorization'], token);
                console.log(config);
                this.setState({
                    authenticated: true,
                    token: token,
                    axiosConfig: config,
                });
            })
            .catch((err) => {
                console.log(err);
            });
    }
    selectPath(path) {
        console.log('Select path: ' + path);
        let rlen = this.context.router.routes.length;
        let current = this.context.router.routes[rlen-1].path;
        if (path !== current) {
            this.context.router.push(path);
        }
        this.setState({ path: path });
    }
    injectChildrenWithProps(children) {
        return React.Children.map(children, (child) => {
            let props = {
                // funcs
                apiGet: this.apiGet,

                // app.state
                token: this.state.token,
            };
            return React.cloneElement(child, props);
        });
    }
    render() {
        return (
            <MuiThemeProvider muiTheme={muiThemer(this.state.path)}>
                <div>
                    <Navigation
                        title="Bid"
                    />
                    <div className="AppBody">
                        {this.state.authenticated?
                            this.injectChildrenWithProps(this.props.children)
                            :
                            <div className="Login">
                                <Login
                                    logIn={this.logIn}
                                />
                                <Snackbar
                                    open={this.state.authError}
                                    message="Please log back in!"
                                />
                            </div>
                        }
                    </div>
                </div>
            </MuiThemeProvider>
        );
    }
}
App.contextTypes = {
    router: React.PropTypes.object,
};

export default App;
