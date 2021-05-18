use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
mod map;
mod player;
pub use map::*;
pub use player::*;
mod rect;
pub use rect::Rect;
mod components;
pub use components::*;

pub struct State {
  ecs: World,
}

impl State {
  fn run_systems(&mut self) {
    // let mut lw = LeftWalker {};
    // lw.run_now(&self.ecs);
    self.ecs.maintain();
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();

    player_input(self, ctx);
    self.run_systems();

    let map = self.ecs.fetch::<Map>(); //Vec<TileType>>();
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
  // gs.ecs.register::<LeftMover>();
  gs.ecs.register::<Player>();

  let map: Map = Map::new_map_rooms_and_corridors();
  let (player_x, player_y) = map.rooms[0].center();
  gs.ecs.insert(map);

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

  rltk::main_loop(context, gs)
}
