use super::{
    smallvec,
    parse_values,
};
use crate::{
    prelude::BevyCssError,
    property::PropertyValues,
};

use cssparser::{
    AtRuleParser, DeclarationParser, Parser, ParseError,
};

pub struct PropertyParser;

impl<'i> DeclarationParser<'i> for PropertyParser {
    type Declaration = (String, PropertyValues);

    type Error = BevyCssError;

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        parser: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, BevyCssError>> {
        let mut tokens = smallvec![];
        for token in parse_values(parser)? {
            match token.try_into() {
                Ok(t) => tokens.push(t),
                Err(_) => continue,
            }
        }

        Ok((name.to_string(), PropertyValues(tokens)))
    }
}

impl<'i> AtRuleParser<'i> for PropertyParser {
    type Prelude = ();
    type AtRule = (String, PropertyValues);
    type Error = BevyCssError;
}
