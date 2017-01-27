import React from 'react';
import Remarkable from 'remarkable'
import hljs from 'highlight.js'

// Posts
import alchemist from 'raw-loader!./posts/announcing-alchemist.md';
import blogRust from 'raw-loader!./posts/blog-about-rust.md';
import haskellRust from 'raw-loader!./posts/haskell-rust.md';
import stdMacros from 'raw-loader!./posts/how-do-i-std-macros.md';
import strString from 'raw-loader!./posts/how-do-i-str-string.md';
import oneRust from 'raw-loader!./posts/1-year-of-rust.md';
import pipers from 'raw-loader!./posts/pipers.md';
import russianDolls from 'raw-loader!./posts/russian-dolls.md';
import rustHaskell from 'raw-loader!./posts/rust-haskell.md';
import schemeEx1 from 'raw-loader!./posts/scheme-ex1.md';
import schemeInput from 'raw-loader!./posts/scheme-input.md';
import schemeParser from 'raw-loader!./posts/scheme-parser.md';
import schrodingersBug from 'raw-loader!./posts/schrodingers-bug.md';
import whereClauses from 'raw-loader!./posts/understanding-where-clauses.md';
import rustIs from 'raw-loader!./posts/rust-is.md';
import dieselRocket from 'raw-loader!./posts/diesel-powered-rocket.md';
import hyperAsync from 'raw-loader!./posts/hyper-async.md';

var md = new Remarkable({
  html: true,
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(lang, str).value;
      } catch (err) {}
    }

    try {
      return hljs.highlightAuto(str).value;
    } catch (err) {}

    return '';
  }
});

export class Alchemist extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(alchemist))};
    return(<div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class BlogRust extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(blogRust))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>); }
}
export class HaskellRust extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(haskellRust))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class StdMacros extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(stdMacros))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class StrString extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(strString))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class OneRust extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(oneRust))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class Pipers extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(pipers))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class RussianDolls extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(russianDolls))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class RustHaskell extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(rustHaskell))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class SchemeEx1 extends React.Component {
  render() {
    const fileHtml = {__html: md.render(eval(schemeEx1))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class SchemeInput extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(schemeInput))};
    return(<div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class SchemeParser extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(schemeParser))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class SchrodingersBug extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(schrodingersBug))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class WhereClauses extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(whereClauses))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class RustIs extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(rustIs))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class DieselRocket extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(dieselRocket))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
export class HyperAsync extends React.Component {
  render () {
    const fileHtml = {__html: md.render(eval(hyperAsync))};
    return( <div dangerouslySetInnerHTML={fileHtml}></div>);
  }
}
