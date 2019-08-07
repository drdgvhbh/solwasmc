use crate::atom::{parse_anything_till_semi, parse_identifier, Atom};
use crate::delimiter::{parse_semicolon, Delimiter};
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::{named, tag, take_until1, take_while, ws, IResult};
use std::fmt;
use std::str::from_utf8;

const PRAGMA: &str = "pragma";
const INTERFACE: &str = "interface";
const LIBRARY: &str = "library";
const CONTRACT: &str = "contract";

pub fn parse_pragma_token_keyword(i: &[u8]) -> IResult<&[u8], Atom> {
  map(
    |b: &[u8]| tag!(b, PRAGMA),
    |b: &[u8]| Atom::Keyword(from_utf8(b).unwrap().to_string()),
  )(i)
}

pub fn parse_interface_keyword(i: &[u8]) -> IResult<&[u8], Atom> {
  map(
    |b: &[u8]| tag!(b, INTERFACE),
    |b: &[u8]| Atom::Keyword(from_utf8(b).unwrap().to_string()),
  )(i)
}

pub fn parse_library_keyword(i: &[u8]) -> IResult<&[u8], Atom> {
  map(
    |b: &[u8]| tag!(b, LIBRARY),
    |b: &[u8]| Atom::Keyword(from_utf8(b).unwrap().to_string()),
  )(i)
}

pub fn parse_contract_keyword(i: &[u8]) -> IResult<&[u8], Atom> {
  map(
    |b: &[u8]| tag!(b, CONTRACT),
    |b: &[u8]| Atom::Keyword(from_utf8(b).unwrap().to_string()),
  )(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn parses_pragma_token_keyword() {
    let input = format!("{} solidity ^0.5.6;", PRAGMA);
    assert_eq!(
      parse_pragma_token_keyword(input.as_bytes()).ok().unwrap(),
      (
        " solidity ^0.5.6;".as_bytes(),
        Atom::Keyword(PRAGMA.to_string())
      )
    )
  }

  #[test]
  fn parses_interface_keyword() {
    let input = format!("{} GeneralERC20 {{", INTERFACE);
    let (remaining, interface) = parse_interface_keyword(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), interface),
      (" GeneralERC20 {", Atom::Keyword(INTERFACE.to_string()))
    )
  }

  #[test]
  fn parses_library_keyword() {
    let input = format!("{} GeneralERC20 {{", LIBRARY);
    let (remaining, interface) = parse_library_keyword(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), interface),
      (" GeneralERC20 {", Atom::Keyword(LIBRARY.to_string()))
    )
  }

  #[test]
  fn parses_contract_keyword() {
    let input = format!("{} GeneralERC20 {{", CONTRACT);
    let (remaining, interface) = parse_contract_keyword(input.as_bytes()).ok().unwrap();
    assert_eq!(
      (from_utf8(remaining).unwrap(), interface),
      (" GeneralERC20 {", Atom::Keyword(CONTRACT.to_string()))
    )
  }
}
