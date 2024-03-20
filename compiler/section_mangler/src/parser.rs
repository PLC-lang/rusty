use crate::{SectionMangler, Type};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::sequence::delimited;
use nom::{IResult, Parser};

type ParseResult<'i, O> = IResult<&'i str, O>;

enum Prefix {
    Fn,
    Var,
}

fn parse_prefix(input: &str) -> ParseResult<Prefix> {
    let fn_prefix = tag("fn").map(|_| Prefix::Fn);
    let var_prefix = tag("var").map(|_| Prefix::Var);

    let (input, prefix) = alt((fn_prefix, var_prefix))(input)?;

    Ok((input, prefix))
}

fn parse_entity_name(input: &str) -> ParseResult<&str> {
    delimited(char('-'), take_until1(":"), char(':'))(input)
}

fn type_void(input: &str) -> ParseResult<Type> {
    char('v').map(|_| Type::Void).parse(input)
}

fn take_u32(input: &str) -> ParseResult<u32> {
    map_res(digit1, str::parse)(input)
}

fn type_integer(input: &str) -> ParseResult<Type> {
    fn parse_signedness(input: &str) -> ParseResult<bool> {
        let signed = char('i').map(|_| true);
        let unsigned = char('u').map(|_| false);

        alt((signed, unsigned))(input)
    }

    parse_signedness
        .and(take_u32)
        .map(|(signed, size)| Type::Integer { signed, size, semantic_size: None })
        .parse(input)
}

fn type_float(input: &str) -> ParseResult<Type> {
    char('f').and(take_u32).map(|(_, size)| Type::Float { size }).parse(input)
}

fn type_pointer(input: &str) -> ParseResult<Type> {
    char('p').and(parse_type).map(|(_, inner)| Type::Pointer { inner: Box::new(inner) }).parse(input)
}

fn parse_type(input: &str) -> ParseResult<Type> {
    alt((type_void, type_integer, type_float, type_pointer))(input)
}

fn parse_var_content<'i>(input: &'i str, name: &str) -> ParseResult<'i, SectionMangler> {
    let (input, ty) = parse_type(input)?;

    Ok((input, SectionMangler::variable(name, ty)))
}

// We don't need to handle any kind of errors, because an invalid mangled string can only be
// caused by a programming error or a mismatch in versions
impl From<&str> for SectionMangler {
    fn from(input: &str) -> SectionMangler {
        let (input, prefix) = parse_prefix(input).unwrap();
        let (input, name) = parse_entity_name(input).unwrap();

        match prefix {
            Prefix::Var => parse_var_content(input, name).unwrap().1,
            Prefix::Fn => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prefix_valid() {
        assert!(parse_prefix("fn").is_ok());
        assert!(parse_prefix("var").is_ok());

        assert!(parse_prefix("").is_err());
        assert!(parse_prefix("a random prefix").is_err());
    }

    #[test]
    fn parse_name_valid() {
        assert_eq!(parse_entity_name("-foo:").unwrap().1, "foo");

        // empty name
        assert!(parse_entity_name("-:").is_err());
    }

    #[test]
    fn parse_integer() {
        assert!(type_integer("i15").is_ok());
        assert!(type_integer("u49").is_ok());
        assert!(type_integer("i0").is_ok());

        assert!(type_integer("i").is_err());
        assert!(type_integer("b49").is_err());
    }

    #[test]
    fn parse_void() {
        assert!(type_void("v").is_ok());

        assert!(type_void("i15").is_err());
    }

    #[test]
    fn parse_float() {
        assert!(type_float("f15").is_ok());
        assert!(type_float("f2560").is_ok());

        assert!(type_float("f").is_err());
        assert!(type_float("i0").is_err());
    }

    #[test]
    fn parse_ptr() {
        assert!(type_pointer("pf15").is_ok());
        assert!(type_pointer("pv").is_ok());
        assert!(type_pointer("pi45").is_ok());
        assert!(type_pointer("pppppppi45").is_ok());

        assert!(type_pointer("p").is_err());
        assert!(type_pointer("i0").is_err());
    }
}
