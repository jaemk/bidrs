import React, { Component } from 'react';

import AppBar from 'material-ui/AppBar';
import Drawer from 'material-ui/Drawer';


class Navigation extends Component {
    constructor() {
        super();
        this.state = {
            open: false,
        }
    }
    render() {
        return (
            <div>
                <AppBar
                    title={this.props.title}
                    onLeftIconButtonTouchTap={() => this.setState({open: !this.state.open})}
                />
                <Drawer
                    docked={false}
                    width={200}
                    open={this.state.open}
                    onRequestChange={(open) => this.setState({open}) }
                />
            </div>
        );
    }
}

export default Navigation;
