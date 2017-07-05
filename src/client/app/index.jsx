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
  AvoidingLogicErrors,
  Alchemist,
  BlogRust,
  DieselRocket,
  GlobalUninit,
  HaskellRust,
  StdMacros,
  StrString,
  OneRust,
  HyperAsync,
  HyperClient,
  Pipers,
  RussianDolls,
  RefactorRust,
  RustHaskell,
  RustIs,
  SchemeEx1,
  SchemeInput,
  SchemeParser,
  SchrodingersBug,
  TwoYears,
  WhereClauses,
} from './posts.jsx';

render(
      <div>
        <Router history={browserHistory}>
          <Route path="/" component={Main}>
            <Route path="about" component={About}/>
            <Route path="archive" component={Archive}/>
            <Route path="avoiding-logic-errors" component={AvoidingLogicErrors}/>
            <Route path="global-uninitialized" component={GlobalUninit}/>
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
            <Route path="2-years-of-rust" component={TwoYears}/>
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
            <Route path="hyper-client" component={HyperClient}/>
            <Route path="refactor-rust" component={RefactorRust}/>
            <Route path="global-uninitialized" component={GlobalUninit}/>
          </Route>
        </Router>
      </div>
      , document.getElementById('root'));

