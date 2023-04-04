use super::{
    smallvec, SmallVec,
    format_error,
    PropertyParser
};
use crate::{
    prelude::BevyCssError,
    Selector, SelectorElement,
    StyleRule,
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
    trace,
};

/// Parses a `css` string using [`RuleListParser`].
pub(crate) struct StyleSheetParser;

impl StyleSheetParser {
    pub(crate) fn parse(
        content: &str
    ) -> SmallVec<[StyleRule; 8]> {
        trace!("StyleSheetParser::parse");
        let mut input = ParserInput::new(content);
        let mut parser = Parser::new(&mut input);

        RuleListParser::new_for_stylesheet(&mut parser, StyleSheetParser)
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
        trace!("QualifiedRuleParser::parse_prelude");
        let mut elements = smallvec![];

        #[derive(Debug, Default, Clone)]
        enum DelimType {
            #[default] None,
            Class,
            #[cfg(feature = "pseudo_class")]
            PseudoClass,
            #[cfg(feature = "pseudo_prop")]
            PseudoProp,
        }

        let mut prev_delim = DelimType::None;

        while let Ok(token) = input.next_including_whitespace() {
            use tomt_cssparser::Token::*;

            match token {
                Ident(v) => {
                    elements.push(match prev_delim {
                        DelimType::None => {
                            prev_delim = DelimType::None;
                            SelectorElement::Component(v.to_string())
                        },
                        DelimType::Class => {
                            prev_delim = DelimType::None;
                            SelectorElement::Class(v.to_string())
                        },
                        #[cfg(feature = "pseudo_class")]
                        DelimType::PseudoClass => {
                            prev_delim = DelimType::None;
                            SelectorElement::PseudoClass(v.to_string())
                        },
                        #[cfg(feature = "pseudo_prop")]
                        DelimType::PseudoProp => {
                            let err_str = format!(":{v}");
                            return Err(input.new_custom_error(BevyCssError::UnexpectedToken(err_str)));
                        },
                    });
                }
                IDHash(v) => {
                    if v.is_empty() {
                        return Err(input.new_custom_error(BevyCssError::InvalidSelector));
                    } else {
                        elements.push(SelectorElement::Name(v.to_string()));
                    }
                }
                WhiteSpace(_) => elements.push(SelectorElement::Child),
                Delim(c) => {
                    prev_delim = match (*c, prev_delim) {
                        ('.', DelimType::None) => DelimType::Class,
                        _ => {
                            let err_str = token.to_css_string();
                            return Err(input.new_custom_error(BevyCssError::UnexpectedToken(err_str)));
                        }
                    };
                },
                #[cfg(feature = "pseudo_class")]
                Colon => {
                    prev_delim = match prev_delim {
                        DelimType::None => DelimType::PseudoClass,
                        #[cfg(feature = "pseudo_prop")]
                        DelimType::PseudoClass => DelimType::PseudoProp,
                        _ => {
                            let err_str = token.to_css_string();
                            return Err(input.new_custom_error(BevyCssError::UnexpectedToken(err_str)));
                        },
                    };
                }
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
        trace!("QualifiedRuleParser::parse_block");
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
