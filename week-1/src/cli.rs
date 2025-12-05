pub enum Command {
    Check(i32),
    Reverse(i32),
    Length(i32),
    Invalid,
}

pub fn parse_args() -> Command {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        return Command::Invalid;
    }

    let cmd = &args[1];
    let num = args[2].parse::<i32>().unwrap_or(0);

    match cmd.as_str() {
        "check"   => Command::Check(num),
        "reverse" => Command::Reverse(num),
        "length"  => Command::Length(num),
        _         => Command::Invalid,
    }
}
