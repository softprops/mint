extern crate regex;


mod lint;
pub use lint::*;

mod parse;
pub use parse::*;

#[derive(Default, Debug)]
pub struct Makefile {
    rules: Vec<Rule>,
    vars: Vec<Var>,
}

impl Makefile {
    fn new() -> Makefile {
        Makefile { ..Default::default() }
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn add_var(&mut self, var: Var) {
        self.vars.push(var);
    }

    fn rule_line(&mut self, line: String) {
        if let Some(mut last) = self.rules.pop() {
            last.body.push(line);
            self.rules.push(last);
        }
    }
}

#[derive(Debug, Default)]
pub struct Rule {
    target: String,
    dependencies: Vec<String>,
    line: usize,
    body: Vec<String>,
}

#[derive(Debug)]
pub enum Var {
    Eq(String, String, usize),
    ColonEq(String, String, usize),
    DoubleColonEq(String, String, usize),
    PlusEq(String, String, usize),
    QuestionEq(String, String, usize),
    Special(String, String, usize)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
