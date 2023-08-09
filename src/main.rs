use ggez::{conf::{self, WindowMode}, ContextBuilder, event};
use hello_ggez::game_constants;
use hello_ggez::event_handles::MyGame;
fn main() {

    //Make a Context.
    let c = conf::Conf::new().
    window_mode(WindowMode {
            width: crate::game_constants::SCREEN_WIDTH,
            height: game_constants::SCREEN_HEIGHT,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
    .default_conf(c)
    .build()
    .unwrap();

    let game = MyGame::new(&mut ctx).unwrap();

    // Run!
    event::run(ctx, event_loop, game);
}
