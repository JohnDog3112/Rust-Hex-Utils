use tiny_skia::{Path, PathBuilder};

use crate::{coord::Coord, direction::Direction, angle::Angle, hex_coord::HexCoord, dynamic_list::DynamicList};

#[derive(Debug, Clone)]
pub struct Pattern {
    pub path: Vec<Coord>,
    pub top_left: Coord,
    pub bottom_right: Coord,
    
    pub left_perimiter: Vec<Coord>,
    pub right_perimiter: Vec<Coord>,

    pub points: Vec<Coord>,
}

impl Pattern {
    pub fn new(rotation: Direction, links: Vec<Angle>) -> Self {
        let mut path = vec![Coord(0,0), Coord(0,0) + rotation];
        let mut top_left = get_min_components(path[0], path[1]);
        let mut bottom_right = get_max_components(path[0], path[1]);

        let mut rotation = rotation;

        let mut left_perimiter = DynamicList::new();
        let mut right_perimiter = DynamicList::new();

        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[0]);
        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[1]);

        for link in links {
            rotation = rotation + link;

            let next_point = *path.last().unwrap() + rotation;

            top_left = get_min_components(top_left, next_point);
            bottom_right = get_max_components(bottom_right, next_point);
            Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, next_point);
            path.push(next_point);
        }
        
        let mut points = path.clone();
        points.dedup();
        Pattern{
            path,
            top_left,
            bottom_right,
            left_perimiter: left_perimiter.to_vector(),
            right_perimiter: right_perimiter.to_vector(),
            points,
        }
    }
    fn add_to_perimiter(left_perimiter: &mut DynamicList<Coord>, right_perimiter: &mut DynamicList<Coord>, point: Coord) {
        if let Some(val) = left_perimiter.get(point.1) {
            if point.0 < val.0 {
                left_perimiter.set(point.1, point);
            }
        } else {
            left_perimiter.set(point.1, point);
        }

        if let Some(val) = right_perimiter.get(point.1) {
            if point.0 > val.0 {
                right_perimiter.set(point.1, point);
            }
        } else {
            right_perimiter.set(point.1, point);
        }
    }

    
    pub fn generate_path(&self, origin: (i32, i32), line_length: f32) -> Path{
        let mut pb = PathBuilder::new();
        
        let origin = Coord::from(origin);

        let hex_origin = HexCoord::from(origin) * line_length;
        pb.move_to(hex_origin.0, hex_origin.1);

        for line in &self.path {
            let current = HexCoord::from(*line + origin) * line_length;
            
            pb.line_to(current.0, current.1);
        }

        pb.finish().unwrap()
    }

    pub fn generate_distorted_path(&self, origin: (i32, i32), line_length: f32) -> Path{
        let mut pb = PathBuilder::new();

        //println!("{:?}", self);
        //println!("Origin: {:?}", origin);

        let origin = ((origin.0 as f32) * line_length, origin.1 as f32 * line_length);
        pb.move_to(origin.0, origin.1);

        for line in &self.path {
            let current_x = origin.0 as f32 + (line.0 as f32 ) * line_length;
            let current_y = origin.1 as f32 + line.1 as f32 * line_length;
            
            pb.line_to(current_x, current_y);
        }

        pb.finish().unwrap()
    }
}
fn get_min_components(a: Coord, b: Coord) -> Coord {
    let mut res = a;
    if b.0 < res.0 {
        res.0 = b.0;
    }
    if b.1 < res.1 {
        res.1 = b.1;
    }
    res
}
fn get_max_components(a: Coord, b: Coord) -> Coord {
    let mut res = a;
    if b.0 > res.0 {
        res.0 = b.0;
    }
    if b.1 > res.1 {
        res.1 = b.1;
    }
    res
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