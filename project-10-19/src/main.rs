use crossterm::Result;
use project_10_19::core::*;

fn main() -> Result<()> {
  let mut test_scene = test::TestScene {
    u: 0,
    v: 0
  };
  let mut g = Game::new(&mut test_scene);

  g.init()?;

  Ok(g.run())
}

