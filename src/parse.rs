use super::{Makefile, Var, Rule};

use regex::Regex;

pub fn parse(file: &str) -> Makefile {
    // rule line with deps
    // https://www.gnu.org/software/make/manual/html_node/Rule-Syntax.html#Rule-Syntax
    let rule = Regex::new("^([a-zA-Z-_]+):(.*)?").unwrap();
    // rule body lines, intended by tabs
    let rule_body = Regex::new("^\t+(.*)").unwrap();

    // http://www.gnu.org/software/make/manual/make.html#Reading-Makefiles
    // variables come in the forms =, :=, ::=, +=, and ?=
    let var = Regex::new("^([a-zA-Z-_]+)\\s*(=|:=|::=|\\+=|\\?=)(.*)").unwrap();

    // special .-prefixed vars
    let special_var = Regex::new("^\\.([a-zA-Z-_]+):(.*)").unwrap();

    let mut makefile = Makefile::new();
    for (i, line) in file.lines().filter(|l| !l.is_empty() && !l.starts_with('#')).enumerate() {
        if let Some(rule) = rule.captures(line) {
            if let Some(target) = rule.at(1) {
                let deps = rule.at(2)
                    .map(|d| {
                        d.trim()
                            .split(" ")
                            .filter(|d| !d.is_empty())
                            .map(|d| d.to_owned())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or(vec![]);
                makefile.add_rule(Rule {
                    target: target.to_owned(),
                    dependencies: deps,
                    line: i,
                    ..Default::default()
                })
            }
        } else if let Some(body) = rule_body.captures(line) {
            if let Some(bod) = body.at(1) {
                makefile.rule_line(bod.trim().to_owned());
            }
        } else if let Some(var) = var.captures(line) {
            if let (Some(k), Some(b), Some(v)) = (var.at(1), var.at(2), var.at(3)) {
                let (key, value) = (k.to_owned(), v.trim().to_owned());
                let v = match b {
                        "=" => Var::Eq(key, value, i),
                        ":=" => Var::ColonEq(key, value, i),
                        "::=" => Var::DoubleColonEq(key, value, i),
                        "+=" => Var::PlusEq(key, value, i),
                        "?=" => Var::QuestionEq(key, value, i),
                        _ => unreachable!()
                };
                makefile.add_var(v);
            }
        } else if let Some(var) = special_var.captures(line) {
            if let (Some(k), Some(v)) = (var.at(1), var.at(2)) {
                let v = Var::Special(k.to_owned(), v.trim().to_owned(), i);
                makefile.add_var(v);
            }
        } else {
            println!("unparsed line! '{}''", line)
        }
    }
    makefile
}
