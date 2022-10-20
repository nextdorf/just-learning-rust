pub mod test;

use std::io::{self, stdout, Write};
use crossterm::{ terminal, Result, event, QueueableCommand, cursor };

/// A Scene is like a chart on a manifold: If you know where you are in the scene, the scene can tell you where you are in the world
pub trait Scene {
  fn step(&mut self, game_data: &mut GameData) -> Result<GameCommand>;
  fn get_bounds(&self) -> Bounds;
}

pub struct Bounds{
  pub x: i64,
  pub y: i64,
  pub width: u16,
  pub heigth: u16
}

impl Bounds {
  pub fn overlaps(&self, other: &Self) -> Option<Bounds> {
    let (l, r) = { //left edge of l is further to the left than the left edge of r
      if self.x <= other.x {(self, other)}
      else {(other, self)}
    };
    let lwidth64 = i64::from(l.width);
    if lwidth64 < r.x-l.x {None} //Out of x bounds
    else { //There is no gap in x-direction between l and r
      if l.y <= r.y{
        let lheigth64 = i64::from(l.heigth);
        if lheigth64 < r.y-l.y {None} //Out of y bounds
        else{ //There is no gap in y-direction between l and r
          Some(Bounds {
            x: r.x,
            y: r.y,
            width: (lwidth64-(r.x-l.x)) as u16,
            heigth: (lheigth64-(r.y-l.y)) as u16
          })
        }
      } else { //l.y > r.y
        let rheigth64 = i64::from(l.heigth);
        if rheigth64 < l.y-r.y {None} //Out of y bounds
        else{ //There is no gap in y-direction between l and r
          Some(Bounds {
            x: r.x,
            y: l.y,
            width: (lwidth64-(r.x-l.x)) as u16,
            heigth: (rheigth64-(l.y-r.y)) as u16
          })
        }
      }
    }
  }
}

/// Represents the position in a scene
pub struct Pos<'a>{
  scene: &'a dyn Scene,
  x: i64,
  y: i64,
}

pub enum GameCommand {
  Exit,
  Continue,
}

/// The game engine
pub struct Game<'a> {
  pub data: GameData,
  pub current_scene: &'a mut dyn Scene,
  suppress_cleanup: bool,
}

pub struct GameData {
  pub stdout: io::Stdout,
  pub width: u16,
  pub height: u16,
  pub pos_x: i64,
  pub pos_y: i64,
}

impl GameData {
  pub fn bounds(&self) -> Bounds{
    Bounds { x: self.pos_x, y: self.pos_y, width: self.width, heigth: self.height }
  }
}

impl Game<'_> {
  pub fn new(start_scene: &mut dyn Scene) -> Game {
    Game {
      data: GameData {
        stdout: stdout(),
        width: 60,
        height: 20,
        pos_x: 0,
        pos_y: 0,
      },
      suppress_cleanup: true,
      current_scene: start_scene,
    }
  }


  pub fn run(&mut self){
    'gameloop: loop {
      let scene = &mut self.current_scene;
      let game_data = &mut self.data;
      let step_result = game_data.stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .and_then(|q| q.queue(terminal::Clear(terminal::ClearType::Purge)))
        .and_then(|q| q.flush())
        .and_then(|_| scene.step(game_data))
        .expect("Scene crashed");

      match step_result {
        GameCommand::Exit => break 'gameloop,
        _ => ()
      }
    }
  }

  pub fn pos(&self) -> Pos{
    Pos{
      scene: self.current_scene,
      x: self.data.pos_x,
      y: self.data.pos_y,
    }
  }

  /// Must be called once in a scope which is equal to or contains the scope of the main loop
  pub fn init(&mut self) -> Result<()>{
    self.suppress_cleanup = false;
    terminal::enable_raw_mode()
      .and(Ok(&mut self.data.stdout))
      .and_then(|q| q.queue(terminal::Clear(terminal::ClearType::All)))
      .and_then(|q| q.queue(crossterm::cursor::Hide))
      .and_then(|q| q.flush())
  }

  fn cleanup(&mut self) -> Result<()>{
    let res_disable_raw = terminal::is_raw_mode_enabled()
      .and_then(|cond|
        if cond {terminal::disable_raw_mode()} 
        else {Ok(())}
      );
    let res_clean_stdout = self.data.stdout
      .queue(crossterm::cursor::Show)
      .and_then(|q| q.queue(cursor::MoveTo(0, 0)))
      .and_then(|q| q.queue(terminal::Clear(terminal::ClearType::All)))
      .and_then(|q| q.queue(terminal::Clear(terminal::ClearType::Purge)))
      .and_then(|q| q.flush());

    res_disable_raw
      .and(res_clean_stdout)
  }
}

impl Drop for Game<'_>{
  fn drop(&mut self) {
    // dbg!(format!("Suppress cleanup in Game::drop: {}", self.suppress_cleanup));
    if !self.suppress_cleanup {
      self.cleanup()
        .expect("Couldn't disable raw mode when dropping core resources")
    }
  }
}




