use std::{collections::HashMap, fs::File, io::Read, path::Path};

fn main() {
    let files = [
        "hexal.txt",
        "hexbound.txt",
        "hexcasting.txt",
        "hexkinetics.txt",
        "hextweaks.txt",
        "moreiotas.txt",
    ];

    let mut imported_patterns = HashMap::new();

    for file_name in files {
        let mut file = File::open(format!("hex_decoder/src/pattern_files/{}", file_name)).unwrap();

        let mut file_data = String::from("");
        file.read_to_string(&mut file_data).unwrap();

        let parts: Vec<String> = file_data.split('\n').map(|a| String::from(a)).collect();

        for i in (0..parts.len()).step_by(5) {
            imported_patterns.insert(
                parts[i + 2].clone(),
                ImportedPattern {
                    parameters: parts[i].clone(),
                    link: parts[i + 1].clone(),
                    pattern: parts[i + 2].clone(),
                    default_direction: parts[i + 3].clone(),
                    great_spell: parts[i + 4].to_lowercase() == "true",
                    name: String::from(parts[i].split(" (").next().unwrap()),
                },
            );
        }
    }

    let extra_patterns = vec![
        ("Introspection", "qqq"),
        ("Retrospection", "eee"),
        ("Consideration", "qqqaw"),
    ];

    for (name, pattern) in extra_patterns {
        imported_patterns.insert(
            pattern.to_string(),
            ImportedPattern {
                link: "N/A".to_string(),
                parameters: "N/A".to_string(),
                pattern: pattern.to_string(),
                default_direction: "WEST".to_string(),
                great_spell: false,
                name: name.to_string(),
            },
        );
    }
    //println!("{:?}", imported_patterns);

    let patterns = "HexPattern(WEST qqqaw), [HexPattern(WEST eaqa), HexPattern(EAST aadaa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST ddewedd), HexPattern(NORTH_EAST aweaqa), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST a), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(NORTH_WEST aqaeqded), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST a)";
    let patterns = "HexPattern(EAST waqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(SOUTH_EAST awdwaaww), HexPattern(SOUTH_EAST awdwa), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(EAST waqaeaq), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(EAST aqwaq), HexPattern(NORTH_EAST aw), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST aada), HexPattern(EAST aqwaq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(EAST aada), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST aqaaw), HexPattern(SOUTH_EAST aqaawaa), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaq), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qqaeaae), HexPattern(NORTH_EAST dwqqqqqwddww), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(EAST wawqwawaw), HexPattern(NORTH_EAST dedq), HexPattern(WEST dwwdwwdwdd), HexPattern(WEST qqq), \"\\\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(WEST qqq), \"/\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawaa), HexPattern(EAST aada), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST edqde), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_WEST wddw), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(EAST aadaadaa), HexPattern(WEST qqq), HexPattern(NORTH_WEST qaeaq), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qeewdweddw), HexPattern(EAST aadaadaa), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST aeea), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(NORTH_EAST dedq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(NORTH_EAST de), HexPattern(WEST qqq), \"Too Many Matches Found!\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(WEST qqq), HexPattern(WEST qqq), \"No Matches Found\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aqdee), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(EAST), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eaqaaw), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(EAST qded), HexPattern(SOUTH_EAST a)";

    let patterns: Vec<&str> = patterns
        .split(", ")
        .map(|a| a[0..a.len() - 1].split(' ').last().unwrap())
        .collect();

    for pattern in patterns {
        if let Some(pattern) = imported_patterns.get(pattern) {
            println!("{}", pattern.name);
            continue;
        }
        if pattern.starts_with("a") || pattern.starts_with("w") {
            let mut characters = pattern.chars().into_iter().skip(1);

            let mut last_was_straight = pattern.starts_with("w");
            let mut valid = true;
            let mut parts = vec![if pattern.starts_with("a") { 'v' } else { '-' }];

            while let Some(ch) = characters.next() {
                if (last_was_straight && ch == 'w') || (!last_was_straight && ch == 'e') {
                    parts.push('-');
                } else if (last_was_straight && ch == 'e') || (!last_was_straight && ch == 'd') {
                    last_was_straight = false;
                    if characters.next().unwrap_or('b') != 'a' {
                        valid = false;
                        break;
                    }
                    parts.push('v');
                } else {
                    valid = false;
                    break;
                }
            }
            if valid {
                println!(
                    "Bookkeeper's Gambit: {}",
                    parts.into_iter().collect::<String>()
                );
                continue;
            }
        }

        if pattern.starts_with("aqaa") {
            let mut num = 0;
            let characters = pattern.chars().into_iter().skip(4);
            let mut valid = true;
            for char in characters {
                match char {
                    'w' => num += 1,
                    'q' => num += 5,
                    'e' => num += 10,
                    'a' => num *= 2,
                    'd' => num /= 2,
                    _ => {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                println!("Number: {}", num);
                continue;
            }
        }

        //if pattern

        println!("Unknown Pattern: {}", pattern);
    }
}

#[derive(Debug, Clone)]
struct ImportedPattern {
    link: String,
    parameters: String,
    pattern: String,
    default_direction: String,
    great_spell: bool,
    name: String,
}

/*
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

pattern = {pattern1 | pattern2 | (direction ~ ((WHITESPACE+ ~ angles) | (WHITESPACE* ~ (&"," | EOI))))}
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
*/

/*
let parsed_patterns = PatternParser::parse(Rule::hex, patterns_str)
    .unwrap()
    .next()
    .unwrap();

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
        } else {
            Some(Pattern::try_from("SOUTH_EAST aqaa").unwrap())
        }
    })
    .collect();*/
