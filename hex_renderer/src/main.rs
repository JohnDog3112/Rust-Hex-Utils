use std::time::Instant;

mod angle;
use angle::Angle;

mod direction;
use direction::Direction;

mod coord;
use coord::Coord;

mod hex_coord;
use draw_options::{Lines, Intersections, Marker, GradientOptions};
use hex_coord::HexCoord;

mod pattern;
use pattern::Pattern;

mod dynamic_list;

mod pattern_grid;

mod draw_options;


use tiny_skia::*;

use crate::draw_options::{Triangle, EndPoint};

const SCALE: f32 = 50.0;
const LINE_THICKNESS: f32 = 0.12;

fn main() {

    let start = Instant::now();

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(128, 0, 128, 255);



    //let patterns_str = "HexPattern(WEST qqq), Air, Chicken Type, Wheat Seeds, Cow Type, Wheat, Sheep Type, Wheat, HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";
    
    //let patterns_str = "HexPattern(NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww), HexPattern(WEST qqq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";

    let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww";

    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweeadedaeewaawwww";
    //let patterns_str = "SOUTH_EAST wqwwwqwwwdwewdwqqdaeeeeeaddwweaqaawewawqwawwwewwwew";

    let patterns: Vec<Pattern> = patterns_str.split(", ").filter_map(|str| Pattern::try_from(str).map_or(None, |pattern| Some(pattern))).collect();
    


    let grid = pattern_grid::PatternGrid::generate(patterns.clone(), 20);
    

    //let mut distorted_pixmap = Pixmap::new(picture_width, picture_height).unwrap();
    
    let beginning_point = EndPoint::Marker(Marker::SinglePoint(Color::WHITE, 0.14));
    let ending_point = EndPoint::Marker(Marker::SinglePoint(Color::BLACK, 0.14));
    let middle_point = Marker::SinglePoint(Color::from_rgba8(255, 100, 0, 255), 0.14);

    let intersection = Intersections::EndsAndMiddle(beginning_point, ending_point, middle_point.clone());

    let intersection = Intersections::EndsAndMiddle(
        EndPoint::BorderedMatch(0.1, Color::WHITE, 0.14),
        //EndPoint::BorderedMatch(4.0, Color::WHITE, 6.0/50.0),
        Marker::SinglePoint(Color::WHITE, 0.1).into(),
        Marker::SinglePoint(Color::WHITE, 0.1),
    );
    //let intersection = Intersections::UniformPoints(Point::SinglePoint(Color::from_rgba8(207, 8, 12, 255), 2.0/50.0));


    let start_color = Color::from_rgba8(175, 15, 255, 255);
    let end_color = Color::from_rgba8(80, 8, 117, 255);
    let line_options = Lines::Gradient(GradientOptions {
        colors: vec![
            Color::from_rgba8(214, 9, 177, 255),
            Color::from_rgba8(108, 25, 140, 255),
            //Color::from_rgba8(50, 102, 207, 255),
            //Color::from_rgba8(102, 110, 125, 255),
        ], 
        segs_per_color: 15,
        bent_corners: true,
    });
    
    let line_options = Lines::SegmentColors(vec![
        Color::from_rgba8(214, 9, 177, 255),
        Color::from_rgba8(108, 25, 140, 255),
        Color::from_rgba8(50, 102, 207, 255),
        Color::from_rgba8(102, 110, 125, 255),
    ], Triangle::BorderStartMatch(0.16, Color::WHITE, 0.24));
    
    //let line_options = Lines::Monocolor(Color::from_rgba8(108, 25, 140, 255));

    //let intersection = Intersections::UniformPoints(Marker::SinglePoint(Color::WHITE, 4.0/50.0));

    //let intersection = Intersections::Nothing;
    
    let border_size = line_options.get_max_radius()
        .max(intersection.get_max_radius())
        .max(LINE_THICKNESS) * SCALE;

    let offset = HexCoord(border_size, border_size);
    let map_size = grid.bottom_right * SCALE + offset * 2.0;
    let mut pixmap = Pixmap::new(map_size.0 as u32, map_size.1 as u32).unwrap();

    let drawing_time = Instant::now();
    for i in 0..grid.patterns.len() {
        let pattern = &grid.patterns[i];
        let location = HexCoord::from(grid.locations[i])*SCALE + offset;


        //let path = pattern.generate_path(location, LINE_LENGTH);

        //pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

        //pattern.draw_pattern(&mut pixmap, location, LINE_LENGTH, 6.0, Lines::Monocolor(Color::from_rgba8(128, 0, 128, 255)), Intersections::UniformPoints(Point::SinglePoint(Color::WHITE, 5.0)));
        

        pattern.draw_pattern(&mut pixmap, location, SCALE, LINE_THICKNESS, &line_options, &intersection);

        
        //distorted_pixmap.stroke_path(&path.1, &paint, &stroke, Transform::identity(), None);
    }

    let drawing_time = drawing_time.elapsed();


    pixmap.save_png("image.png").unwrap();
    //distorted_pixmap.save_png("distorted_image.png").unwrap();

    println!("drawing_time: {:?}", drawing_time);
    println!("total_time: {:?}", start.elapsed());


}
