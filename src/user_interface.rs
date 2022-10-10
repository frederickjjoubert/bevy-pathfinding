use crate::{PlacementMode, ResetEvent, SolveEvent, StepEvent};
use bevy::prelude::*;
use bevy::ui::Display::Flex;

use super::GameState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// === Components ===
#[derive(Component, Debug)]
pub struct PathButton {}

#[derive(Component, Debug)]
pub struct ObstacleButton {}

#[derive(Component, Debug)]
pub struct StartButton {}

#[derive(Component, Debug)]
pub struct GoalButton {}

#[derive(Component, Debug)]
pub struct StepButton {}

#[derive(Component, Debug)]
pub struct SolveButton {}

#[derive(Component, Debug)]
pub struct ResetButton {}

#[derive(Component, Debug)]
pub struct CurrentAlgorithmText {}

#[derive(Component, Debug)]
pub struct CycleAlgorithmLeftButton {}

#[derive(Component, Debug)]
pub struct CycleAlgorithmRightButton {}

// === Events ===
pub struct UserInterfaceInteractionEvent {} // Empty Event

// === Systems ===
pub fn setup_user_interface(mut commands: Commands, asset_server: Res<AssetServer>) {
    // === Styles ===
    let button_container_style = Style {
        display: Flex,
        flex_direction: FlexDirection::Row,
        size: Size::new(Val::Auto, Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_style = Style {
        display: Flex,
        flex_direction: FlexDirection::Row,
        size: Size::new(Val::Auto, Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::new(Val::Px(16.0), Val::Px(16.0), Val::Px(16.0), Val::Px(16.0)),
        padding: UiRect::new(Val::Px(16.0), Val::Px(16.0), Val::Px(16.0), Val::Px(16.0)),
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans/FiraSans-Bold.ttf"),
        font_size: 16.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    // === Create UI ===
    let root_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Root Container"))
        .id();

    let bottom_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Bottom Container"))
        .id();

    let spacer = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                flex_grow: 1.0,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Spacer"))
        .id();

    let top_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Top Container"))
        .id();

    // Path Button
    let path_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Path Button Container"))
        .id();

    let path_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Path Button"))
        .insert(PathButton {})
        .id();

    let path_button_text = commands
        .spawn_bundle(TextBundle::from_section("Open", button_text_style.clone()))
        .id();

    commands
        .entity(path_button)
        .push_children(&[path_button_text]);
    commands
        .entity(path_button_container)
        .push_children(&[path_button]);

    // Obstacle Button
    let obstacle_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Obstacle Button Container"))
        .id();

    let obstacle_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Obstacle Button"))
        .insert(ObstacleButton {})
        .id();

    let obstacle_button_text = commands
        .spawn_bundle(TextBundle::from_section(
            "Obstacle",
            button_text_style.clone(),
        ))
        .id();

    commands
        .entity(obstacle_button)
        .push_children(&[obstacle_button_text]);
    commands
        .entity(obstacle_button_container)
        .push_children(&[obstacle_button]);

    // Start Button
    let start_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Start Button Container"))
        .id();

    let start_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Start Button"))
        .insert(StartButton {})
        .id();

    let start_button_text = commands
        .spawn_bundle(TextBundle::from_section("Start", button_text_style.clone()))
        .id();

    commands
        .entity(start_button)
        .push_children(&[start_button_text]);
    commands
        .entity(start_button_container)
        .push_children(&[start_button]);

    // Goal Button
    let goal_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Goal Button Container"))
        .id();

    let goal_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Goal Button"))
        .insert(GoalButton {})
        .id();

    let goal_button_text = commands
        .spawn_bundle(TextBundle::from_section("Goal", button_text_style.clone()))
        .id();

    commands
        .entity(goal_button)
        .push_children(&[goal_button_text]);
    commands
        .entity(goal_button_container)
        .push_children(&[goal_button]);

    commands.entity(bottom_container).push_children(&[
        path_button_container,
        obstacle_button_container,
        start_button_container,
        goal_button_container,
    ]);

    // Step Button
    let step_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Step Button Container"))
        .id();

    let step_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Step Button"))
        .insert(StepButton {})
        .id();

    let step_button_text = commands
        .spawn_bundle(TextBundle::from_section("Step", button_text_style.clone()))
        .id();

    commands
        .entity(step_button)
        .push_children(&[step_button_text]);
    commands
        .entity(step_button_container)
        .push_children(&[step_button]);

    // Solve Button
    let solve_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Solve Button Container"))
        .id();

    let solve_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Solve Button"))
        .insert(SolveButton {})
        .id();

    let solve_button_text = commands
        .spawn_bundle(TextBundle::from_section("Solve", button_text_style.clone()))
        .id();

    commands
        .entity(solve_button)
        .push_children(&[solve_button_text]);
    commands
        .entity(solve_button_container)
        .push_children(&[solve_button]);

    // Reset Button
    let reset_button_container = commands
        .spawn_bundle(NodeBundle {
            style: button_container_style.clone(),
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Reset Button Container"))
        .id();

    let reset_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Reset Button"))
        .insert(ResetButton {})
        .id();

    let reset_button_text = commands
        .spawn_bundle(TextBundle::from_section("Reset", button_text_style.clone()))
        .id();

    commands
        .entity(reset_button)
        .push_children(&[reset_button_text]);
    commands
        .entity(reset_button_container)
        .push_children(&[reset_button]);

    // Title
    let title_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Title Container"))
        .id();

    let title_text = commands
        .spawn_bundle(TextBundle::from_section(
            "Bevy Pathfinding Example Project",
            TextStyle {
                font: asset_server.load("fonts/FiraSans/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .id();

    commands
        .entity(title_container)
        .push_children(&[title_text]);

    // Algorithm Cycler
    let algorithm_cycler_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .id();

    // Cycle Algorithm Left Button
    let cycle_algorithm_left_button_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .id();

    let cycle_algorithm_left_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Cycle Algorithm Left Button"))
        .insert(CycleAlgorithmLeftButton {})
        .id();

    let cycle_algorithm_left_text = commands
        .spawn_bundle(TextBundle::from_section("<", button_text_style.clone()))
        .id();

    commands
        .entity(cycle_algorithm_left_button)
        .push_children(&[cycle_algorithm_left_text]);
    commands
        .entity(cycle_algorithm_left_button_container)
        .push_children(&[cycle_algorithm_left_button]);

    // Current Algorithm Text
    let current_algorithm_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Current Algorithm Container"))
        .id();

    let current_algorithm_text = commands
        .spawn_bundle(TextBundle::from_section(
            "BFS",
            TextStyle {
                font: asset_server.load("fonts/FiraSans/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .insert(CurrentAlgorithmText {})
        .id();

    commands
        .entity(current_algorithm_container)
        .push_children(&[current_algorithm_text]);

    // Cycle Algorithm Right Button
    let cycle_algorithm_right_button_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Flex,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .id();

    let cycle_algorithm_right_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(Name::new("Cycle Algorithm Right Button"))
        .insert(CycleAlgorithmRightButton {})
        .id();

    let cycle_algorithm_right_text = commands
        .spawn_bundle(TextBundle::from_section(">", button_text_style.clone()))
        .id();

    commands
        .entity(cycle_algorithm_right_button)
        .push_children(&[cycle_algorithm_right_text]);
    commands
        .entity(cycle_algorithm_right_button_container)
        .push_children(&[cycle_algorithm_right_button]);

    commands.entity(algorithm_cycler_container).push_children(&[
        cycle_algorithm_left_button_container,
        current_algorithm_container,
        cycle_algorithm_right_button_container,
    ]);

    commands.entity(top_container).push_children(&[
        step_button_container,
        solve_button_container,
        reset_button_container,
        title_container,
        algorithm_cycler_container,
    ]);

    commands
        .entity(root_container)
        .push_children(&[bottom_container, spacer, top_container]);
}

pub fn path_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut path_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<PathButton>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in path_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.placement_mode = PlacementMode::Path;
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn obstacle_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut obstacle_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ObstacleButton>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in obstacle_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.placement_mode = PlacementMode::Obstacle;
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn start_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut start_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in start_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.placement_mode = PlacementMode::Start;
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn goal_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut goal_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<GoalButton>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in goal_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.placement_mode = PlacementMode::Goal;
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn step_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut step_event_writer: EventWriter<StepEvent>,
    mut step_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<StepButton>),
    >,
) {
    for (interaction, mut color) in step_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
                step_event_writer.send(StepEvent {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn solve_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut solve_event_writer: EventWriter<SolveEvent>,
    mut solve_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<SolveButton>),
    >,
) {
    for (interaction, mut color) in solve_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
                solve_event_writer.send(SolveEvent {})
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn reset_button_system(
    mut user_interface_interaction_event_writer: EventWriter<UserInterfaceInteractionEvent>,
    mut reset_event_writer: EventWriter<ResetEvent>,
    mut reset_button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in reset_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                user_interface_interaction_event_writer.send(UserInterfaceInteractionEvent {});
                reset_event_writer.send(ResetEvent {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
