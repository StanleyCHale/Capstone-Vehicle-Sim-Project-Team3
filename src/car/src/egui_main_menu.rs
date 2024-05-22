#![allow(dead_code)]
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_integrator::GameState;

// Egui Main Menu Plugin
pub struct EguiMainMenuPlugin;

impl Plugin for EguiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        let main_menu_struct = MainMenu::default();

        app.add_state::<MenuState>()
            .add_plugins(EguiPlugin) // EguiPlugin is needed for literally all bevy_egui functionality
            .insert_resource(main_menu_struct)
            .add_systems(Update, egui_main_menu); // "Main" function for this file
    }
}

// STATES
// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsAudio,
    SettingsVehicle,
    Disabled,
}

// The following code adapted from the bevy_egui examples found here https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs and here https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Resource)]
pub struct MainMenu {
    main: bool,
    settings: bool,
    visible: bool,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            main: true,
            settings: false,
            visible: true,
        }
    }
}

impl MainMenu {
    fn show(&mut self, ctx: &egui::Context, app_exit_events: EventWriter<AppExit>, game_state: ResMut<NextState<GameState>>,) {
        if self.main {
            self.gallery_main_contents(ctx, app_exit_events, game_state);
        }
        if self.settings {
            self.gallery_settings_contents(ctx);
        }
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_main_contents(
        &mut self,
        ctx: &egui::Context,
        app_exit_events: EventWriter<AppExit>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        let Self {
            main: _,
            settings: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Main Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Start Game"))
                    .clicked()
                {
                    //Transition to the "In Game" state
                    game_state.set(GameState::InGame);
                    
                    self.main = false;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Settings"))
                    .clicked()
                {
                    self.settings = true;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Quit"))
                    .clicked()
                {
                    exit_program(app_exit_events);
                }

                ui.end_row();
            });
        });
    }


    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_settings_contents(
        &mut self,
        ctx: &egui::Context,
    ) {
        let Self {
            main: _,
            settings: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Audio Settings"))
                    .clicked()
                {
                    
                    self.main = false;
                    self.settings = true;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Vehicle Settings"))
                    .clicked()
                {
                    println!("egui: Settings Clicked");
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Terrain Settings"))
                    .clicked()
                {
                    println!("egui: Settings Clicked");
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.settings = false;
                    self.main = true;
                }

                ui.end_row();
            });
        });
    }
}

pub fn egui_main_menu(
    mut contexts: EguiContexts,
    mut main_menu_struct: ResMut<MainMenu>,
    app_exit_events: EventWriter<AppExit>,
    game_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    // Show the main menu
    main_menu_struct.show(ctx, app_exit_events, game_state);
}

/*
 * Exits the program when called
 */
pub fn exit_program(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}

/*
 * Inputs: Query for the entities to despawn, commands
 * Outputs: None
 * Description: This function will despawn all entities with the component T
 */
fn despawn_recursive<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
