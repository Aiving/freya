use torin::alignment::Alignment;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for Alignment {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "start" => Some(Self::Start),
                "center" => Some(Self::Center),
                "end" => Some(Self::End),
                "space-between" => Some(Self::SpaceBetween),
                "space-evenly" => Some(Self::SpaceEvenly),
                "space-around" => Some(Self::SpaceAround),
                _ => None,
            })
        })
    }
}
