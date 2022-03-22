use crate::soql::identifier;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str;
use std::str::FromStr;

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
    FieldsAll,
    FieldsStandard,
    FieldsCustom,
    CountAll,
    Avg(Field),
    Count(Field),
    Count_Distinct(Field),
    Max(Field),
    Min(Field),
    Sum(Field),
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
/// 
/// https://developer.salesforce.com/docs/atlas.en-us.236.0.soql_sosl.meta/soql_sosl/sforce_api_calls_soql_select_fields.htm
pub fn field_fields_parser(i: &[u8]) -> IResult<&[u8], FunctionExpression> {
    alt((
        map(
            tag_no_case("FIELDS(ALL)"), 
            |_| { FunctionExpression::FieldsAll } 
        ), 
        map(
            tag_no_case("FIELDS(CUSTOM)"),
            |_| { FunctionExpression::FieldsCustom }
        ), 
        map(
            tag_no_case("FIELDS(STANDARD)"), 
        |_| { FunctionExpression::FieldsStandard }
        ), 
    ))(i)
}

pub fn field_parser(i: &[u8]) -> IResult<&[u8], Field> {
    map(
        tuple((many0(terminated(identifier, tag("."))), identifier)),
        |p| Field {
            name: str::from_utf8(p.1).unwrap().to_string(),
            object: match p.0.last() {
                None => None,
                Some(t) => Some(str::from_utf8(t).unwrap().to_string()),
            },
        },
    )(i)
}

/// Parse the function args
fn function_args_parser(i: &[u8]) -> IResult<&[u8], Option<&[u8]>> {
    delimited(
        tag("("),
        delimited(multispace0, opt(identifier), multispace0),
        tag(")"),
    )(i)
}

/// Parse the field funcitons
pub fn field_function_parser(i: &[u8]) -> IResult<&[u8], FunctionExpression> {
    alt((
        map(
            preceded(tag_no_case("count"), function_args_parser),
            |args| match args {
                None => FunctionExpression::CountAll,
                Some(id) => FunctionExpression::Count(Field {
                    name: str::from_utf8(id).unwrap().to_string(),
                    object: None,
                }),
            },
        ),
        map(preceded(tag_no_case("avg"), function_args_parser), |args| {
            FunctionExpression::Avg(Field {
                name: str::from_utf8(args.unwrap()).unwrap().to_string(),
                object: None,
            })
        }),
        map(preceded(tag_no_case("min"), function_args_parser), |args| {
            FunctionExpression::Min(Field {
                name: str::from_utf8(args.unwrap()).unwrap().to_string(),
                object: None,
            })
        }),
        map(preceded(tag_no_case("max"), function_args_parser), |args| {
            FunctionExpression::Max(Field {
                name: str::from_utf8(args.unwrap()).unwrap().to_string(),
                object: None,
            })
        }),
        map(preceded(tag_no_case("sum"), function_args_parser), |args| {
            FunctionExpression::Sum(Field {
                name: str::from_utf8(args.unwrap()).unwrap().to_string(),
                object: None,
            })
        }),
        map(
            preceded(tag_no_case("count_distinct"), function_args_parser),
            |args| {
                FunctionExpression::Count_Distinct(Field {
                    name: str::from_utf8(args.unwrap()).unwrap().to_string(),
                    object: None,
                })
            },
        ),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_parser() {
        assert_eq!(
            field_parser(b"Account.Name").unwrap(),
            (
                &b""[..],
                Field {
                    name: "Name".to_owned(),
                    object: Some("Account".to_owned())
                }
            )
        );

        assert_eq!(
            field_parser(b"Contact__r.Lastname").unwrap(),
            (
                &b""[..],
                Field {
                    name: "Lastname".to_owned(),
                    object: Some("Contact__r".to_owned())
                }
            )
        );

        assert_eq!(
            field_parser(b"FirstName ").unwrap(),
            (
                &b" "[..],
                Field {
                    name: "FirstName".to_owned(),
                    object: None
                }
            )
        );

        println!("{:?}", field_parser(b"Contact.Account.Name"));
        assert_eq!(
            field_parser(b"Contact.Account.Name").unwrap(),
            (
                &b""[..],
                Field {
                    name: "Name".to_owned(),
                    object: Some("Account".to_owned())
                }
            )
        );

        assert!(field_parser(b"Contact.  Account.Name").is_err());
        assert!(field_parser(b"Contact.\nAccount.Name").is_err());
    }

    #[test]
    fn test_function_parser() {
        assert_eq!(
            field_function_parser(b"count()").unwrap().1,
            FunctionExpression::CountAll
        );

        assert_eq!(
            field_function_parser(b"count( Name )").unwrap().1,
            FunctionExpression::Count(Field {
                name: "Name".to_owned(),
                object: None
            })
        );

        assert_eq!(
            field_function_parser(b"max(Total)").unwrap().1,
            FunctionExpression::Max(Field {
                name: "Total".to_owned(),
                object: None
            })
        );

        assert_eq!(
            field_function_parser(b"min(Total)").unwrap().1,
            FunctionExpression::Min(Field {
                name: "Total".to_owned(),
                object: None
            })
        );

        assert_eq!(
            field_function_parser(b"sum(Total)").unwrap().1,
            FunctionExpression::Sum(Field {
                name: "Total".to_owned(),
                object: None
            })
        );

        assert_eq!(
            field_function_parser(b"count_distinct(Total)").unwrap().1,
            FunctionExpression::Count_Distinct(Field {
                name: "Total".to_owned(),
                object: None
            })
        );
    }

    #[test]
    fn test_field_fields_function() {
        assert_eq!(field_fields_parser(b"FIELDS(STANDARD)").unwrap().1, FunctionExpression::FieldsStandard);
        assert_eq!(field_fields_parser(b"FIELDS(ALL)").unwrap().1, FunctionExpression::FieldsAll);
        assert_eq!(field_fields_parser(b"FIELDS(CUSTOM)").unwrap().1, FunctionExpression::FieldsCustom);

    }
}
