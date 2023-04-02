use super::{
    smallvec, SmallVec,
    format_error,
    PropertyParser
};
use crate::{
    Selector, SelectorElement,
    StyleRule, BevyCssError,
};

use tomt_cssparser::{
    DeclarationListParser,
    AtRuleParser, RuleListParser,
    Parser, ParserInput, ParseError,
    QualifiedRuleParser,
    ToCss,
};
use bevy::log::{
    error,
};

/// Parses a `css` string using [`RuleListParser`].
pub(crate) struct StyleSheetParser;

impl StyleSheetParser {
    pub(crate) fn parse(content: &str) -> SmallVec<[StyleRule; 8]> {
        let mut input = ParserInput::new(content);
        let mut parser = Parser::new(&mut input);

        RuleListParser::new_for_stylesheet(&mut parser, StyleSheetParser)
            .into_iter()
            .filter_map(|result| match result {
                Ok(rule) => Some(rule),
                Err((err, rule)) => {
                    error!(
                        "Failed to parse rule: {}. Error: {}",
                        rule,
                        format_error(err)
                    );
                    None
                }
            })
            .collect()
    }
}

impl<'i> QualifiedRuleParser<'i> for StyleSheetParser {
    type Prelude = Selector;
    type QualifiedRule = StyleRule;
    type Error = BevyCssError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let mut elements = smallvec![];

        let mut next_is_class = false;

        while let Ok(token) = input.next_including_whitespace() {
            use tomt_cssparser::Token::*;
            match token {
                Ident(v) => {
                    if next_is_class {
                        next_is_class = false;
                        elements.push(SelectorElement::Class(v.to_string()));
                    } else {
                        elements.push(SelectorElement::Component(v.to_string()));
                    }
                }
                IDHash(v) => {
                    if v.is_empty() {
                        return Err(input.new_custom_error(BevyCssError::InvalidSelector));
                    } else {
                        elements.push(SelectorElement::Name(v.to_string()));
                    }
                }
                WhiteSpace(_) => elements.push(SelectorElement::Child),
                Delim(c) if *c == '.' => next_is_class = true,
                _ => {
                    let token = token.to_css_string();
                    return Err(input.new_custom_error(BevyCssError::UnexpectedToken(token)));
                }
            }
        }

        if elements.is_empty() {
            return Err(input.new_custom_error(BevyCssError::InvalidSelector));
        }

        // Remove noise the trailing white spaces, if any
        while !elements.is_empty() && elements.last().unwrap() == &SelectorElement::Child {
            elements.remove(elements.len() - 1);
        }

        Ok(Selector::new(elements))
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &tomt_cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut rule = StyleRule {
            selector: prelude,
            properties: Default::default(),
        };

        for property in DeclarationListParser::new(input, PropertyParser) {
            match property {
                Ok((name, property)) => {
                    rule.properties.insert(name, property);
                }
                Err((err, a)) => println!("Failed: {:?} ({})", err, a),
            }
        }

        Ok(rule)
    }
}

impl<'i> AtRuleParser<'i> for StyleSheetParser {
    type Prelude = ();
    type AtRule = StyleRule;
    type Error = BevyCssError;
}
