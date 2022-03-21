use std::fmt;
use std::fmt::{Display};
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{alphanumeric0, digit1, line_ending, multispace0, multispace1};
use nom::character::is_alphanumeric;
use nom::combinator::{peek, eof, not, map, opt};
use nom::multi::many0;
use nom::sequence::{terminated, preceded, tuple};
use serde_derive::{Serialize, Deserialize};
use std::str;
use std::str::FromStr;


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
fn keyword(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        keywords_split_1,
        keywords_split_2,
    ))(i)
}

// An indentifier is a literal which is not keyword
fn identifier(i: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        preceded(not(peek(keyword)), take_while1(is_valid_identifier)),
    ))(i)
}

pub fn is_valid_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8 || chr == '@' as u8
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub object: Option<String>,
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}


#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FunctionExpression {
    AVG(Field),
    COUNT(Field),
    COUNT_DISTINCT(Field),
    MAX(Field),
    MIN(Field),
    SUM(Field),
}

impl Display for FunctionExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldExpression {
    Col(Field),
}

impl Display for FieldExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
        // match *self {
        //     FieldExpression::Col(ref col) => write!(f, "{}", col)
        // }
    }
}

/// https://developer.salesforce.com/docs/atlas.en-us.soql_sosl.meta/soql_sosl/sforce_api_calls_soql_select.htm
/// Specifies a list of one or more fields, separated by commas, that you want to retrieve from the specified object. The bold elements in the following examples are fieldlist values:

///     SELECT Id, Name, BillingCity FROM Account
///     SELECT count() FROM Contact
///     SELECT Contact.Firstname, Contact.Account.Name FROM Contact
///     SELECT FIELDS(STANDARD) FROM Contact

pub fn field_parser(i: &[u8]) -> IResult<&[u8], Field> {
    
    map(
        tuple((
            many0(terminated(identifier, tag("."))), 
            identifier
        )), 
        |p| Field {
            name: str::from_utf8(p.1).unwrap().to_string(),
            object: match p.0.last() {
                None => None,
                Some(t) => Some(str::from_utf8(t).unwrap().to_string()),
            }
        }
    )(i)
}

pub fn field_function_parser(i: &[u8]) -> IResult<&[u8], Field> {

    
    todo!()
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
        assert!(
            keyword(&b"INVALID_KEYWORD"[..]).is_err()
        );
    }

    #[test]
    fn test_identifier() {
        // an valid keyword
        assert!(
            identifier("Name \t".as_bytes()).is_ok()
        );

        // not an valid identi because it is a keyword
        assert!(
            identifier("select ".as_bytes()).is_err()
        );
    }

    #[test]
    fn test_field_parser() {

        assert_eq!(field_parser(b"Account.Name").unwrap(), (&b""[..],Field { name: "Name".to_owned(), object: Some("Account".to_owned()) }));

        assert_eq!(field_parser(b"Contact__r.Lastname").unwrap(), (&b""[..],Field { name: "Lastname".to_owned(), object: Some("Contact__r".to_owned()) }));

        assert_eq!(field_parser(b"FirstName ").unwrap(), (&b" "[..],Field { name: "FirstName".to_owned(), object: None }));

        println!("{:?}", field_parser(b"Contact.Account.Name"));
        assert_eq!(field_parser(b"Contact.Account.Name").unwrap(), (&b""[..],Field { name: "Name".to_owned(), object: Some("Account".to_owned()) }));

        assert!(field_parser(b"Contact.  Account.Name").is_err());
        assert!(field_parser(b"Contact.\nAccount.Name").is_err());
    }

    #[test]
    fn test() {
        let res = many0(terminated(identifier, tag(".")))("Contact.Account.Name".as_bytes()).unwrap();
        println!("{:?}", str::from_utf8(res.1[0].clone()).unwrap().to_string());
        println!("{:?}", str::from_utf8(res.0.clone()).unwrap().to_string());
    }
}