use super::{
    format_error,
    PropertyParser,
};
use crate::{
    DynArray,
    prelude::BevyCssError,
    selector::{Selector, SelectorElement},
    stylesheet::StyleRule,
};

use bevy::log::error;
use cssparser::{
    AtRuleParser,
    ParseError, Parser, ParserInput,
    QualifiedRuleParser,
    RuleBodyParser,
    ToCss,
};

/// Parses a `css` string using [`StyleSheetParser`].
pub(crate) struct StyleSheetParser;

impl StyleSheetParser
{
    pub(crate) fn parse(
        content: &str
    ) -> DynArray<StyleRule> {
        let mut input = ParserInput::new(content);
        let mut parser = Parser::new(&mut input);

        cssparser::StyleSheetParser::new(&mut parser, &mut StyleSheetParser)
            .filter_map(|result| match result
            {
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

impl<'i> QualifiedRuleParser<'i>
for StyleSheetParser
{
    type Prelude = Selector;
    type QualifiedRule = StyleRule;
    type Error = BevyCssError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let mut elements = DynArray::new();

        #[derive(Debug, Default, Clone)]
        enum DelimType
        {
            #[default]
            None,
            Class,
            #[cfg(feature = "pseudo_class")]
            PseudoClass,
            #[cfg(feature = "pseudo_prop")]
            PseudoProp,
        }

        let mut prev_delim = DelimType::None;

        while let Ok(token) = input.next_including_whitespace() {
            use cssparser::Token::*;

            match token {
                Ident(v) => elements.push(match prev_delim
                {
                    DelimType::None => {
                        prev_delim = DelimType::None;
                        SelectorElement::Component(v.to_string())
                    }

                    DelimType::Class => {
                        prev_delim = DelimType::None;
                        SelectorElement::Class(v.to_string())
                    }

                    #[cfg(feature = "pseudo_class")]
                    DelimType::PseudoClass => {
                        prev_delim = DelimType::None;
                        SelectorElement::PseudoClass(v.to_string())
                    }

                    #[cfg(feature = "pseudo_prop")]
                    DelimType::PseudoProp => {
                        let err_str = format!(":{v}");
                        return Err(
                            input.new_custom_error(BevyCssError::UnexpectedToken(err_str))
                        );
                    }
                }),

                IDHash(v) => match v.is_empty()
                {
                    true => return Err(input.new_custom_error(BevyCssError::InvalidSelector)),
                    false => elements.push(SelectorElement::Name(v.to_string())),
                }
                
                WhiteSpace(_) => elements.push(SelectorElement::Child),

                Delim(c) => prev_delim = match (*c, prev_delim)
                {
                    ('.', DelimType::None) => DelimType::Class,
                    _ => {
                        let err_str = token.to_css_string();
                        return Err(
                            input.new_custom_error(BevyCssError::UnexpectedToken(err_str))
                        );
                    }
                },

                #[cfg(feature = "pseudo_class")]
                Colon => prev_delim = match prev_delim
                {
                    DelimType::None => DelimType::PseudoClass,

                    #[cfg(feature = "pseudo_prop")]
                    DelimType::PseudoClass => DelimType::PseudoProp,

                    _ => {
                        let err_str = token.to_css_string();
                        return Err(
                            input.new_custom_error(BevyCssError::UnexpectedToken(err_str))
                        );
                    }
                },

                _ => {
                    let token = token.to_css_string();
                    return Err(input.new_custom_error(BevyCssError::UnexpectedToken(token)));
                }
            }
        }

        if elements.is_empty()
        {
            return Err(input.new_custom_error(BevyCssError::InvalidSelector));
        }

        // Remove noise the trailing white spaces, if any
        while !elements.is_empty() && elements.last().unwrap() == &SelectorElement::Child
        {
            elements.remove(elements.len() - 1);
        }

        Ok(Selector::new(elements))
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut rule = StyleRule::new(prelude);

        for property in RuleBodyParser::new(input, &mut PropertyParser)
        {
            match property
            {
                Ok((name, property)) => {
                    rule.properties.insert(name, property);
                }
                Err((err, a)) => println!("Failed: {:?} ({})", err, a),
            }
        }

        Ok(rule)
    }
}

impl<'i> AtRuleParser<'i>
for StyleSheetParser
{
    type Prelude = ();
    type AtRule = StyleRule;
    type Error = BevyCssError;
}
