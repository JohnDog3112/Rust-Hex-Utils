use std::{
    fs::{self, OpenOptions},
    io::{Read, Seek},
    path::Path,
    time::{Duration, SystemTime},
};

use hex_renderer::{grids::GridDraw, pattern_utils};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

const FILE_LOC: &str =
    "/home/johng/.local/share/PrismLauncher/instances/HexxyCraft/.minecraft/logs/latest.log";
fn main() {
    let path = Path::new(FILE_LOC);

    let data = fs::read_to_string(path).unwrap();

    let mut characters = data.len() as u64;

    //println!("{}", data);

    loop {
        let metadata = fs::metadata(path).unwrap();

        if metadata.len() > characters {
            let mut file = OpenOptions::new().read(true).open(path).unwrap();

            //seek what's been read
            file.seek(std::io::SeekFrom::Start(characters)).unwrap();

            //read new stuff
            let mut new_stuff = Vec::new();
            file.read_to_end(&mut new_stuff).unwrap();

            //convert to string
            let data = String::from_utf8(new_stuff).unwrap();

            //println!("{}", data);
            characters = file.metadata().unwrap().len();

            parse_str(&data);
        }

        std::thread::sleep(Duration::from_secs(5));
    }
}

#[derive(Parser)]
#[grammar_inline = r#"

everything = { SOI ~ (chat_msg | ANY)+ ~ EOI}

chat_msg = {time ~ render_type ~ ":" ~ chat_subtype ~ iota~ NEWLINE*}

time = { "[" ~ (ASCII_DIGIT* ~ ":"*){3} ~ "]"}

render_type = { "[Render thread/INFO]" }

chat_subtype = { "[CHAT]" }

till_end = { (!NEWLINE ~ ANY)+}

WHITESPACE = _{" "}


iota = _{ pattern | list | string | number | vector}

pattern = { "<" ~ direction ~ "," ~ pattern_segs? ~ ">" }

direction = { "southeast" | "east" | "northeast" | "southwest" | "west" | "northwest" }

pattern_segs = { segments+ }

segments = _{ "w" | "e" | "d" | "s" | "a" | "q" }

list = { "[" ~ (iota ~ ",")* ~ iota* ~ "]" }

string = {"\"" ~ (!"\"" ~ ANY)* ~ "\"" }

number = { ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }

vector = { "(" ~ (number ~ ","){2} ~ number ~ ")" }

"#]
struct PatternParser;

pub fn parse_str(patterns_str: &str) {
    let parsed_iotas = PatternParser::parse(Rule::everything, patterns_str)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    for iota in parsed_iotas {
        if iota.as_rule() != Rule::chat_msg {
            continue;
        }
        let iota = iota.into_inner().skip(3).next().unwrap();

        let iota = parse_iota(iota).unwrap();

        //println!("{:?}", iota);

        if let Iota::List(list) = iota {
            check_command(list);
        }
    }
}

fn parse_iota(iota: Pair<'_, Rule>) -> Option<Iota> {
    Some(match iota.as_rule() {
        Rule::pattern => parse_pattern(iota)?,
        Rule::list => parse_list(iota)?,
        Rule::string | Rule::number | Rule::vector => Iota::Unknown,
        _ => return None,
    })
}
fn parse_pattern(iota: Pair<'_, Rule>) -> Option<Iota> {
    let mut iota = iota.into_inner();
    //println!("{iota}");
    let direction = iota.next()?.as_str().to_string();

    Some(Iota::Pattern(Pattern {
        direction,
        segments: if let Some(segments) = iota.next() {
            segments.as_str().to_string()
        } else {
            String::new()
        },
    }))
}
fn parse_list(iota: Pair<'_, Rule>) -> Option<Iota> {
    Some(Iota::List(
        iota.into_inner()
            .map(|iota| parse_iota(iota))
            .collect::<Option<Vec<Iota>>>()?,
    ))
}

fn check_command(list: Vec<Iota>) {
    if list.len() < 1 {
        return;
    }
    if let Iota::Pattern(pattern) = &list[0] {
        if pattern.segments == "wwqaqdada" {
            initialize_dictionary(list);
        }
    }
}

fn initialize_dictionary(list: Vec<Iota>) {
    if list.len() != 2 {
        return;
    }
    let dictionary = {
        let [_, iota]: [Iota; 2] = list.try_into().unwrap();
        if let Iota::List(dictionary) = iota {
            dictionary
        } else {
            return;
        }
    };
    if dictionary.len() != 2 {
        return;
    }

    let (keys, values) = {
        let [keys, values]: [Iota; 2] = dictionary.try_into().unwrap();
        if let Iota::List(keys) = keys {
            if let Iota::List(values) = values {
                (keys, values)
            } else {
                return;
            }
        } else {
            return;
        }
    };

    if keys.len() != values.len() {
        return;
    }

    let (keys, values) = {
        let keys = keys
            .into_iter()
            .map(|iota| {
                if let Iota::Pattern(pattern) = iota {
                    Some(pattern)
                } else {
                    None
                }
            })
            .collect::<Option<Vec<Pattern>>>();
        let values = values
            .into_iter()
            .map(|iota| {
                if let Iota::List(iota) = iota {
                    Some(iota)
                } else {
                    None
                }
            })
            .collect::<Option<Vec<Vec<Iota>>>>();
        if let Some(keys) = keys {
            if let Some(values) = values {
                (keys, values)
            } else {
                return;
            }
        } else {
            return;
        }
    };

    for (key, patterns) in keys.into_iter().zip(values.into_iter()) {
        let name = format!("{}.png", key.segments);

        let patterns: Vec<hex_renderer::Pattern> = patterns
            .iter()
            .filter_map(|iota| match iota {
                Iota::Pattern(pattern) => Some({
                    let direction =
                        pattern_utils::Direction::try_from(&pattern.direction[..]).unwrap();
                    let angles = pattern
                        .segments
                        .chars()
                        .map(|ch| pattern_utils::Angle::try_from(ch).unwrap())
                        .collect();
                    hex_renderer::Pattern::new(direction, angles)
                }),
                Iota::List(_) => None,
                Iota::Unknown => None,
            })
            .collect();

        let hex_grid = hex_renderer::grids::HexGrid::new_normal(patterns, 50).unwrap();

        hex_grid
            .draw_grid_to_file(&name, 50.0, &hex_renderer::defaults::GRADIENT)
            .unwrap();
    }
}

#[derive(Debug)]
enum Iota {
    Pattern(Pattern),
    List(Vec<Iota>),
    Unknown,
}
#[derive(Debug)]
struct Pattern {
    pub direction: String,
    pub segments: String,
}
