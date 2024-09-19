use super::{
    parse_values,
    smallvec,
};
use crate::{
    prelude::BevyCssError,
    property::PropertyValues,
};

use cssparser::{AtRuleParser, DeclarationParser, Parser, ParseError, RuleBodyItemParser, QualifiedRuleParser};

pub struct PropertyParser;

impl<'i> DeclarationParser<'i>
for PropertyParser
{
    type Declaration = (String, PropertyValues);
    type Error = BevyCssError;

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        parser: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, BevyCssError>> {
        let mut tokens = smallvec![];
        for token in parse_values(parser)?
        {
            match token.try_into()
            {
                Ok(t) => tokens.push(t),
                Err(_) => continue,
            }
        }

        Ok((name.to_string(), PropertyValues(tokens)))
    }
}

impl<'i> RuleBodyItemParser<'i, (String, PropertyValues), BevyCssError> for PropertyParser {
    fn parse_declarations(&self) -> bool
    {
        true
    }

    fn parse_qualified(&self) -> bool
    {
        false
    }
}

impl<'i> AtRuleParser<'i>
for PropertyParser
{
    type Prelude = ();
    type AtRule = (String, PropertyValues);
    type Error = BevyCssError;
}

impl<'i> QualifiedRuleParser<'i> for PropertyParser {
    type Prelude = ();
    type QualifiedRule = (String, PropertyValues);
    type Error = BevyCssError;
}