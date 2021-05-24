use crate::{Map, Monster, Name, Position, Viewshed};
use specs::prelude::*;

use rltk::{console, Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    WriteExpect<'a, Map>,
    ReadExpect<'a, Point>,
    WriteStorage<'a, Viewshed>,
    ReadStorage<'a, Monster>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, Position>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

    for (mut viewshed, _monster, name, mut pos) in
      (&mut viewshed, &monster, &name, &mut position).join()
    {
      if viewshed.visible_tiles.contains(&*player_pos) {

      }
    }
  }
}
