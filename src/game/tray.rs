use bevy::{
    prelude::*,
};
use std::cmp;
use crate::screens::Screen;
use crate::game::cards::Card;
use crate::game::cards::Dragging;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Tray>();
    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_trays,
    );
    app.add_systems(
        Update,
        update_cards_pos,
    );
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct ContainingTray(Option<Entity>);

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Tray(Vec<Entity>);

fn spawn_trays(
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Tray"),
        Tray::default(),
        Sprite::from_color(Color::BLACK, Vec2::new(320.0, 200.0)),
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
        Pickable::default(),
    ))
    .observe(add_card_to_tray);

    commands.spawn((
        Name::new("Tray"),
        Tray::default(),
        Sprite::from_color(Color::BLACK, Vec2::new(320.0, 200.0)),
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        Pickable::default(),
    ))
    .observe(add_card_to_tray);
}

fn add_card_to_tray(
    trigger: Trigger<Pointer<DragOver>>,
    cards_query: Query<&Card>,
    mut containing_trays_query: Query<&mut ContainingTray>,
    mut trays_query: Query<&mut Tray>,
    tray_transforms_query: Query<&Transform, With<Tray>>,
) {
    let dragged_entity = trigger.event().dragged;
    let target_entity = trigger.target();
    let hit_position = trigger.event().hit.position.unwrap(); //Vec3

    if cards_query.contains(dragged_entity) {
        if let Ok(mut containing_tray) =
            containing_trays_query.get_mut(dragged_entity)
        {
            if let Some(prev_tray_entity) = containing_tray.0 {
                if let Ok(mut prev_tray) = trays_query.get_mut(prev_tray_entity) {
                    prev_tray.retain(|&x| x != dragged_entity);
                }
            }
            containing_tray.0 = Some(target_entity);
        }

        if let Ok(mut new_tray) = trays_query.get_mut(target_entity) {
            let tray_transform = tray_transforms_query.get(target_entity).unwrap();
            let tray_center_x = tray_transform.translation.x;
            let num_cards = new_tray.len();

            let mut insert_index = num_cards; // Default to inserting at the end

            if num_cards > 0 {
                let total_width = (num_cards as f32) * CARD_GAP;
                let start_x = tray_center_x - (total_width * 0.5);

                for i in 0..num_cards {
                    let card_x_pos = start_x + (i as f32 * CARD_GAP) + (CARD_GAP * 0.5); // Midpoint of the card slot
                    if hit_position.x < card_x_pos {
                        insert_index = i;
                        break;
                    }
                }
            }

            new_tray.insert(insert_index, dragged_entity);
        }
    }
}
const CARD_GAP: f32 = 90.0;

fn update_cards_pos(
    time: Res<Time>,
    trays: Query<(Entity, &Tray)>,
    dragging: Query<&Dragging>,
    mut transforms: Query<&mut Transform>,
) {
    for (e, tray) in &trays {
        let tray_transform = transforms.get(e).unwrap().translation;
        let len = tray.len();
        for (i, card) in tray.iter().enumerate() {
            if !(**dragging.get(*card).unwrap()) {
                let card_trans = transforms.get(*card).unwrap().translation;
                let card_xy = card_trans.truncate();
                let dest = tray_transform.truncate() + Vec2::new((cmp::max(len - 1, 0) as f32 * -CARD_GAP)*0.5 + i as f32 * CARD_GAP, 0.0);

                transforms.get_mut(*card).unwrap().translation =
                    card_xy
                    .lerp(dest, 10.0 * time.delta_secs())
                    .extend(card_trans.z);
            }
        }
    }
}

