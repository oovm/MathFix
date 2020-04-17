pub struct LaTeXParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    empty_line,
    RestOfLine,
    Number,
    SignedNumber,
    Decimal,
    DecimalBad,
    Integer,
    Exponent,
    Zero,
    Dot,
    Underline,
    Sign,
    String,
    Quotation,
    Group,
    Atom,
    Expression,
    EmptyGroup,
    Function,
    FractionFunction,
    NormalFunction,
    Fraction,
    LargeFunction,
    Escape,
    Operators,
    SYMBOL,
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
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| self::Exponent(state).or_else(|state| self::SignedNumber(state)).and_then(|state| state.optional(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SignedNumber(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::SignedNumber, |state| state.sequence(|state| state.optional(|state| self::Sign(state)).and_then(|state| self::Decimal(state).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::DecimalBad, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))).or_else(|state| state.sequence(|state| self::Dot(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Zero(state).or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Exponent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Exponent, |state| state.sequence(|state| self::SignedNumber(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("e").or_else(|state| state.match_string("E"))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Sign(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::ASCII_DIGIT(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Zero(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("0")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+").or_else(|state| state.match_string("-"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::String, |state| self::SYMBOL(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotation, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\"")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Group(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Group, |state| self::EmptyGroup(state).or_else(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::Atom(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))).or_else(|state| state.sequence(|state| self::Expression(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Atom(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Atom, |state| self::Number(state).or_else(|state| self::Operators(state)).or_else(|state| self::SYMBOL(state)).or_else(|state| self::NEWLINE(state)).or_else(|state| self::Group(state)).or_else(|state| self::Function(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Expression, |state| state.sequence(|state| self::Atom(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Atom(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Atom(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyGroup(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EmptyGroup, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Function(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Function, |state| self::FractionFunction(state).or_else(|state| self::NormalFunction(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn FractionFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::FractionFunction, |state| state.sequence(|state| self::Fraction(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Integer(state).or_else(|state| self::Atom(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Integer(state).or_else(|state| self::Atom(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NormalFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NormalFunction, |state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Atom(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Fraction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Fraction, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| state.match_string("frac").or_else(|state| state.match_string("dfrac"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LargeFunction(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LargeFunction, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Escape(state).and_then(|state| state.match_string("int").or_else(|state| state.match_string("sum")).or_else(|state| state.match_string("prod")).or_else(|state| state.match_string("limit"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Operators(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Operators, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("+").or_else(|state| state.match_string("-")).or_else(|state| state.match_string("&")).or_else(|state| state.match_string("*")).or_else(|state| state.match_string("(")).or_else(|state| state.match_string(")")).or_else(|state| state.match_string("[")).or_else(|state| state.match_string("]")).or_else(|state| state.match_string("<")).or_else(|state| state.match_string(">")).or_else(|state| self::IDS_BINARY_OPERATOR(state)).and_then(|state| state.repeat(|state| state.match_string("+").or_else(|state| state.match_string("-")).or_else(|state| state.match_string("&")).or_else(|state| state.match_string("*")).or_else(|state| state.match_string("(")).or_else(|state| state.match_string(")")).or_else(|state| state.match_string("[")).or_else(|state| state.match_string("]")).or_else(|state| state.match_string("<")).or_else(|state| state.match_string(">")).or_else(|state| self::IDS_BINARY_OPERATOR(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state))))))
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
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn IDS_BINARY_OPERATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::IDS_BINARY_OPERATOR)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
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
            Rule::Number => rules::Number(state),
            Rule::SignedNumber => rules::SignedNumber(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Exponent => rules::Exponent(state),
            Rule::Zero => rules::Zero(state),
            Rule::Dot => rules::Dot(state),
            Rule::Underline => rules::Underline(state),
            Rule::Sign => rules::Sign(state),
            Rule::String => rules::String(state),
            Rule::Quotation => rules::Quotation(state),
            Rule::Group => rules::Group(state),
            Rule::Atom => rules::Atom(state),
            Rule::Expression => rules::Expression(state),
            Rule::EmptyGroup => rules::EmptyGroup(state),
            Rule::Function => rules::Function(state),
            Rule::FractionFunction => rules::FractionFunction(state),
            Rule::NormalFunction => rules::NormalFunction(state),
            Rule::Fraction => rules::Fraction(state),
            Rule::LargeFunction => rules::LargeFunction(state),
            Rule::Escape => rules::Escape(state),
            Rule::Operators => rules::Operators(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
