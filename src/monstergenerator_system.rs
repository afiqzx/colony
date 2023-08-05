use crate::{prelude::*, components::{Attributeset, PersonalityTrait}};

// Make plugin
pub struct MonsterGeneratorPlugin;

impl Plugin for MonsterGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(monster_generator),
        );
    }
}

pub fn monster_generator(
    mut commands: Commands,
    entities: Query<(Entity, &Position), With<MonsterGenerator>>,
    tile_types: Query<(&Position, &TileType)>,
    generated_monsters: Query<(Entity, &GeneratedBy)>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (entity, position) in entities.iter() {
        let mut new_position = *position;
        let dir = random::<i32>() % 4;
        match dir {
            0 => new_position.y += 1,
            1 => new_position.y -= 1,
            2 => new_position.x -= 1,
            3 => new_position.x += 1,
            _ => {}
        }
        let mut can_generate = false;
        for (tile_position, tile_type) in tile_types.iter() {
            let mut p2 = new_position;
            p2.z = 0;
            if *tile_position == p2 && !tile_type.is_wall() {
                can_generate = true;
            }
        }
        for (_ent, parent) in generated_monsters.iter() {
            if parent.entity == entity {
                can_generate = false;
            }
        }

        if !can_generate {
            return;
        }
        let sprite =  TextureAtlasSprite::new(ActorType::Rat.sprite_index());
        commands
            .spawn(SpriteSheetBundle {
                sprite,
                texture_atlas: sprite_sheet.0.clone(),
                ..default()
            })
            .insert(new_position)
            .insert(SizeXYZ::cube(1.1))
            .insert(new_position.to_transform_layer(1.0))
            .insert(GeneratedBy { entity })
            // .insert(MoveTowardsNearestAttackable)
            .insert( PhysicalBody {
                needs_food: None,//Some(NeedsFood { current: 25.1, max: 100.0, rate: 0.1 }),
                needs_entertainment: None,//Some(NeedsEntertainment { current: 100.0, max: 100.0, rate: 0.1 }),
                needs_sleep: None,//Some(NeedsSleep { current: 15.2, max: 100.0, rate: 0.1 }),
                index: 0,
                crisis: None,
                danger: None,
                injured: false,
                afflictions: Vec::new(),
                skillset: Skillset::default(),
                attributes: Attributeset::default(),
            } )
            .insert( Brain {
                motivation: None,
                task: None,
                order: None,
                personality: vec![PersonalityTrait::Creature, PersonalityTrait::Vicious],
            } )
            //.insert( HasName { name: "Wolf".to_string() } )
            ;
        //*position = new_position;
    }

}