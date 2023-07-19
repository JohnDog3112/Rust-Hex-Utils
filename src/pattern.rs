use tiny_skia::{Path, PathBuilder, Pixmap, Paint, Stroke, LineCap, Transform, FillRule, Color, LinearGradient, SpreadMode, GradientStop};

use crate::{coord::Coord, direction::Direction, angle::Angle, hex_coord::HexCoord, dynamic_list::DynamicList, draw_options::{Intersections, Lines, Point}};

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

       
    
    pub fn generate_path(&self, origin: HexCoord, line_length: f32) -> Path{
        let mut pb = PathBuilder::new();

        pb.move_to(origin.0, origin.1);

        for line in &self.path {
            let current = HexCoord::from(*line) * line_length + origin;
            
            pb.line_to(current.0, current.1);
        }

        pb.finish().unwrap()
    }

    pub fn draw_pattern(&self, pixmap: &mut Pixmap, origin: HexCoord, line_length: f32, line_thickness: f32, line_options: &Lines, point_options: &Intersections) {
        let mut paint = Paint::default();
        let mut stroke = Stroke::default();
        stroke.width = line_thickness;
        stroke.line_cap = LineCap::Round;
        
        match line_options {
            Lines::Monocolor(color) => {        
                paint.set_color(*color);
                let path = self.generate_path(origin, line_length);
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            },
            Lines::Gradient(start_color, end_color) => {
                self.draw_gradient_lines(pixmap, &stroke, origin, line_length, *start_color,*end_color);
            },
            Lines::SegmentColors(_) => todo!(),
            Lines::MultiGradient(_) => todo!(),
        }

        
        match point_options {
            Intersections::Nothing => (),
            Intersections::UniformPoints(point) => {
                Self::draw_points(&self.points, pixmap, origin, line_length, &point);
            },
            Intersections::EndsAndMiddle(beginning, end, middle) => {
                let beginning_point = self.path[0];
                let ending_point = self.path[self.path.len()-1];

                Self::draw_points(&vec![beginning_point], pixmap, origin, line_length, &beginning);
                if beginning_point != ending_point {
                    Self::draw_points(&vec![ending_point], pixmap, origin, line_length, &end);
                }
                let middle_points: Vec<Coord> = self.points.clone().into_iter().filter(|&point| point != beginning_point && point != ending_point).collect();
                
                Self::draw_points(&middle_points, pixmap, origin, line_length, &middle);
            },  
        }
    }

    fn draw_gradient_lines(&self, pixmap: &mut Pixmap, stroke: &Stroke,
        origin: HexCoord, line_length: f32, 
        beginning_color: Color, ending_color: Color
    ) {
        let segments = self.path.len() as f32 - 1.0;

        let beg_col = [beginning_color.red(), beginning_color.green(), beginning_color.blue(), beginning_color.alpha()];
        let end_col = [ending_color.red(), ending_color.green(), ending_color.blue(), ending_color.alpha()];

        let mut change_per_seg = [0.0; 4];
        for i in 0..4 {
            change_per_seg[i] = (beg_col[i]-end_col[i])/segments;
        }

        let mut cur_color = end_col;

        let mut paint = Paint::default();
        paint.anti_alias = false;
        for i in (1..self.path.len()).rev() {
            let loc_next = origin + HexCoord::from(self.path[i-1]) * line_length;
            let loc_prev = origin + HexCoord::from(self.path[i]) * line_length;

            let mut next_color = [0.0; 4];
            for j in 0..4 {
                next_color[j] = cur_color[j] + change_per_seg[j];
                if next_color[j] > 1.0 {
                    next_color[j] = 1.0;
                } else if next_color[j] < 0.0 {
                    next_color[j] = 0.0;
                }
            }
            
            paint.shader = LinearGradient::new(
                tiny_skia::Point::from_xy(loc_prev.0, loc_prev.1),
                tiny_skia::Point::from_xy(loc_next.0, loc_next.1),
                vec![
                    GradientStop::new(0.0, Color::from_rgba(cur_color[0], cur_color[1], cur_color[2], cur_color[3]).unwrap()),
                    GradientStop::new(1.0, Color::from_rgba(next_color[0], next_color[1], next_color[2], next_color[3]).unwrap()),
                ],
                SpreadMode::Pad,
                Transform::identity(),
            ).unwrap();

            let mut pb = PathBuilder::new();

            pb.move_to(loc_prev.0, loc_prev.1);
            pb.line_to(loc_next.0, loc_next.1);

            let path = pb.finish().unwrap();

            pixmap.stroke_path(&path, &paint, stroke, Transform::identity(), None);

            cur_color = next_color;
        }
    }
    fn draw_points(points: &Vec<Coord>, pixmap: &mut Pixmap, origin: HexCoord, line_length: f32, point: &Point) {
        let mut paint = Paint::default();

        let mut paint2 = Paint::default();

        match point {
            Point::None => (),
            Point::SinglePoint(color, radius) => {
                paint.set_color(*color);
                for point in points {
                    let loc = HexCoord::from(*point) * line_length + origin;
                    let path = PathBuilder::from_circle(loc.0, loc.1, *radius).unwrap();
                    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
                }
            },
            Point::DoublePoint(color1, radius1, color2, radius2) => {
                paint.set_color(*color1);
                paint2.set_color(*color2);
                for point in points {
                    let loc = HexCoord::from(*point) * line_length + origin;
                    let path = PathBuilder::from_circle(loc.0, loc.1, *radius1).unwrap();
                    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

                    let path = PathBuilder::from_circle(loc.0, loc.1, *radius2).unwrap();
                    pixmap.fill_path(&path, &paint2, FillRule::Winding, Transform::default(), None);
                }
            },
        }
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