use crate::cannon::{
    ARENA_HEIGHT, ARENA_WIDTH,
    Ball, BALL_RADIUS, NUGGET_RADIUS
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
        ReadStorage<'s, Ball>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement_y = input.axis_value("vertical");
            let movement_x = input.axis_value("horizontal");

            if let Some(mv_amount) = movement_y {
                let scaled_amount = 1.0 * mv_amount as f32;
                let paddle_y = transform.translation().y;

                transform.set_translation_y((paddle_y + scaled_amount).min(ARENA_HEIGHT - BALL_RADIUS * 0.5)
                .max(BALL_RADIUS * 1.0),);
            }

            if let Some(mv_amount) = movement_x {
                let scaled_amount = 0.5 * mv_amount as f32;
                let paddle_x = transform.translation().x;

                transform.set_translation_x((paddle_x + scaled_amount).min(ARENA_WIDTH - BALL_RADIUS * 0.5)
                .max(BALL_RADIUS * 0.5));
            }
        }
    }
}
