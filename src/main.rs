use std::time::Instant;

mod angle;
use angle::Angle;

mod direction;
use direction::Direction;

mod coord;
use coord::Coord;

mod hex_coord;
use hex_coord::HexCoord;

mod pattern;
use pattern::Pattern;

mod dynamic_list;

mod pattern_grid;


use tiny_skia::*;

const LINE_LENGTH: f32 = 50.0;

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
    let mut current_x_offset = 0;
    let mut current_y = 0;

    let mut max_y_row = 0;

    let mut left_circle_paths = vec![];
    let mut right_circle_paths = vec![];

    let paths: Vec<(Path, ())> = (0..patterns.len()).map(|index| {
        let pattern = &patterns[index];

        
        let height = pattern.bottom_right.1 - pattern.top_left.1;

        if index == 0 {
            current_x -= pattern.top_left.0;
            let mut left_most = f32::MAX;
            for point in &pattern.left_perimiter {
                let point = HexCoord::from(*point + Coord(current_x, current_y));
                if point.0 < left_most {
                    left_most = point.0;
                }
            }
            current_x_offset = -left_most as i32;
        } else if true {
            let prev_pattern = &patterns[index-1];
            let mut max_distance_decrease = i32::MAX;
            for i in 0..pattern.left_perimiter.len().min(prev_pattern.left_perimiter.len()) {
                let right_point = pattern.left_perimiter[i].0;
                let left_point = prev_pattern.right_perimiter[i].0;

                let dist = right_point - left_point;

                //println!("{} - {} = {}", right_point, left_point, dist);

                if dist < max_distance_decrease {
                    max_distance_decrease = dist;
                }
            }
            //println!("max: {}", max_distance_decrease);
            current_x -= max_distance_decrease-1;
        }

        if LINE_LENGTH * HexCoord::from(Coord(current_x+pattern.bottom_right.0, max_y_row)).0 > max_width {
            current_x = -pattern.top_left.0;
            current_y += max_y_row + 1;
            
            let mut left_most = f32::MAX;
            for point in &pattern.left_perimiter {
                let point = HexCoord::from(*point + Coord(current_x, current_y));
                if point.0 < left_most {
                    left_most = point.0;
                }
            }
            current_x_offset = -left_most as i32;

            max_y_row = 0;
        } 

        if height > max_y_row {
            max_y_row = height;
        }

        let draw_x = current_x + current_x_offset;
        let draw_y: i32 = current_y - pattern.top_left.1;
        let path = pattern.generate_path((draw_x, draw_y), LINE_LENGTH);
        //let distorted_path = pattern.generate_distorted_path((current_x, current_y - pattern.top_left.1), LINE_LENGTH);
        
        
        for loc in &pattern.points {
            let draw_loc = HexCoord::from(Coord(draw_x, draw_y) + *loc) * LINE_LENGTH;
            //println!("{:?}", draw_loc);
            left_circle_paths.push(PathBuilder::from_circle(draw_loc.0, draw_loc.1, 10.0).unwrap());
        }

        /*for loc in &pattern.right_perimiter {
            let draw_loc = HexCoord::from(Coord(draw_x, draw_y) + *loc) * LINE_LENGTH;
            //println!("{:?}", draw_loc);
            right_circle_paths.push(PathBuilder::from_circle(draw_loc.0, draw_loc.1, 5.0).unwrap());
        }*/
        
        
        (path, ())
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
    //let mut distorted_pixmap = Pixmap::new(picture_width, picture_height).unwrap();
    
    for path in paths {
        pixmap.stroke_path(&path.0, &paint, &stroke, Transform::identity(), None);
        //distorted_pixmap.stroke_path(&path.1, &paint, &stroke, Transform::identity(), None);
    }

    paint.set_color_rgba8(120, 120, 120, 255);
    for path in left_circle_paths {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
    }

    paint.set_color_rgba8(120, 200, 120, 255);
    for path in right_circle_paths {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
    }

    let paint_paths = start.elapsed() - path_init;

    pixmap.save_png("image.png").unwrap();
    //distorted_pixmap.save_png("distorted_image.png").unwrap();

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
