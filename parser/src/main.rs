extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{alpha1, alphanumeric1},
    character::is_alphanumeric,
    multi::{many0, many1},
    sequence::delimited,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct OwnerQualifiedDependency<'a> {
    owner: &'a str,
    bale: &'a str,
    module: &'a str,
}

#[derive(Debug)]
pub struct BaleQualifiedDependency<'a> {
    bale: &'a str,
    module: &'a str,
}

#[derive(Debug)]
pub struct ModuleQualifiedDependency<'a> {
    module: &'a str,
}

#[derive(Debug)]
pub enum DependancySpecifier<'a> {
    Owner(OwnerQualifiedDependency<'a>),
    Bale(BaleQualifiedDependency<'a>),
    Module(ModuleQualifiedDependency<'a>),
}

#[derive(Debug)]
pub struct Identifier<'a>(&'a str);

#[derive(Debug)]
pub struct UseDependencyStatement<'a> {
    dependancy_specifier: DependancySpecifier<'a>,
    imported_objects: Vec<Identifier<'a>>,
}

fn parse_between_brackets(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn parse_between_braces(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}

fn parse_dependancy_specifier_section(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn parse_dependancy_import(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_owner_qualified_dependency(input: &str) -> IResult<&str, DependancySpecifier> {
    let (input, (_, owner, _, _, bale, _, module)) = tuple((
        char('@'),
        parse_dependancy_specifier_section,
        char('/'),
        char('~'),
        parse_dependancy_specifier_section,
        char('/'),
        parse_dependancy_specifier_section,
    ))(input)?;

    return Ok((
        input,
        DependancySpecifier::Owner(OwnerQualifiedDependency {
            owner,
            bale,
            module,
        }),
    ));
}

fn parse_bale_qualified_dependency(input: &str) -> IResult<&str, DependancySpecifier> {
    let (input, (_, bale, _, module)) = tuple((
        char('~'),
        parse_dependancy_specifier_section,
        char('/'),
        parse_dependancy_specifier_section,
    ))(input)?;

    return Ok((
        input,
        DependancySpecifier::Bale(BaleQualifiedDependency { bale, module }),
    ));
}

fn parse_module_qualified_dependency(input: &str) -> IResult<&str, DependancySpecifier> {
    let (input, (module)) = parse_dependancy_specifier_section(input)?;

    return Ok((
        input,
        DependancySpecifier::Module(ModuleQualifiedDependency { module }),
    ));
}

fn parse_dependancy_specifier(input: &str) -> IResult<&str, DependancySpecifier> {
    let (input, dependancy_specifier_source) = parse_between_brackets(input)?;
    let (_, output) = alt((
        parse_owner_qualified_dependency,
        parse_bale_qualified_dependency,
        parse_module_qualified_dependency,
    ))(dependancy_specifier_source)?;

    Ok((input, output))
}

fn parse_dependancy_imports(input: &str) -> IResult<&str, Vec<Identifier>> {
    let (input, imports) = parse_between_braces(input)?;
    let (_, imports) = many1(parse_dependancy_import)(imports)?;

    Ok((input, imports.into_iter().map(Identifier).collect()))
}

fn parse_use_dependency(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("use")(input)?;
    let (input, dependancy_specifier) = parse_dependancy_specifier(input)?;
    let (input, imports) = parse_dependancy_imports(input)?;

    dbg!(&(input, dependancy_specifier, imports));

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_parse() {
        let source = "use(@std/~style/color){ Color };";

        //include_str!("../../examples/src/theme.ui");
        dbg!(&source);
        let parsed = parse_use_dependency(&source);
        dbg!(&parsed);
        assert!(parsed.is_ok());
    }
}

fn main() {
    println!("Hello, world!");
}
