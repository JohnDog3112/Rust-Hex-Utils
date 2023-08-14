use hex_renderer::{
    self,
    pattern_utils::{Angle, Direction},
    Pattern,
};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"

WHITESPACE = _{" "}

hex = {SOI ~ iota ~ ("," ~ iota)* ~ EOI}
iota = _{pattern | number | vector | unknown | empty}

empty = _{&"," | EOI}

unknown = {(!"," ~ ANY)+}

vector = {"(" ~ number ~ ("," ~ number){2} ~ ")"}

number = @{"-"? ~ digit+ ~ (("." ~ digit+) | !".")}
digit = _{'0'..'9'}

pattern = ${pattern1 | pattern2 | (direction ~ ((WHITESPACE+ ~ angles) | (WHITESPACE* ~ (&"," | EOI))))}
pattern1 = _{"<" ~ WHITESPACE* ~ pattern12_inner ~ WHITESPACE* ~ ">"}
pattern2 = _{^"HexPattern(" ~ WHITESPACE* ~ pattern12_inner ~ WHITESPACE* ~ ")"}
pattern12_inner = _{direction ~ WHITESPACE* ~ angles? ~ (("," ~ WHITESPACE* ~ angles) | !",")}

direction = _{north_east | east | south_east | south_west | west | north_west}
north_east = {^"north_east" | ^"northeast" | ^"ne"}
east = {^"east" | ^"e"}
south_east = {^"south_east" | ^"southeast" | ^"se"}
south_west = {^"south_west" | ^"soutwest" | ^"sw"}
west = {^"west" | ^"w"}
north_west = {^"north_west" | ^"northwest" | ^"nw"}

angles = ${angle+}
angle = _{w | e | d | s | a | q}
w = {^"w"}
e = {^"e"}
d = {^"d"}
s = {^"s"}
a = {^"a"}
q = {^"q"}

"#]
struct PatternParser;

#[derive(Clone)]
pub struct PatternParseResults {
    pub valid: Vec<Pattern>,
    pub invalid: Vec<String>,
}

pub fn parse_str(patterns_str: &str) -> PatternParseResults {
    let parsed_patterns = PatternParser::parse(Rule::hex, patterns_str)
        .unwrap()
        .next()
        .unwrap();

    let mut errors = vec![];
    let patterns = parsed_patterns
        .into_inner()
        .filter_map(|rule| {
            if matches!(rule.as_rule(), Rule::pattern) {
                let mut innards = rule.into_inner();
                let direction = match innards.next().unwrap().as_rule() {
                    Rule::north_east => Direction::NorthEast,
                    Rule::east => Direction::East,
                    Rule::south_east => Direction::SouthEast,
                    Rule::south_west => Direction::SouthWest,
                    Rule::west => Direction::West,
                    Rule::north_west => Direction::NorthWest,
                    _ => unreachable!(),
                };
                let angles: Vec<Angle> = if let Some(angles) = innards.next() {
                    angles
                        .into_inner()
                        .map(|a| match a.as_rule() {
                            Rule::w => Angle::Forward,
                            Rule::e => Angle::Right,
                            Rule::d => Angle::BackRight,
                            Rule::s => Angle::Back,
                            Rule::a => Angle::BackLeft,
                            Rule::q => Angle::Left,
                            _ => unreachable!(),
                        })
                        .collect()
                } else {
                    vec![]
                };
                Some(Pattern::new(direction, angles))
            } else if !matches!(rule.as_rule(), Rule::EOI) && !matches!(rule.as_rule(), Rule::empty)
            {
                errors.push(rule.as_str().to_string());
                None
            } else {
                None
            }
        })
        .collect();
    PatternParseResults {
        valid: patterns,
        invalid: errors,
    }
}
