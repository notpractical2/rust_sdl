use specs::prelude::*;

use crate::components::*;

const DAMAGE:i32  = 10;

pub struct Damage;

impl <'a> System<'a> for Damage {
    type SystemData = (
        ReadStorage<'a, Position>
    )
}