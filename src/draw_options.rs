use tiny_skia::Color;

pub enum Lines {
    Monocolor(Color),
    Gradient(Color, Color),
    MultiGradient(Vec<Color>),
    SegmentColors(Vec<Color>),
}

pub enum Point {
    None,
    SinglePoint(Color, f32),
    DoublePoint(Color, f32, Color, f32),
}

pub enum Intersections {
    Nothing,
    UniformPoints(Point),
    EndsAndMiddle(Point, Point, Point),
}