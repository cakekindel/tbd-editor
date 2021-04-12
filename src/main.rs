use clap::Clap;
use prelude::*;

mod app;
use app::App;

#[derive(Clap)]
pub struct Opts {
  #[clap(short, long, required = true)]
  file: String,
}

fn main() -> Result<(), AnyError> {
  let opts = Opts::parse();

  let mut app = App::try_new()?;
  app.draw()
}
