use crate::{Parse, ParseResult, ParseToEnd, Parser};

use sway_ast::brackets::{Braces, Parens, SquareBrackets};
use sway_ast::token::OpeningDelimiter;
use sway_error::handler::ErrorEmitted;
use sway_error::parser_error::ParseErrorKind;

pub trait ParseBracket<T>: Sized {
    fn try_parse(&self, parser: &mut Parser) -> ParseResult<Option<Self>>
    where
        T: ParseToEnd;

    fn parse_all_inner(
        &self,
        parser: &mut Parser,
        on_error: impl FnOnce(Parser) -> ErrorEmitted,
    ) -> ParseResult<Self>
    where
        T: Parse;

    fn try_parse_all_inner(
        &self,
        parser: &mut Parser,
        on_error: impl FnOnce(Parser) -> ErrorEmitted,
    ) -> ParseResult<Option<Self>>
    where
        T: Parse;
}

macro_rules! impl_brackets (
    ($ty_name:ident, $delimiter:ident, $error:ident) => {
        impl<T> ParseBracket<T> for $ty_name<T> {
            fn try_parse(&self, parser: &mut Parser) -> ParseResult<Option<$ty_name<T>>>
            where
                T: ParseToEnd
            {
                match parser.enter_delimited(OpeningDelimiter::$delimiter) {
                    Some((parser, span)) => {
                        let (inner, _consumed) = parser.parse_to_end()?;
                        Ok(Some($ty_name { open_token: self.open_token, inner, close_token: self.close_token }))
                    },
                    None => Ok(None),
                }
            }

            fn parse_all_inner(
                &self,
                parser: &mut Parser,
                on_error: impl FnOnce(Parser) -> ErrorEmitted,
            ) -> ParseResult<$ty_name<T>>
            where
                T: Parse
            {
                match parser.enter_delimited(OpeningDelimiter::$delimiter) {
                    Some((mut parser, span)) => {
                        let inner = parser.parse()?;
                        if !parser.is_empty() {
                            return Err(on_error(parser))
                        }
                        Ok($ty_name { open_token: self.open_token, inner, close_token: self.close_token })
                    },
                    None => Err(parser.emit_error(ParseErrorKind::$error)),
                }
            }

            fn try_parse_all_inner(
                &self,
                parser: &mut Parser,
                on_error: impl FnOnce(Parser) -> ErrorEmitted,
            ) -> ParseResult<Option<$ty_name<T>>>
            where
                T: Parse
            {
                match parser.enter_delimited(OpeningDelimiter::$delimiter) {
                    Some((mut parser, span)) => {
                        let inner = parser.parse()?;
                        if !parser.is_empty() {
                            return Err(on_error(parser))
                        }
                        Ok(Some($ty_name { open_token: self.open_token, inner, close_token: self.close_token }))
                    },
                    None => Ok(None),
                }
            }
        }

        impl<T> Parse for $ty_name<T>
        where
            T: ParseToEnd,
        {
            fn parse(parser: &mut Parser) -> ParseResult<$ty_name<T>> {
                match parser.enter_delimited(OpeningDelimiter::$delimiter) {
                    Some((parser, span)) => {
                        let (inner, _consumed) = parser.parse_to_end()?;
                        Ok($ty_name { open_token, inner, close_token })
                    },
                    None => Err(parser.emit_error(ParseErrorKind::$error)),
                }
            }
        }
    };
);

impl_brackets!(Braces, CurlyBrace, ExpectedOpenBrace);
impl_brackets!(Parens, Parenthesis, ExpectedOpenParen);
impl_brackets!(SquareBrackets, SquareBracket, ExpectedOpenBracket);
