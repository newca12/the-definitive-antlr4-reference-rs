### About ###
This is a Rust port of the code from Terence Parr's ["The Definitive ANTLR 4 Reference"](https://pragprog.com/titles/tpantlr2/the-definitive-antlr-4-reference/).

the-definitive-antlr4-reference-rs is an EDLA project.

The purpose of [edla.org](http://www.edla.org) is to promote the state of the art in various domains.

### Usage ###
The code use a preview of [antlr4rust 0.3](https://github.com/rrevenantt/antlr4rust/pull/29).

### Issues with antlr4rust (all very minor so far) ###
* starter: ArrayInitListener in translate.rs show no value in enter_value, exit_value has been used instead.
* tour: generation with -no-listener -visitor leave LabeledExprListener usage in labeledexprparser.rs and lead to compilation error. Use only -visitor fix it.

### License ###
© 2021 Olivier ROLAND. Distributed under the GPLv3 License.