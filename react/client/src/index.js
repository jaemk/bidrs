import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import About from './components/About';

import { Router, Route, hashHistory } from 'react-router';

ReactDOM.render(
    <Router history={hashHistory}>
        <Route path="/" component={App}>
            <Route path="/about" component={About}/>
        </Route>
    </Router>
    ,
    document.getElementById('root')
);
