use crate::{FunctionArgument, SectionMangler, StringEncoding, Type};

use std::str;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::{many0, many_m_n};
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

    let (input, _) = tag(crate::RUSTY_PREFIX)(input)?;
    let (input, prefix) = alt((fn_prefix, var_prefix))(input)?;

    Ok((input, prefix))
}

fn parse_entity_name(input: &str) -> ParseResult<&str> {
    delimited(char('-'), take_until1(":"), char(':'))(input)
}

fn type_void(input: &str) -> ParseResult<Type> {
    char('v').map(|_| Type::Void).parse(input)
}

fn number<T: str::FromStr>(input: &str) -> ParseResult<T> {
    map_res(digit1, str::parse)(input)
}

fn type_integer(input: &str) -> ParseResult<Type> {
    fn parse_signedness(input: &str) -> ParseResult<bool> {
        let signed = char('i').map(|_| true);
        let unsigned = char('u').map(|_| false);

        alt((signed, unsigned))(input)
    }

    parse_signedness
        .and(number::<u32>)
        .map(|(signed, size)| Type::Integer { signed, size, semantic_size: None })
        .parse(input)
}

fn type_float(input: &str) -> ParseResult<Type> {
    char('f').and(number::<u32>).map(|(_, size)| Type::Float { size }).parse(input)
}

fn type_pointer(input: &str) -> ParseResult<Type> {
    char('p').and(parse_type).map(|(_, inner)| Type::Pointer { inner: Box::new(inner) }).parse(input)
}

fn type_struct(input: &str) -> ParseResult<Type> {
    let (input, (_, n)) = char('r').and(number::<usize>).parse(input)?;

    many_m_n(n, n, parse_type).map(|members| Type::Struct { members }).parse(input)
}

fn type_enum(input: &str) -> ParseResult<Type> {
    char('e')
        .and(number::<usize>)
        .and(parse_type)
        .map(|((_, elements), ty)| Type::Enum { referenced_type: Box::new(ty), elements })
        .parse(input)
}

fn string_encoding(input: &str) -> ParseResult<StringEncoding> {
    let utf8 = tag("8u").map(|_| StringEncoding::Utf8);
    let utf16 = tag("16u").map(|_| StringEncoding::Utf16);

    alt((utf8, utf16))(input)
}

fn type_string(input: &str) -> ParseResult<Type> {
    char('s')
        .and(string_encoding)
        .and(number::<usize>)
        .map(|((_, encoding), size)| Type::String { size, encoding })
        .parse(input)
}

fn type_array(input: &str) -> ParseResult<Type> {
    char('a').and(parse_type).map(|(_, inner_ty)| Type::Array { inner: Box::new(inner_ty) }).parse(input)
}

fn parse_type(input: &str) -> ParseResult<Type> {
    alt((type_void, type_integer, type_float, type_pointer, type_struct, type_enum, type_string, type_array))(
        input,
    )
}

fn parse_var_content<'i>(input: &'i str, name: &str) -> ParseResult<'i, SectionMangler> {
    let (input, ty) = parse_type(input)?;

    Ok((input, SectionMangler::variable(name, ty)))
}

fn parse_fn_content<'i>(input: &'i str, name: &str) -> ParseResult<'i, SectionMangler> {
    let (input, return_type) = parse_type(input)?;
    let (input, parameters) = delimited(char('['), many0(parse_type), char(']'))(input)?;

    // TODO: Do not always encode parameters as ByValue
    let mangler = parameters
        .into_iter()
        .fold(SectionMangler::function(name).with_return_type(return_type), |mangler, param| {
            mangler.with_parameter(FunctionArgument::ByValue(param))
        });

    // TODO: Would it be better for the function to encode the number of arguments it has?
    // or just parse what is in between `[]` like we do currently?

    Ok((input, mangler))
}

// We don't need to handle any kind of errors, because an invalid mangled string can only be
// caused by a programming error or a mismatch in versions
impl From<&str> for SectionMangler {
    fn from(input: &str) -> SectionMangler {
        let (input, prefix) = parse_prefix(input).unwrap();
        let (input, name) = parse_entity_name(input).unwrap();

        match prefix {
            Prefix::Var => parse_var_content(input, name).unwrap().1,
            Prefix::Fn => parse_fn_content(input, name).unwrap().1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prefix_valid() {
        assert!(parse_prefix("$RUSTY$fn").is_ok());
        assert!(parse_prefix("$RUSTY$var").is_ok());

        assert!(parse_prefix("$RUSTY$random").is_err());
        assert!(parse_prefix("fn").is_err());
        assert!(parse_prefix("var").is_err());

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

    #[test]
    fn parse_struct() {
        assert!(type_struct("r1u8").is_ok());
        assert!(type_struct("r1r2u8u49").is_ok());
        assert!(type_struct("r5pu8r1u8u32u32pv").is_ok());

        // these are fine - we parse a struct which is valid, but we have remaining input.
        // this needs to be handled by the toplevel parse function
        assert!(type_struct("r0u8u8").is_ok());
        assert!(type_struct("r1u8u8").is_ok());
        assert!(type_struct("r5s8u1025s8u2049s8u3u64s8u3").is_ok());

        // invalid number of elements
        assert!(type_struct("r15u8").is_err());
        assert!(type_struct("r1").is_err());
        assert!(type_struct("r2r1u8").is_err());
    }

    #[test]
    fn parse_enum() {
        assert!(type_enum("e15u8").is_ok());
        assert!(type_enum("e12pv").is_ok());

        assert!(type_enum("e1").is_err());
        assert!(type_enum("eu8").is_err());
    }

    #[test]
    fn parse_variable() {
        let _ = SectionMangler::from("$RUSTY$var-name:u8");
    }

    #[test]
    fn parse_function() {
        let _ = SectionMangler::from("$RUSTY$fn-foo:u8[]");
        let _ = SectionMangler::from("$RUSTY$fn-foo:v[]");
        let _ = SectionMangler::from("$RUSTY$fn-foo:v[pvu8]");
        let _ = SectionMangler::from("$RUSTY$fn-foo:e156u394[pvu8r1e12u8]");
    }

    #[test]
    #[should_panic]
    fn parse_function_invalid_no_return_type() {
        let _ = SectionMangler::from("$RUSTY$fn-no_return_type:[]");
    }

    #[test]
    #[should_panic]
    fn parse_function_invalid_no_arguments() {
        let _ = SectionMangler::from("$RUSTY$fn-no_arguments:u16u8");
    }

    #[test]
    fn parse_qualified_var_name() {
        let mangled = SectionMangler::from("$RUSTY$var-Color.red:e4i32");

        assert_eq!(mangled.name(), "Color.red");
    }

    #[test]
    fn parse_complex1() {
        let mangled = SectionMangler::from("$RUSTY$var-__File__init:r5s8u1025s8u2049s8u3u64s8u3");

        assert_eq!(mangled.name(), "__File__init");
    }

    #[test]
    fn parse_complex2() {
        let inputs = [
            "$RUSTY$var-__CosineSignal__init:r4f64i32f64f64",
            "$RUSTY$var-__File__init:r5s8u1025s8u2049s8u3u64s8u3",
            "$RUSTY$var-__SineSignal__init:r4f64i32f64f64",
            "$RUSTY$var-__mainProg_state.Init:e3i32",
            "$RUSTY$var-__mainProg_state.Running:e3i32",
            "$RUSTY$var-__mainProg_state.Stopped:e3i32",
            "$RUSTY$var-__SR__init:r3u8u8u8",
            "$RUSTY$var-__RS__init:r3u8u8u8",
            "$RUSTY$var-__CTU__init:r6u8u8i16u8i16u8",
            "$RUSTY$var-__CTU_INT__init:r6u8u8i16u8i16u8",
            "$RUSTY$var-__CTU_DINT__init:r6u8u8i32u8i32u8",
            "$RUSTY$var-__CTU_UDINT__init:r6u8u8u32u8u32u8",
            "$RUSTY$var-__CTU_LINT__init:r6u8u8i64u8i64u8",
            "$RUSTY$var-__CTU_ULINT__init:r6u8u8u64u8u64u8",
            "$RUSTY$var-__CTD__init:r6u8u8i16u8i16u8",
            "$RUSTY$var-__CTD_INT__init:r6u8u8i16u8i16u8",
            "$RUSTY$var-__CTD_DINT__init:r6u8u8i32u8i32u8",
            "$RUSTY$var-__CTD_UDINT__init:r6u8u8u32u8u32u8",
            "$RUSTY$var-__CTD_LINT__init:r6u8u8i64u8i64u8",
            "$RUSTY$var-__CTD_ULINT__init:r6u8u8u64u8u64u8",
            "$RUSTY$var-__CTUD__init:r10u8u8u8u8i16u8u8i16u8u8",
            "$RUSTY$var-__CTUD_INT__init:r10u8u8u8u8i16u8u8i16u8u8",
            "$RUSTY$var-__CTUD_DINT__init:r10u8u8u8u8i32u8u8i32u8u8",
            "$RUSTY$var-__CTUD_UDINT__init:r10u8u8u8u8u32u8u8u32u8u8",
            "$RUSTY$var-__CTUD_LINT__init:r10u8u8u8u8i64u8u8i64u8u8",
            "$RUSTY$var-__CTUD_ULINT__init:r10u8u8u8u8u64u8u8u64u8u8",
            "$RUSTY$var-__R_TRIG__init:r3u8u8u8",
            "$RUSTY$var-__F_TRIG__init:r3u8u8u8",
            "$RUSTY$var-__TP__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TP_TIME__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TP_LTIME__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TON__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TON_TIME__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TON_LTIME__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TOF__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TOF_TIME__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$var-__TOF_LTIME__init:r7u8i64u8i64u8u8au8",
            "$RUSTY$fn-CosineSignal:v[f64][i32][f64]",
            "$RUSTY$fn-File:v[s8u1025][s8u2049][s8u3]",
            "$RUSTY$fn-File.Open:v",
            "$RUSTY$fn-File.Write:v",
            "$RUSTY$fn-File.Close:v",
            "$RUSTY$fn-File.Clear:v",
            "$RUSTY$fn-SineSignal:v[f64][i32][f64]",
            "$RUSTY$fn-mainProg:v",
            "$RUSTY$var-mainProg:r7i32r4f64i32f64f64r4f64i32f64f64e3i32r5s8u1025s8u2049s8u3u64s8u3r5s8u1025s8u2049s8u3u64s8u3r5s8u1025s8u2049s8u3u64s8u3"
        ];

        inputs.into_iter().for_each(|input| {
            let _ = SectionMangler::from(dbg!(input));
        });
    }
}
