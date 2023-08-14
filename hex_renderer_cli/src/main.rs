use clap::{error::ErrorKind, Args, CommandFactory, Parser, ValueEnum};
use hex_renderer::{
    defaults,
    grids::{GridDraw, HexGrid, SquareGrid},
    options::GridOptions,
};
use parse_patterns::PatternParseResults;

mod parse_patterns;

#[derive(Parser)]
struct Cli {
    #[arg(value_parser = pattern_parser)]
    patterns: PatternParseResults,

    #[command(flatten)]
    grid_type: GridArgs,

    #[command(flatten)]
    square_args: SquareArgs,

    #[command(flatten)]
    size: SizeArgs,

    #[arg(long, value_parser = not_zero)]
    max_width: Option<usize>,

    file_name: Option<String>,

    option: Option<PatternOption>,

    #[arg(long)]
    ignore_invalid: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct GridArgs {
    #[arg(long)]
    hex: bool,
    #[arg(long, group = "square_group")]
    square: bool,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PatternOption {
    UniformGradient,
    Gradient,
    UniformPointGradient,
    PointGradient,

    UniformSegment,
    Segment,

    UniformMonocolor,
    Monocolor,
}

#[derive(Args)]
struct SizeArgs {
    #[arg(long, group = "size", value_parser = not_zero)]
    x_width: Option<usize>,
    #[arg(long, group = "size", value_parser = not_zero)]
    y_width: Option<usize>,
    #[arg(long, conflicts_with = "size", value_parser = greater_than_zero)]
    scale: Option<f32>,
}

#[derive(Args)]
#[group(requires = "square_group")]
struct SquareArgs {
    #[arg(long, value_parser = positive_float)]
    x_padding: Option<f32>,
    #[arg(long, value_parser = positive_float)]
    y_padding: Option<f32>,
    #[arg(long, value_parser = positive_float)]
    max_scale: Option<f32>,
}

fn main() {
    let cli = Cli::parse();

    let patterns = cli.patterns.valid;
    if !cli.patterns.invalid.is_empty() && !cli.ignore_invalid {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "Invalid patterns! {:?} add --ignore-invalid to ignore them.",
                cli.patterns.invalid
            ),
        )
        .exit();
    }

    let options = cli.option.unwrap_or(PatternOption::Segment).into();

    let file_name = cli.file_name.unwrap_or("HexPatterns.png".to_string());

    let grid: Box<dyn GridDraw>;
    if cli.grid_type.square {
        let args = cli.square_args;
        let square_grid = SquareGrid::new_normal(
            patterns,
            cli.max_width.unwrap_or(20),
            args.max_scale.unwrap_or(0.4),
            args.x_padding.unwrap_or(0.2),
            args.y_padding.unwrap_or(0.1),
        )
        .unwrap();

        grid = Box::new(square_grid);
    } else {
        grid = Box::new(HexGrid::new_normal(patterns, cli.max_width.unwrap_or(50)).unwrap());
    }

    let scale;
    if let Some(sc) = cli.size.scale {
        scale = sc;
    } else if let Some(x_width) = cli.size.x_width {
        let y_width = cli.size.y_width.unwrap_or(usize::MAX);
        scale = grid.get_bound_scale((x_width as f32, y_width as f32), options);
    } else if let Some(y_width) = cli.size.y_width {
        scale = grid.get_bound_scale((f32::MAX, y_width as f32), options);
    } else {
        scale = 100.0;
    }

    grid.draw_grid_to_file(&file_name, scale, options).unwrap();
}

impl Into<&GridOptions> for PatternOption {
    fn into(self) -> &'static GridOptions {
        match self {
            PatternOption::UniformGradient => &defaults::UNIFORM_GRADIENT,
            PatternOption::Gradient => &defaults::GRADIENT,
            PatternOption::UniformPointGradient => &defaults::UNIFORM_POINT_GRADIENT,
            PatternOption::PointGradient => &defaults::POINT_GRADIENT,
            PatternOption::UniformSegment => &defaults::UNIFORM_SEGMENT,
            PatternOption::Segment => &defaults::SEGMENT,
            PatternOption::UniformMonocolor => &defaults::UNIFORM_MONOCOLOR,
            PatternOption::Monocolor => &defaults::MONOCOLOR,
        }
    }
}

fn positive_float(s: &str) -> Result<f32, String> {
    let val: f32 = s.parse().map_err(|_| format!("`{s}` isn't a float"))?;
    if val >= 0.0 {
        Ok(val)
    } else {
        Err(format!("{} is not greater than or equal to zero!", val))
    }
}

fn greater_than_zero(s: &str) -> Result<f32, String> {
    let val: f32 = s.parse().map_err(|_| format!("`{s}` isn't a float!"))?;

    if val > 0.0 {
        Ok(val)
    } else {
        Err(format!("{} isn't greater than zero!", val))
    }
}

fn not_zero(s: &str) -> Result<usize, String> {
    let val: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a positive integer!"))?;

    if val != 0 {
        Ok(val)
    } else {
        Err(format!("Value can't be zero!"))
    }
}

fn pattern_parser(s: &str) -> Result<PatternParseResults, String> {
    Ok(parse_patterns::parse_str(s))
}
