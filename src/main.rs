use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
mod map;
pub use map::*;
mod rect;
pub use rect::Rect;

#[derive(Component)]
struct Position {
  x: i32,
  y: i32,
}

#[derive(Component)]
struct Renderable {
  glyph: rltk::FontCharType,
  fg: RGB,
  bg: RGB,
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
  type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

  fn run(&mut self, (left_mover, mut pos): Self::SystemData) {
    for (_leftm, pos) in (&left_mover, &mut pos).join() {
      pos.x -= 1;
      if pos.x < 0 {
        pos.x = 79;
      }
    }
  }
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();

  let map = ecs.fetch::<Vec<TileType>>();

  for (_player, pos) in (&mut players, &mut positions).join() {
    let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
    if map[destination_idx] != TileType::Wall {
      pos.x = min(79, max(0, pos.x + delta_x));
      pos.y = min(49, max(0, pos.y + delta_y));
    }
  }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
  //player movement
  match ctx.key {
    None => {} // nothing happened
    Some(key) => match key {
      VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
        try_move_player(-1, 0, &mut gs.ecs)
      }

      VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
        try_move_player(1, 0, &mut gs.ecs)
      }

      VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
        try_move_player(0, -1, &mut gs.ecs)
      }

      VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
        try_move_player(0, 1, &mut gs.ecs)
      }
      _ => {}
    },
  }
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
  let mut y = 0;
  let mut x = 0;

  for tile in map.iter() {
    // render at tile based on type
    match tile {
      TileType::Floor => {
        ctx.set(
          x,
          y,
          RGB::from_f32(0.5, 0.5, 0.5),
          RGB::from_f32(0., 0., 0.),
          rltk::to_cp437('.'),
        );
      }
      TileType::Wall => {
        ctx.set(
          x,
          y,
          RGB::from_f32(0.0, 1.0, 0.0),
          RGB::from_f32(0., 0., 0.),
          rltk::to_cp437('#'),
        );
      }
    }
    // move the coordinates
    x += 1;
    if x > 79 {
      x = 0;
      y += 1;
    }
  }
}

struct State {
  ecs: World,
}

impl State {
  fn run_systems(&mut self) {
    let mut lw = LeftWalker {};
    lw.run_now(&self.ecs);
    self.ecs.maintain();
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();

    player_input(self, ctx);
    self.run_systems();

    let map = self.ecs.fetch::<Vec<TileType>>();
    draw_map(&map, ctx);

    let positions = self.ecs.read_storage::<Position>();
    let renderables = self.ecs.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
      ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
  }
}

fn main() -> rltk::BError {
  let context = rltk::RltkBuilder::simple80x50()
    .with_title("Roguelike Tutorial")
    .build()?;
  let mut gs = State { ecs: World::new() };
  gs.ecs.register::<Position>();
  gs.ecs.register::<Renderable>();
  gs.ecs.register::<LeftMover>();
  gs.ecs.register::<Player>();

  let (rooms, map) = new_map_rooms_and_corridors();
  gs.ecs.insert(map);
  let (player_x, player_y) = rooms[0].center();

  gs.ecs
    .create_entity()
    .with(Position {
      x: player_x,
      y: player_y,
    })
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
    })
    .with(Player {})
    .build();

  // for i in 0..10 {
  //   gs.ecs
  //     .create_entity()
  //     .with(Position { x: i * 7, y: 20 })
  //     .with(Renderable {
  //       glyph: rltk::to_cp437('☺'),
  //       fg: RGB::named(rltk::RED),
  //       bg: RGB::named(rltk::BLACK),
  //     })
  //     .with(LeftMover {})
  //     .build();
  // }

  rltk::main_loop(context, gs)
}
