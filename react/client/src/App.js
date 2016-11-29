import React, { Component } from 'react';
//import logo from './logo.svg';
import './App.css';

import axios from 'axios';

import injectTapEventPlugin from 'react-tap-event-plugin';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import getMuiTheme from 'material-ui/styles/getMuiTheme';

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
            token: "",
            path: "/",

            axiosConfig: {
                headers: {
                    'Authorization': "",
                    'X-CSRF-TOKEN': "",
                    'Content-Type': 'application/json',
                },
            },
        };

        this.logIn = this.logIn.bind(this);
        this.selectPath = this.selectPath.bind(this);
        this.injectChildrenWithProps = this.injectChildrenWithProps.bind(this);
    }
    componentWillMount() {
        console.log('will mount');
        if (this.state.path === "/") {
            this.selectPath(DEFAULT_LANDING);
        }
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
    logIn(username, password) {
        console.log("log in! " + username + ', ' + password);
        let data = {
            username: username,
            password: password,
        };
        axios.post('/login', data, this.state.axiosConfig)
            .then((resp) => {
                console.log(resp);
            })
            .catch((err) => {
                console.log(err);
            });
        this.setState({
            authenicated: true,
        });

    }
    injectChildrenWithProps(children) {
        return React.Children.map(children, (child) => {
            let props = {
                // funcs
                logIn: this.logIn,

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
                        {this.state.authenicated?
                            this.injectChildrenWithProps(this.props.children)
                            :
                            <Login
                                logIn={this.logIn}
                            />
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
