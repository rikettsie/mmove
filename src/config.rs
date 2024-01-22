use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Config {

  /// X coordinate of the first target point to click
  #[arg(short)]
  pub x: i32,

  /// Y coordinate of the first target point to click
  #[arg(short)]
  pub y: i32,

  /// Vertical offset between the two points to click
  #[arg(short, long)]
  pub v_offset: i32,
}
