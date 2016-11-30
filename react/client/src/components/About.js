import React, { Component } from 'react';

import RaisedButton from 'material-ui/RaisedButton';

class About extends Component {
    constructor() {
        super();
        this.state = {
            msg: null,
        };
        this.click = this.click.bind(this);
        this.handleResp = this.handleResp.bind(this);
    }
    handleResp(resp) {
        this.setState({msg: resp.data.msg});
    }
    click() {
        this.props.apiGet("/msg", this.handleResp);
    }
    render() {
        return (
            <div>
                <pre> I am about! </pre>
                { this.props.token?
                    <pre> You're authenicated! {this.props.token} </pre>
                        :
                    ""
                }
                <RaisedButton
                    label="try a get"
                    onTouchTap={this.click}
                />
                <pre>
                    { this.state.msg?
                        this.state.msg : "No msg"
                    }
                </pre>
            </div>
        );
    }
}

export default About;
