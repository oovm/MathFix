pub struct LaTeXParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    empty_line,
    RestOfLine,
    Group,
    Atom,
    Expression,
    EmptyGroup,
    Function,
    NormalFunction,
    UnaryFunction,
    NormalHead,
    UnaryHead,
    SYMBOL,
    Number,
    Escape,
    Operators,
    EscapedOperator,
    COMMENT,
    WHITESPACE,
    NEWLINE,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for LaTeXParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::SEPARATOR(state).or_else(|state| state.sequence(|state| self::empty_line(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::empty_line(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::empty_line(state)))))))))).or_else(|state| self::Expression(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn empty_line(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.optional(|state| self::WHITESPACE(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::WHITESPACE(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::NEWLINE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RestOfLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RestOfLine, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Group(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Group, |state| self::EmptyGroup(state).or_else(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::Group(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}")))).or_else(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::Atom(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))).or_else(|state| state.sequence(|state| self::Expression(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Atom(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Atom, |state| self::Number(state).or_else(|state| self::Operators(state)).or_else(|state| self::SYMBOL(state)).or_else(|state| self::NEWLINE(state)).or_else(|state| self::Function(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Expression, |state| state.sequence(|state| self::Atom(state).or_else(|state| self::Group(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Atom(state).or_else(|state| self::Group(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Atom(state).or_else(|state| self::Group(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyGroup(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EmptyGroup, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Function(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Function, |state| self::UnaryFunction(state).or_else(|state| self::NormalFunction(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NormalFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NormalFunction, |state| state.sequence(|state| self::NormalHead(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state).or_else(|state| state.sequence(|state| state.optional(|state| self::Group(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Group(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn UnaryFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::UnaryFunction, |state| state.sequence(|state| self::UnaryHead(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Atom(state).or_else(|state| self::Group(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NormalHead(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NormalHead, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::SYMBOL(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn UnaryHead(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::UnaryHead, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("^").or_else(|state| state.match_string("_"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_START(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Number, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))).or_else(|state| state.sequence(|state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))).and_then(|state| state.match_string(".")).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))).or_else(|state| state.sequence(|state| state.match_string(".").and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))).or_else(|state| state.sequence(|state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))).and_then(|state| state.match_string("."))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Operators(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Operators, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_range('!'..'/').or_else(|state| state.match_range(':'..'@')).or_else(|state| state.match_string("`")).or_else(|state| state.match_string("[")).or_else(|state| state.match_string("]")).or_else(|state| self::IDS_BINARY_OPERATOR(state)).or_else(|state| self::IDS_TRINARY_OPERATOR(state)).or_else(|state| self::EscapedOperator(state)).and_then(|state| state.repeat(|state| state.match_range('!'..'/').or_else(|state| state.match_range(':'..'@')).or_else(|state| state.match_string("`")).or_else(|state| state.match_string("[")).or_else(|state| state.match_string("]")).or_else(|state| self::IDS_BINARY_OPERATOR(state)).or_else(|state| self::IDS_TRINARY_OPERATOR(state)).or_else(|state| self::EscapedOperator(state))))).or_else(|state| state.match_string("–")).or_else(|state| state.match_string("−")).or_else(|state| state.match_string("|"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EscapedOperator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EscapedOperator, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| self::Escape(state).or_else(|state| state.match_string("{")).or_else(|state| state.match_string("}")).or_else(|state| state.match_string(","))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::COMMENT, |state| state.sequence(|state| state.match_string("%").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::SPACE_SEPARATOR(state).or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NEWLINE, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\r\n").or_else(|state| state.match_string("\r")).or_else(|state| state.match_string("\n"))))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn IDS_BINARY_OPERATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::IDS_BINARY_OPERATOR)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn IDS_TRINARY_OPERATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::IDS_TRINARY_OPERATOR)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SEPARATOR)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::empty_line => rules::empty_line(state),
            Rule::RestOfLine => rules::RestOfLine(state),
            Rule::Group => rules::Group(state),
            Rule::Atom => rules::Atom(state),
            Rule::Expression => rules::Expression(state),
            Rule::EmptyGroup => rules::EmptyGroup(state),
            Rule::Function => rules::Function(state),
            Rule::NormalFunction => rules::NormalFunction(state),
            Rule::UnaryFunction => rules::UnaryFunction(state),
            Rule::NormalHead => rules::NormalHead(state),
            Rule::UnaryHead => rules::UnaryHead(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::Number => rules::Number(state),
            Rule::Escape => rules::Escape(state),
            Rule::Operators => rules::Operators(state),
            Rule::EscapedOperator => rules::EscapedOperator(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
