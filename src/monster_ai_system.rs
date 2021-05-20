use specs::prelude::*;
use super::{Monster, Viewshed};
use rltk::{Point, console};


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadExpect<'a, Point>,
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Monster>,
  );

  fn run(&mut self, data: Self::SystemData){
    let (player_pos, viewshed, monster) = data;

    for (viewshed, monster) in (&viewshed, &monster).join() {
      if viewshed.visible_tiles.contains(&*player_pos){
        console::log("I see you!")
      }
    }
  }
}
