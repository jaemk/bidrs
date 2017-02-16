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
            administrator: false,
            authError: false,
            authErrorMessage: "",
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
        this.logOut = this.logOut.bind(this);
        this.relogOnUnauthorized = this.relogOnUnauthorized.bind(this);
        this.resetAuthError = this.resetAuthError.bind(this);
        this.apiGet = this.apiGet.bind(this);
        this.apiPost = this.apiPost.bind(this);
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

    // Wrapper around axios.post, applies 'handler' to a
    // successful response and optional 'errHandler' to any error response
    apiPost(url, data, handler, errHandler) {
        axios.post(url, data, this.state.axiosConfig.toJS())
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
    logIn(email, password) {
        let data = {
            email: email,
            password: password,
        };
        const handler = (resp) => {
            let token = resp.data.token;
            let admin = resp.data.admin? true : false;
            let config = this.state.axiosConfig;
            config = config.setIn(['headers', 'Authorization'], token);
            console.log(config);
            this.setState({
                authenticated: true,
                administrator: admin,
                token: token,
                axiosConfig: config,
            });
        };
        const errHandler = (err) => {
            console.log(err);
            this.setState({
                authErrorMessage: "Invalid Credentials",
            });
        };
        this.apiPost('/login', data, handler, errHandler);
    }

    // Log out user/admin
    logOut() {
        const handler = (resp) => {
            let msg = resp.data.msg;
            if (msg === "logout") {
                this.setState({
                    authenticated: false,
                    administrator: false,
                });
            }
        };
        const errHandler = (err) => { console.log(err); }
        this.apiPost('/logout', {}, handler, errHandler);
    }

    // Reset auth errors that propagate to Login
    resetAuthError() {
        if (this.state.authError || this.state.authErrorMessage) {
            this.setState({
                authError: false,
                authErrorMessage: "",
            });
        }
    }

    // Redirect router to the specified path
    selectPath(path) {
        console.log('Select path: ' + path);
        let rlen = this.context.router.routes.length;
        let current = this.context.router.routes[rlen-1].path;
        if (path !== current) {
            this.context.router.push(path);
        }
        this.setState({ path: path });
    }

    // Inject specified properties into all child components
    injectChildrenWithProps(children) {
        return React.Children.map(children, (child) => {
            let props = {
                // funcs
                apiGet: this.apiGet,
                apiPost: this.apiPost,

                logOut: this.logOut,

                // app.state
                token: this.state.token,

                // login
                login: {
                    error: this.state.authError,
                },
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
                                    error={this.state.authError}
                                    resetError={this.resetAuthError}
                                />
                                <Snackbar
                                    open={this.state.authError}
                                    message={this.state.authErrorMessage?
                                            this.state.authErrorMessage
                                            :
                                            "Please log back in!"}
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
