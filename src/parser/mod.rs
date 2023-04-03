mod style_sheet_parser;
pub(crate) use style_sheet_parser::*;

mod property_parser;
use property_parser::PropertyParser;

use crate::prelude::BevyCssError;

use tomt_cssparser::{
    ParseError, Parser,
    ToCss, Token,
};
use smallvec::{smallvec, SmallVec};


fn format_error(
    error: ParseError<BevyCssError>
) -> String {
    let error_description = match error.kind {
        tomt_cssparser::ParseErrorKind::Basic(b) => match b {
            tomt_cssparser::BasicParseErrorKind::UnexpectedToken(token) => {
                format!("Unexpected token {}", token.to_css_string())
            }
            tomt_cssparser::BasicParseErrorKind::EndOfInput => "End of input".to_string(),
            tomt_cssparser::BasicParseErrorKind::AtRuleInvalid(token) => {
                format!("At rule isn't supported {}", token)
            }
            tomt_cssparser::BasicParseErrorKind::AtRuleBodyInvalid => {
                "At rule isn't supported".to_string()
            }
            tomt_cssparser::BasicParseErrorKind::QualifiedRuleInvalid => "Invalid rule".to_string(),
        },
        tomt_cssparser::ParseErrorKind::Custom(c) => c.to_string(),
    };

    format!(
        "{} at {}:{}",
        error_description, error.location.line, error.location.column
    )
}

fn parse_values<'i>(
    parser: &mut Parser<'i, '_>,
) -> Result<SmallVec<[Token<'i>; 8]>, ParseError<'i, BevyCssError>> {
    let mut values = SmallVec::new();

    while let Ok(token) = parser.next_including_whitespace() {
        values.push(token.clone())
    }

    Ok(values)
}

#[cfg(test)]
mod tests;
