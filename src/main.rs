mod map;
mod player;

pub mod prelude {
    // Export bracket-lib to extend this library
    pub use bracket_lib::prelude::*;

    // Available constants to the public
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    // Internal libraries made public via this prelude
    pub use crate::map::*;
    pub use crate::player::*;
}

// Use all exports from the "dungeoncrawl::prelude::*" library in this file
use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;

    main_loop(context, State::new())
}
