mod camera;
mod map;
mod map_builder;
mod player;

pub mod prelude {
    // Export bracket-lib to extend this library
    pub use bracket_lib::prelude::*;

    // Available constants to the public
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    // Internal libraries made public via this prelude
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

// Use all exports from the "dungeoncrawl::prelude::*" library in this file
use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
