use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::combinator::{eof, peek};
use nom::sequence::terminated;
use nom::IResult;

// Below characters are followed by the keyword
fn keyword_following_char(i: &[u8]) -> IResult<&[u8], &[u8]> {
    peek(alt((
        tag(" "),
        tag("\n"),
        tag(";"),
        tag("("),
        tag(")"),
        tag("\t"),
        tag(","),
        tag("="),
        eof,
    )))(i)
}

//  Note: alt only supports for tuples of up to 21 elements
//  change the &str to &u[8] due to some
fn keywords_split_1(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        terminated(tag_no_case("SELECT"), keyword_following_char),
        terminated(tag_no_case("FROM"), keyword_following_char),
        terminated(tag_no_case("WHERE"), keyword_following_char),
        terminated(tag_no_case("GROUP BY"), keyword_following_char),
        terminated(tag_no_case("ORDER BY"), keyword_following_char),
        terminated(tag_no_case("LIMIT"), keyword_following_char),
        terminated(tag_no_case("OFFSET"), keyword_following_char),
        terminated(tag_no_case("CASE"), keyword_following_char),
    ))(i)
}

//  Note: alt only supports for tuples of up to 21 elements
//  change the &str to &u[8] due to some
fn keywords_split_2(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        terminated(tag_no_case("AND"), keyword_following_char),
        terminated(tag_no_case("OR"), keyword_following_char),
        terminated(tag_no_case("ASC"), keyword_following_char),
        terminated(tag_no_case("DESC"), keyword_following_char),
    ))(i)
}

// Note: alt only supports for tuples of up to 21 elements
// Consider to split the keywords into multiple different functions if the parsers are more than 21
pub fn keyword(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((keywords_split_1, keywords_split_2))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keyword() {
        // an valid keyword
        assert_eq!(
            keyword("GROUP BY \t".as_bytes()),
            Ok((&b" \t"[..], &b"GROUP BY"[..]))
        );
        // an invalid keyword
        assert!(keyword(&b"INVALID_KEYWORD"[..]).is_err());
    }
}
