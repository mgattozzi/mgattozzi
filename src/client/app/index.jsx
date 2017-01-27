// React
import React from 'react';
import {render} from 'react-dom';

// React Router
import { Router, Route, Link, browserHistory } from 'react-router'

// Nav Bar Pages
import About from './about.jsx';
import Archive from './archive.jsx';
import Contact from './contact.jsx';
import Main from './main.jsx';
import Resume from './resume.jsx';
import Count from './count.jsx';
import {
  Alchemist,
  BlogRust,
  DieselRocket,
  HaskellRust,
  StdMacros,
  StrString,
  OneRust,
  HyperAsync,
  Pipers,
  RussianDolls,
  RustHaskell,
  RustIs,
  SchemeEx1,
  SchemeInput,
  SchemeParser,
  SchrodingersBug,
  WhereClauses,
} from './posts.jsx';

render(
      <div>
        <Router history={browserHistory}>
          <Route path="/" component={Main}>
            <Route path="about" component={About}/>
            <Route path="archive" component={Archive}/>
            <Route path="contact" component={Contact}/>
            <Route path="counting" component={Count}/>
            <Route path="diesel-powered-rocket" component={DieselRocket}/>
            <Route path="resume" component={Resume}/>
            <Route path="announcing-alchemist" component={Alchemist}/>
            <Route path="blog-about-rust" component={BlogRust}/>
            <Route path="haskell-rust" component={HaskellRust}/>
            <Route path="how-do-i-std-macros" component={StdMacros}/>
            <Route path="how-do-i-str-string" component={StrString}/>
            <Route path="1-year-of-rust" component={OneRust}/>
            <Route path="pipers" component={Pipers}/>
            <Route path="russian-dolls" component={RussianDolls}/>
            <Route path="rust-haskell" component={RustHaskell}/>
            <Route path="rust-is" component={RustIs}/>
            <Route path="scheme-ex1" component={SchemeEx1}/>
            <Route path="scheme-input" component={SchemeInput}/>
            <Route path="scheme-parser" component={SchemeParser}/>
            <Route path="schrodingers-bug" component={SchrodingersBug}/>
            <Route path="understanding-where-clauses" component={WhereClauses}/>
            <Route path="hyper-async" component={HyperAsync}/>
          </Route>
        </Router>
      </div>
      , document.getElementById('root'));

