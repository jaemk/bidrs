import React, { Component } from 'react';
import redux from 'redux';
import axios from 'axios';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import getMuiTheme from 'material-ui/styles/getMuiTheme';
import RaisedButton from 'material-ui/RaisedButton';


class App extends Component {
    constructor() {
        super();
        this.state = {
            msg: 'No Message',
        };

        this.themer = this.themer.bind(this);
        this.fetchHello = this.fetchHello.bind(this);
    }

    /**
     * themer
     *
     * @returns {muiTheme}
     */
    themer() {
        return getMuiTheme({
            palette: {
                textColor: '#333333',
            },
            appBar: {
                height: 50,
            },
        });
    }

    fetchHello() {
        axios.get("/hello")
            .then((resp) => {
                console.log(resp);
                this.setState({msg: resp.data.data})
            }).catch((err) => console.log(err));
    }

    render() {
        return (
            <MuiThemeProvider muiTheme={this.themer()}>
                <div>
                    <div> Msg: {this.state.msg} </div>
                    <RaisedButton
                        label="Fetch!"
                        onTouchTap={this.fetchHello}
                    />
                </div>
            </MuiThemeProvider>
        );
    }
}

export default App;
