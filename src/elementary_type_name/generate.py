def snake_to_upper_camel(s):
    words = s.split("_")
    return "".join([word.lower().capitalize() for word in words])


print("// GENERATED: DO NOT EDIT")
print("""use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map, peek},
    sequence::{terminated, tuple},
    IResult,
};
""")
with open('elementary_type_names.txt') as f:
    lines = [line.rstrip() for line in f]
    for line in lines:
        print(f"const {line.upper()}: &str = r#\"{line}\"#;")
    print("")
    print("""#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ElementaryTypeName {""")
    for line in lines:
        print(f"\t{snake_to_upper_camel(line)},")
    print("}")
    print("")
    for line in lines:

        print(f"""fn parse_{line.lower()}(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {{
    map(
        tuple((
            peek(terminated(tag({line.upper()}), one_of(" \\n\\r)"))),
            tag({line.upper()}),
        )),
        |_| ElementaryTypeName::{snake_to_upper_camel(line)},
    )(i)
}}""")
    print(
        "pub fn parse(i: &[u8]) -> IResult<&[u8], ElementaryTypeName> {")
    print("\talt((")
    for line in lines:
        print(f"\t\tparse_{line.lower()},")
    print("\t))(i)")
    print("}")

    print("")
    print("#[cfg(test)]")
    print(
        "mod tests {\n\tuse super::*;\n\tuse std::str::from_utf8;\n")
    for line in lines:
        print(f"""\t#[test]
\tfn parses_{line.lower()}() {{
\t    let input = r#"{line} a"#;
\t    let (remaining, name) = parse_{line.lower()}(input.as_bytes()).ok().unwrap();
\t    assert_eq!(
\t        (from_utf8(remaining).unwrap(), name),
\t        (" a", ElementaryTypeName::{snake_to_upper_camel(line)}))
\t}}""")
print("}")
