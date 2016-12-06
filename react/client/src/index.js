import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';

import About from './components/About';
import WhoAmI from './components/WhoAmI';

import AdminApp from './admin/AdminApp';

import { Router, Route, hashHistory } from 'react-router';

ReactDOM.render(
    <Router history={hashHistory}>
        <Route path="/" component={App}>
            <Route path="/about" component={About}/>
            <Route path="/whoami" component={WhoAmI}/>
            <Route path="/admin" component={AdminApp}>

            </Route>
        </Route>
    </Router>
    ,
    document.getElementById('root')
);
