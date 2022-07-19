#[derive(structopt::StructOpt)]
struct Args {
    /// Input bias
    #[structopt(short, long, default_value = "disable")]
    bias: gpiod::Bias,

    /// Active state
    #[structopt(short, long, default_value = "high")]
    active: gpiod::Active,

    /// Request label
    #[structopt(short, long, default_value = "gpioset")]
    label: String,

    /// GPIO chip
    #[structopt()]
    chip: std::path::PathBuf,

    /// GPIO lines
    #[structopt()]
    lines: Vec<gpiod::LineId>,
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    if args.lines.len() > gpiod::Values::MAX {
        anyhow::bail!("Too many lines");
    }

    let chip = gpiod::Chip::new(&args.chip)?;

    let input = chip.request_input(
        &args.lines,
        args.active,
        Default::default(),
        args.bias,
        &args.label,
    )?;

    for value in input.get_values::<gpiod::Values>()? {
        print!("{}", if value { 1 } else { 0 });
    }
    println!("");

    Ok(())
}
