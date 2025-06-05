use bevy::{
    prelude::*,
};

use crate::screens::Screen;
use crate::game::tray::ContainingTray;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Card>();
    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_cards,
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Card;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref, Reflect)]
#[reflect(Component)]
pub struct Dragging(pub bool);

fn spawn_cards(
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Card"),
        Card,
        ContainingTray::default(),
        Dragging(false),
        Sprite::from_color(Color::WHITE, Vec2::splat(80.0)),
        Transform::from_translation(Vec3::new(-100.0, 0.0, 1.0)),
        Pickable {should_block_lower: false, is_hoverable: true},
    ))
    .observe(drop_card)
    .observe(drag_card);

    commands.spawn((
        Name::new("Card"),
        Card,
        ContainingTray::default(),
        Dragging(false),
        Sprite::from_color(Color::srgb(0.86, 0.08, 0.24), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        Pickable {should_block_lower: false, is_hoverable: true},
    ))
    .observe(drop_card)
    .observe(drag_card);

    commands.spawn((
        Name::new("Card"),
        Card,
        ContainingTray::default(),
        Dragging(false),
        Sprite::from_color(Color::srgb(0.0, 0.5, 0.0), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::new(100.0, 0.0, 1.0)),
        Pickable {should_block_lower: false, is_hoverable: true},
    ))
    .observe(drop_card)
    .observe(drag_card);
}

fn drag_card(
    trigger: Trigger<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    mut dragging: Query<&mut Dragging>
) {
    *dragging.get_mut(trigger.target()).unwrap() = Dragging(true);
    let mut transform = transforms.get_mut(trigger.target()).unwrap();
    let drag = trigger.event();
    transform.translation.x += drag.delta.x;
    transform.translation.y -= drag.delta.y;
    transform.translation.z = 3.0;
}

fn drop_card(
    trigger: Trigger<Pointer<DragEnd>>,
    mut transforms: Query<&mut Transform>,
    mut dragging: Query<&mut Dragging>
) {
    *dragging.get_mut(trigger.target()).unwrap() = Dragging(false);
    let mut transform = transforms.get_mut(trigger.target()).unwrap();
    transform.translation.z = 2.0;
}

