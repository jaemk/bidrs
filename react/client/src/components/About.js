import React, { Component } from 'react';

class About extends Component {
    render() {
        return (
            <div>
                <pre> I am about! </pre>
                { this.props.token?
                    <pre> You're authenicated! {this.props.token} </pre>
                        :
                    ""
                }
            </div>
        );
    }
}

export default About;
