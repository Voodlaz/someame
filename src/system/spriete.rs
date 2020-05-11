use crate::cannon::{
    ARENA_HEIGHT, ARENA_WIDTH,
    PADDLE_HEIGHT, PADDLE_WIDTH,
    Paddle
};

use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement_y = input.axis_value("vertical");
            let movement_x = input.axis_value("horizontal");

            if let Some(mv_amount) = movement_y {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;

                transform.set_translation_y((paddle_y + scaled_amount).min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                .max(PADDLE_HEIGHT * 0.5),);
            }

            if let Some(mv_amount) = movement_x {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_x = transform.translation().x;

                transform.set_translation_x((paddle_x + scaled_amount).min(ARENA_WIDTH - PADDLE_WIDTH * 0.5)
                .max(PADDLE_WIDTH * 0.5),);
            }
        }
    }
}
