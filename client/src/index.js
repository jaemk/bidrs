import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';

import { polyfill as ObjAssignPolyfill } from 'es6-object-assign';
import PromiseFill from 'es6-promise';
import injectTapEventPlugin from 'react-tap-event-plugin';


ObjAssignPolyfill();
PromiseFill.polyfill();
injectTapEventPlugin();


ReactDOM.render(
  <App />,
  document.getElementById('root')
);
