use ggez::{conf::{self, WindowMode, WindowSetup}, ContextBuilder, event};
use bee_battle::game_constants;
use bee_battle::event_handlers::GameConfiguration;
fn main() {

    let c = conf::Conf::new().
    window_mode(WindowMode {
            width: crate::game_constants::SCREEN_WIDTH,
            height: game_constants::SCREEN_HEIGHT,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("Bee_Battles", "Adriyan Ibovski")
    .default_conf(c)
    .window_setup(WindowSetup{
        title: "BeeBattle".to_string(),
        ..Default::default()
    })
    .build()
    .unwrap();

    let game = GameConfiguration::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, game);
}
