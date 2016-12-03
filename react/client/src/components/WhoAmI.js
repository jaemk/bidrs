import React, { Component } from 'react';

import RaisedButton from 'material-ui/RaisedButton';


class WhoAmI extends Component {
    constructor() {
        super();
        this.state = {
            whoami: "",
        };
        this.updateMe = this.updateMe.bind(this);
        this.findMe = this.findMe.bind(this);
    }
    updateMe(resp) {
        console.log(resp);
        let email = resp.data.email;
        this.setState({
            whoami: email,
        });
    }
    findMe() {
        this.props.apiGet('/whoami', this.updateMe);
    }
    render() {
        return (
            <div>
                { this.state.whoami?
                    'You are ' + this.state.whoami + '!!'
                    :
                    'Who are you?'
                }
                <br/>
                <br/>
                <RaisedButton
                    label="Find out!"
                    onTouchTap={this.findMe}
                />
            </div>
        );
    }
}

export default WhoAmI;
