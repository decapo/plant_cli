use clap::{Arg, Command};
pub fn get_args() -> (String, String, f32, usize, u32) {
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
        .get_matches();

    let rules_x: String = matches.get_one("rules_x").cloned().unwrap();

    let rules_f = matches
        .get_one("rules_f")
        .cloned()
        .unwrap_or("FF")
        .to_string();

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

    (rules_x, rules_f, delta, speed, iterations)
}
