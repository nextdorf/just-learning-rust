use std::io::Write;

use super::{Scene, GameCommand, GameData, Pos};
use crossterm::{Result, cursor, style::{self, Stylize}, QueueableCommand, event};

pub struct TestScene{
  pub u: u16,
  pub v: u16,
  room_width: u16,
  room_height: u16,
  corridor_len: u16,

}

impl Scene for TestScene{
  fn step(&mut self, game: &mut GameData) -> Result<GameCommand> {
    // let _pos = Pos { scene: self, x: game.pos_x, y: game.pos_y };
    let game = Ok(game)
      .and_then(|g| self.print_room(g))
      .and_then(|g| self.print_player(g))
      .and_then(|g| g.stdout.flush().and(Ok(g)))?;
      // .and_then(|g| self.user_input(g))

    // let res1 = self.user_input(game);

    match event::read()? {
      event::Event::Key(event::KeyEvent {code, ..} ) => {
        match code {
          event::KeyCode::Esc => return Ok(GameCommand::Exit),
          event::KeyCode::Left => if self.u>0 {self.u-=1},
          event::KeyCode::Right => if self.u<game.width-1 {self.u+=1},
          event::KeyCode::Up => if self.v>0 {self.v-=1},
          event::KeyCode::Down => if self.v<game.height-1 {self.v+=1},
          _ => ()
        }
      },
      _ => ()
    } 

    Ok(GameCommand::Continue)
  }

  fn get_bounds(&self) -> super::Bounds {
    super::Bounds {
      x: 0,
      y: 0,
      width: 2*self.room_width,
      heigth: 2*self.room_height
    }
  }
}



impl TestScene {
  fn print_room<'a>(&'a self, game: &'a mut GameData) -> Result<&'a mut GameData>{
    for y in 0..game.height {
      for x in 0..game.width {
        if (y == 0 || y == game.height - 1) || (x == 0 || x == game.width - 1) {
          // in this loop we are more efficient by not flushing the buffer.
          game.stdout
            .queue(cursor::MoveTo(x,y))?
            .queue(style::PrintStyledContent( "█".magenta()))?;
        }
      }
    }
    Ok(game)
  }
  fn print_single_room<'a>(&'a self, game: &'a mut GameData, bounds: super::Bounds,
    orientation: u8, doorwidth: u8, wallStyle: style::StyledContent<&str>) -> Result<&'a mut GameData>
  {
    let obounds;
    match bounds.overlaps(&game.bounds()){
      Some(b) => obounds = b,
      None => return Ok(game)
    };
    for x in bounds.x..(bounds.x+i64::from(bounds.width)){
      game.stdout
        .queue(cursor::MoveTo(x,bounds.y))?
        .queue(style::PrintStyledContent(wallStyle))?;

    }
  }

  fn print_player<'a>(&'a self, game: &'a mut GameData) -> Result<&'a mut GameData>{
    game.stdout
      .queue(cursor::MoveTo(self.u, self.v))?
      .queue(style::PrintStyledContent("X".cyan()))?;
    Ok(game)
  }

  fn user_input<'a>(&'a mut self, game: &'a GameData) -> Result<GameCommand> {
    match event::read()? {
      event::Event::Key(event::KeyEvent {code, ..} ) => {
        match code {
          event::KeyCode::Esc => return Ok(GameCommand::Exit),
          event::KeyCode::Left => if self.u>0 {self.u-=1},
          event::KeyCode::Right => if self.u<game.width-1 {self.u+=1},
          event::KeyCode::Up => if self.v>0 {self.v-=1},
          event::KeyCode::Down => if self.v<game.height-1 {self.v+=1},
          _ => ()
        }
      },
      _ => ()
    } 

    Ok(GameCommand::Continue)
  }
}

