use crate::prelude::*;

// pub fn task_system_play(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut Brain, &Position), Without<Targeting>>,
//     mut query_bed: Query<(Entity, &Position, &Bed)>,
// ) {
//     for (entity, mut brain, position) in query.iter_mut() {
//         if brain.task != Some(Task::Play) { continue; }

//     }
// }

pub fn task_system_playing(
    _commands: Commands,
    mut query: Query<(&mut Brain, &mut PhysicalBody)>
) {
    for (mut brain, mut physical_body) in query.iter_mut() {
        if brain.task != Some(Task::Play) { continue; }
        if let Some(n) = &mut physical_body.needs_entertainment {
            n.current += 10.0;
            if n.current >= n.max {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
}