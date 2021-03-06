use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use Ball;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (ball, local) in (&balls, &mut locals).join() {
            local.translation[0] += ball.velocity[0] * time.delta_seconds();
            local.translation[1] += ball.velocity[1] * time.delta_seconds();
        }
    }
}
