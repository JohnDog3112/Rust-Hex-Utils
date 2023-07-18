use std::ops::Add;
use std::time::{Duration, Instant};

use tiny_skia::*;

fn main() {
    let start = Instant::now();

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(128, 0, 128, 255);

    let paint_init = start.elapsed();

    let patterns_str = "HexPattern(WEST qqq), Air, Chicken Type, Wheat Seeds, Cow Type, Wheat, Sheep Type, Wheat, HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";

    let patterns: Vec<Pattern> = patterns_str.split(", ").filter_map(|str| Pattern::try_from(str).map_or(None, |pattern| Some(pattern))).collect();
    
    let pattern_init = start.elapsed() - paint_init;
    

    let max_width = 1000.0;

    let mut current_x = 0;
    let mut current_y = 0;

    let mut max_y_row = 0;

    let paths: Vec<(Path, Path)> = patterns.iter().map(|pattern| {
        current_x -= pattern.top_left.0;

        let height = pattern.bottom_right.1 - pattern.top_left.1;
        if height > max_y_row {
            max_y_row = height;
        }

        if HexCoord::from(Coord(current_x+pattern.bottom_right.0, max_y_row)).0 > max_width {
            current_x = -pattern.top_left.0;
            current_y += max_y_row + 1;
            
            max_y_row = 0;
        }
        let path = pattern.generate_path((current_x-current_y/2, current_y - pattern.top_left.1));
        let distorted_path = pattern.generate_distorted_path((current_x, current_y - pattern.top_left.1));
        
        current_x += pattern.bottom_right.0 + 1;
        
        (path, distorted_path)
    }).collect();

    let path_init = start.elapsed() - pattern_init;

    let mut stroke = Stroke::default();
    stroke.width = 6.0;
    stroke.line_cap = LineCap::Round;
    stroke.dash = None;


    let picture_width = if current_y == 0 {
        (current_x as u32 +5)*LINE_LENGTH as u32
    } else {
        max_width as u32
    };
    let picture_height = (current_y + max_y_row) as u32 * LINE_LENGTH as u32;

    let mut pixmap = Pixmap::new(picture_width, picture_height).unwrap();
    let mut distorted_pixmap = Pixmap::new(picture_width, picture_height).unwrap();
    
    for path in paths {
        pixmap.stroke_path(&path.0, &paint, &stroke, Transform::identity(), None);
        distorted_pixmap.stroke_path(&path.1, &paint, &stroke, Transform::identity(), None);
    }
    let paint_paths = start.elapsed() - path_init;

    pixmap.save_png("image.png").unwrap();
    distorted_pixmap.save_png("distorted_image.png").unwrap();

    let save_images = start.elapsed() - paint_paths;

    let total_time = start.elapsed();
    println!("paint init {:?}", paint_init);
    println!("pattern init {:?}", pattern_init);
    println!("path init: {:?}", path_init);
    println!("Total Logic Code: {:?}", paint_init + pattern_init + path_init);
    println!("paint paths: {:?}", paint_paths);
    println!("save images: {:?}", save_images);
    println!("Total Time: {:?}", total_time);

    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East = 0,
    SouthEast = 1,
    SouthWest = 2,
    West = 3,
    NorthWest = 4,
    NorthEast = 5,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Angle {
    Forward = 0,
    Right = 1,
    BackRight = 2,
    Back = 3,
    BackLeft = 4,
    Left = 5,
}


impl Add<Angle> for Direction {
    type Output = Self;

    fn add(self, rhs: Angle) -> Self::Output {
        ((self as u8 + rhs as u8)%6).try_into().unwrap()
    }
}

impl TryFrom<char> for Angle {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Angle::Forward),
            'e' => Ok(Angle::Right),
            'd' => Ok(Angle::BackRight),
            's' => Ok(Angle::Back),
            'a' => Ok(Angle::BackLeft),
            'q' => Ok(Angle::Left),
            _ => Err(()),
        }
    }
}
impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::East),
            1 => Ok(Direction::SouthEast),
            2 => Ok(Direction::SouthWest),
            3 => Ok(Direction::West),
            4 => Ok(Direction::NorthWest),
            5 => Ok(Direction::NorthEast),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "east" | "e" => Ok(Direction::East),
            "southeast" | "south_east" | "se" => Ok(Direction::SouthEast),
            "southwest" | "south_west" | "sw" => Ok(Direction::SouthWest),
            "west" | "w" => Ok(Direction::West),
            "northwest" | "north_west" | "nw" => Ok(Direction::NorthWest),
            "northeast" | "north_east" | "ne" => Ok(Direction::NorthEast),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i32, i32);

impl Add<Direction> for Coord {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::East => (1, 0),
            Direction::West => (-1, 0),

            Direction::NorthEast => (1, -1),
            Direction::SouthWest => (-1, 1),

            Direction::SouthEast => (0, 1),
            Direction::NorthWest => (0, -1),
        }
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<(i32 ,i32)> for Coord {
    fn from(value: (i32 ,i32)) -> Self {
        Coord(value.0, value.1)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct HexCoord(pub f32, pub f32);

const LINE_LENGTH: f32 = 50.0;
const Y_FACTOR: f32 = 0.866025403784;

impl From<Coord> for HexCoord {
    fn from(value: Coord) -> Self {
        HexCoord(
            (value.0 as f32 + 0.5 * value.1 as f32) * LINE_LENGTH,
            value.1 as f32 * LINE_LENGTH * Y_FACTOR
        )
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    pub path: Vec<Coord>,
    pub top_left: Coord,
    pub bottom_right: Coord,
}

impl Pattern {
    fn new(rotation: Direction, links: Vec<Angle>) -> Self {
        let mut path = vec![Coord(0,0), Coord(0,0) + rotation];
        let mut rotation = rotation;
        

        let mut top_left = Coord(0,0);
        let mut bottom_right = Coord(0,0);

        if top_left.0 > path[1].0 {
            top_left.0 = path[1].0;
        } else if bottom_right.0 < path[1].0 {
            bottom_right.0 = path[1].0;
        }

        if top_left.1 > path[1].1 {
            top_left.1 = path[1].1;
        } else if bottom_right.1 < path[1].1 {
            bottom_right.1 = path[1].1;
        }

        for link in links {
            rotation = rotation + link;

            let next_point = *path.last().unwrap() + rotation;

            if top_left.0 > next_point.0 {
                top_left.0 = next_point.0;
            } else if bottom_right.0 < next_point.0 {
                bottom_right.0 = next_point.0;
            }

            if top_left.1 > next_point.1 {
                top_left.1 = next_point.1;
            } else if bottom_right.1 < next_point.1 {
                bottom_right.1 = next_point.1;
            }
            path.push(next_point);
        }

        Pattern {
            path,
            top_left,
            bottom_right
        }
    }

    
    fn generate_path(&self, origin: (i32, i32)) -> Path{
        let mut pb = PathBuilder::new();

        //let origin = ((origin.0 as f32 + 0.5 * origin.1 as f32) * LINE_LENGTH, origin.1 as f32 * LINE_LENGTH * Y_FACTOR);
        
        let origin = Coord::from(origin);

        let hex_origin = HexCoord::from(origin);
        pb.move_to(hex_origin.0, hex_origin.1);

        for line in &self.path {
            let current = HexCoord::from(*line + origin);
            
            pb.line_to(current.0, current.1);
        }

        pb.finish().unwrap()
    }

    fn generate_distorted_path(&self, origin: (i32, i32)) -> Path{
        let mut pb = PathBuilder::new();

        //println!("{:?}", self);
        //println!("Origin: {:?}", origin);

        let origin = ((origin.0 as f32) * LINE_LENGTH, origin.1 as f32 * LINE_LENGTH);
        pb.move_to(origin.0, origin.1);

        for line in &self.path {
            let current_x = origin.0 as f32 + (line.0 as f32 ) * LINE_LENGTH;
            let current_y = origin.1 as f32 + line.1 as f32 * LINE_LENGTH;
            
            pb.line_to(current_x, current_y);
        }

        pb.finish().unwrap()
    }
}

impl TryFrom<&str> for Pattern {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: Vec<&str> = value.split(' ').collect();

        if parts.len() != 2 {
            return Err(());
        }

        if parts[0].to_lowercase().starts_with("hexpattern(") {
            if parts[1].ends_with(')') {
                parts[0] = &parts[0]["hexpattern(".len()..];
                parts[1] = &parts[1][..parts[1].len()-1];
            } else {
                return Err(());
            }
        }

        let direction: Direction = parts[0].try_into()?;

        let angles: Vec<Angle> = parts[1].chars().map(|a| Angle::try_from(a)).collect::<Result<Vec<Angle>, _>>()?;

        return Ok(Pattern::new(direction, angles));
    }
}