use crate::keywords::keyword;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::is_alphanumeric;
use nom::combinator::{eof, map, not, opt, peek};
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use nom::IResult;

// An indentifier is a literal which is not keyword
pub fn identifier(i: &[u8]) -> IResult<&[u8], &[u8]> {
    // Don't check spaces here
    preceded(not(peek(keyword)), take_while1(is_valid_identifier))(i)
    // delimited(
    //     multispace0,
    //     preceded(not(peek(keyword)), take_while1(is_valid_identifier)),
    //     multispace0,
    // )(i)
}

pub fn is_valid_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8 || chr == '@' as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        // an valid keyword
        assert!(identifier("Name \t".as_bytes()).is_ok());
        // not an valid identi because it is a keyword
        assert!(identifier("select ".as_bytes()).is_err());
    }
}
