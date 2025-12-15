use std::f32::consts::PI;

use bevy::color::palettes::css::{BLUE, RED};
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::*;

use super::components::Bot;

pub fn compute_rays(
    bot: (&Bot, Mut<'_, Transform>, Entity),
    context: &RapierContext,
    gizmos: &mut Gizmos,
) -> Vec<Option<(Entity, f32)>> {
    let (bot, transform, current_bot) = bot;

    let bot_angle =
        transform.rotation.to_axis_angle().0.z * transform.rotation.to_axis_angle().1 + (PI / 2.0);

    (0..bot.class.resolution)
        .map(|ray_id| {
            let ray_dir = Vec2::from_angle(
                bot_angle - (bot.class.view_angle / 2.0)
                    + ray_id as f32 * (bot.class.view_angle / ((bot.class.resolution - 1) as f32)),
            );
            if let Some((entity, toi)) = context.cast_ray(
                transform.translation.truncate(),
                ray_dir,
                bot.class.view_distance,
                false,
                QueryFilter::new().exclude_collider(current_bot),
            ) {
                let hit_point = transform.translation.truncate() + ray_dir * toi;
                gizmos.line(transform.translation, hit_point.extend(0.0), RED);
                Some((entity, toi))
            } else {
                gizmos.line(
                    transform.translation,
                    (transform.translation.truncate() + ray_dir * bot.class.view_distance)
                        .extend(0.0),
                    BLUE,
                );
                None
            }
        })
        .collect::<Vec<Option<(Entity, f32)>>>()
}
