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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[relationship(relationship_target = Tray)]
pub struct ContainingTray(pub Entity);

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Deref)]
#[reflect(Component)]
#[relationship_target(relationship = ContainingTray)]
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
    cards: Query<&Card>,
    mut commands: Commands,
) {
    if cards.contains(trigger.event().dragged) {
        commands.entity(trigger.event().dragged)
            .insert(ContainingTray(trigger.target()));
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
            if **dragging.get(card).unwrap() == false {
                let card_trans = transforms.get(card).unwrap().translation;
                let card_xy = card_trans.truncate();
                let dest = tray_transform.truncate() + Vec2::new((cmp::max(len - 1, 0) as f32 * -CARD_GAP)*0.5 + i as f32 * CARD_GAP, 0.0);

                transforms.get_mut(card).unwrap().translation =
                    card_xy
                    .lerp(dest, 10.0 * time.delta_secs())
                    .extend(card_trans.z);
            }
        }
    }
}

