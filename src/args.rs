use clap::{Arg, Command};
use nannou::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

pub fn get_args() -> (String, String, f32, usize, u32, Rgb<u8>, Rgb<u8>, f32) {
    let matches = Command::new("L-System Tree Drawer")
        .version("1.0")
        .author("Sammy Nouri <sammynouri@gmail.com>")
        .about("Draws trees using an L-system")
        .arg(
            Arg::new("rules_x")
                .short('x')
                .long("rules_x")
                .value_name("RULES_X")
                .help("Sets the rules for X")
                .default_value("F[+X][-X]FX")
                .num_args(1),
        )
        .arg(
            Arg::new("rules_f")
                .short('f')
                .long("rules_f")
                .value_name("RULES_F")
                .help("Sets the rules for F")
                .default_value("FF")
                .num_args(1),
        )
        .arg(
            Arg::new("delta")
                .short('d')
                .long("delta")
                .value_name("DELTA")
                .help("Sets the delta angle")
                .default_value("25.7")
                .num_args(1),
        )
        .arg(
            Arg::new("speed")
                .short('s')
                .long("speed")
                .value_name("SPEED")
                .help("Sets the animation speed")
                .default_value("3")
                .num_args(1),
        )
        .arg(
            Arg::new("iterations")
                .short('i')
                .long("iterations")
                .value_name("ITERATIONS")
                .help("Sets the number of iterations")
                .default_value("7")
                .num_args(1),
        )
        .arg(
            Arg::new("bg_color")
                .short('b')
                .long("bg_color")
                .value_name("BG_COLOR")
                .help("Sets the background color (RGB format: R, G, B)")
                .default_value("(0, 0, 0)")
                .num_args(1),
        )
        .arg(
            Arg::new("tree_color")
                .short('t')
                .long("tree_color")
                .value_name("TREE_COLOR")
                .help("Sets the tree color (RGB format: R, G, B)")
                .default_value("(144, 238, 144)")
                .num_args(1),
        )
        .arg(
            Arg::new("scaling_factor")
                .short('c')
                .long("scaling_factor")
                .value_name("SCALING_FACTOR")
                .help("Sets the scaling factor for distance")
                .default_value("1.0")
                .num_args(1),
        )
        .get_matches();

    let rules_x = matches.get_one("rules_x").cloned().unwrap();

    let rules_f = matches.get_one("rules_f").cloned().unwrap();

    let delta: f32 = matches
        .get_one::<String>("delta")
        .cloned()
        .unwrap()
        .parse::<f32>()
        .unwrap();

    let iterations: u32 = matches
        .get_one::<String>("iterations")
        .cloned()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let speed: usize = matches
        .get_one::<String>("speed")
        .cloned()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let bg_color = parse_color(&matches.get_one::<String>("bg_color").cloned().unwrap())
        .expect("Invalid background color");

    let tree_color = parse_color(&matches.get_one::<String>("tree_color").cloned().unwrap())
        .expect("Invalid tree color");

    let scaling_factor: f32 = matches
        .get_one::<String>("scaling_factor")
        .cloned()
        .unwrap()
        .parse::<f32>()
        .unwrap();

    (
        rules_x,
        rules_f,
        delta,
        speed,
        iterations,
        bg_color,
        tree_color,
        scaling_factor,
    )
}

fn parse_u8(input: &str) -> IResult<&str, u8> {
    let (input, _) = multispace0(input)?;
    let (input, num) = map_res(digit1, |s: &str| s.parse::<u8>())(input)?;
    Ok((input, num))
}

fn parse_color_tuple(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, _) = tag("(")(input)?;
    let (input, (r, _, g, _, b)) =
        tuple((parse_u8, tag(","), parse_u8, tag(","), parse_u8))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (r, g, b)))
}

fn parse_color(color_str: &str) -> Result<Rgb<u8>, &str> {
    let (_, (r, g, b)) = parse_color_tuple(color_str).map_err(|_| "Invalid color format")?;
    Ok(rgb(r, g, b))
}
