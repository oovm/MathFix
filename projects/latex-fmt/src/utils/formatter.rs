use crate::{
    utils::{replace_operator, replace_operator_name},
    Settings,
};
use latex_parser::{LaTeXParser, Rule};
use pest::{iterators::Pair, Parser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl Settings {
    pub fn format_file(&self, path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r);
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        return Ok(());
    }
    pub fn format(&self, text: &str) -> String {
        let pairs = LaTeXParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::NEWLINE => code.push_str("\n"),
                Rule::Expression => code.push_str(&self.format_expression(pair)),
                Rule::COMMENT => code.push_str(&format!("{}\n", pair.as_str())),
                _ => debug_cases!(pair),
            };
        }
        //        println!("{:#?}", codes);
        //        unreachable!();
        return code;
    }

    fn format_expression(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::COMMENT => {
                    let c = pair.as_str();
                    codes.push(format!("% {}", c[1..c.len()].trim()))
                }
                Rule::Group => codes.push(self.format_group(pair, false)),
                Rule::Atom => codes.push(self.format_atom(pair)),
                _ => debug_cases!(pair),
            };
        }
        return codes.join("");
    }

    fn format_group(&self, pairs: Pair<Rule>, keep_group: bool) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Atom => codes.push(self.format_atom(pair)),
                Rule::Group => {
                    if keep_group {
                        codes.push(String::from("{"));
                        codes.push(self.format_group(pair, true));
                        codes.push(String::from("}"));
                    }
                    else {
                        codes.push(self.format_group(pair, false));
                    }
                }
                Rule::Expression => codes.push(self.format_expression(pair)),
                _ => debug_cases!(pair),
            };
        }
        return codes.join("");
    }

    fn format_atom(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::NEWLINE => codes.push(String::from("\n")),
                Rule::Number => codes.push(pair.as_str().to_string()),
                Rule::SYMBOL => codes.push(pair.as_str().to_string()),
                Rule::Operators => codes.push(replace_operator(pair.as_str())),
                Rule::Group => codes.push(self.format_group(pair, false)),
                Rule::Function => codes.push(self.format_function(pair)),
                _ => debug_cases!(pair),
            };
        }
        return codes.join("\n");
    }

    fn format_function(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::NormalFunction => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::Escape => continue,
                            Rule::NormalHead => codes.push(replace_operator_name(inner.as_str())),
                            Rule::SYMBOL => codes.push(format!(" {}", inner.as_str())),
                            Rule::Group => {
                                codes.push(String::from("{"));
                                codes.push(self.format_group(inner, false));
                                codes.push(String::from("}"));
                            }
                            _ => debug_cases!(inner),
                        };
                    }
                }
                Rule::UnaryFunction => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::UnaryHead => codes.push(inner.as_str().to_string()),
                            Rule::Atom => codes.push(self.format_atom(inner)),
                            Rule::Group => {
                                codes.push(String::from("{"));
                                codes.push(self.format_group(inner, false));
                                codes.push(String::from("}"));
                            }
                            _ => debug_cases!(inner),
                        };
                    }
                }
                _ => debug_cases!(pair),
            };
        }
        return codes.join("");
    }
}
