#![allow(dead_code)]
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_integrator::GameState;
use grid_terrain::examples::TerrainPreferences;
use rand::Rng;

use crate::preferences::CarPreferences;

// Egui Main Menu Plugin
pub struct EguiMainMenuPlugin;

impl Plugin for EguiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        let main_menu_struct = MainMenu::default();

        app
            .add_plugins(EguiPlugin) // EguiPlugin is needed for literally all bevy_egui functionality
            .insert_resource(main_menu_struct)
            .add_systems(Update, egui_main_menu); // "Main" function for this file
    }
}

// ENUM
// Enum used for the current menu screen state
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsAudio,
    SettingsVehicle,
    SettingsTerrain,
    Disabled,
}

// The following code adapted from the bevy_egui examples found here https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs and here https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Resource)]
pub struct MainMenu {
    menu: MenuState,
    visible: bool,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            menu: MenuState::Main,
            visible: true,
        }
    }
}

impl MainMenu {
    fn show(
        &mut self, 
        ctx: &egui::Context, 
        app_exit_events: EventWriter<AppExit>, 
        game_state: ResMut<NextState<GameState>>, 
        car_preferences: ResMut<CarPreferences>, 
        terrain_preferences: ResMut<TerrainPreferences>
    ) {
        match self.menu {
            MenuState::Main => {
                self.gallery_main_contents(ctx, app_exit_events, game_state);
            }
            MenuState::Settings => {
                self.gallery_settings_contents(ctx);
            }
            MenuState::SettingsAudio => {
                self.gallery_audio_settings_contents(ctx, car_preferences);
            }
            MenuState::SettingsVehicle => {
                self.gallery_vehicle_settings_contents(ctx, car_preferences);
            }
            MenuState::SettingsTerrain => {
                self.gallery_terrain_settings_contents(ctx, terrain_preferences);
            }
            MenuState::Disabled => {}
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
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

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
                    
                    self.menu = MenuState::Disabled;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Settings"))
                    .clicked()
                {
                    self.menu = MenuState::Settings;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui.add_sized([200.0, 50.0], egui::Button::new("Quit"))
                    .clicked()
                {
                    exit_program(app_exit_events);
                }

                ui.end_row();
            });
        });
    }

    /*
     * This function generates the actual UI of the settings. Any UI element rearranging should be done here.
     */
    fn gallery_settings_contents(
        &mut self,
        ctx: &egui::Context,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

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
                    self.menu = MenuState::SettingsAudio;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Vehicle Settings"))
                    .clicked()
                {
                    self.menu = MenuState::SettingsVehicle;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui.add_sized([200.0, 50.0], egui::Button::new("Terrain Settings"))
                    .clicked()
                {
                    self.menu = MenuState::SettingsTerrain;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Main;
                }

                ui.end_row();
            });
        });
    }

    /*
     * This function generates the actual UI of the settings submenus. Any UI element rearranging should be done here.
     */
    fn gallery_audio_settings_contents(
        &mut self,
        ctx: &egui::Context,
        mut car_preferences: ResMut<CarPreferences>,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Audio Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                //Grab the current value of the volume
                let mut my_f64 = car_preferences.volume;

                let volume_slider = ui.add(
                    egui::Slider::new(&mut my_f64, 0.0..=1.0)
                    .trailing_fill(true)
                    .step_by(0.01)
                    .text("Audio Volume")
                );
                if volume_slider.changed() 
                {
                    //Update the volume
                    car_preferences.volume = my_f64;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Main;
                }

                ui.end_row();
            });
        });
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_vehicle_settings_contents(
        &mut self,
        ctx: &egui::Context,
        mut car_preferences: ResMut<CarPreferences>,
    ) {
        
        let Self {
            menu: _,
            visible: _,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Vehicle Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                //Grab the current value of the chassis mass
                let mut my_f64 = car_preferences.mass;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 1.0..=15000.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Chassis Mass (kg)")
                    )
                    .changed() 
                {
                    //Update the chassis mass
                    car_preferences.mass = my_f64;
                }
                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                //Grab the current value of gravity
                my_f64 = car_preferences.gravity;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 1.0..=200.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Gravity (m/s)")
                    )
                    .changed() 
                {
                    //Update the gravity
                    car_preferences.gravity= my_f64;
                }
                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                //Grab the current value of the max speed
                my_f64 = car_preferences.max_speed;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 1.0..=200.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Max Speed (m/s)")
                    )
                    .changed() 
                {
                    //Update the max speed
                    car_preferences.max_speed= my_f64;
                }
                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                //Grab the current value of the max torque
                my_f64 = car_preferences.max_torque;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 1.0..=10000.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Car Torque (Nm)")
                    )
                    .changed() 
                {
                    //Update the max torque
                    car_preferences.max_torque= my_f64;
                }
                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                //Grab the current value of the friction coefficient
                my_f64 = car_preferences.friction_coefficient;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 0.0..=5.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Friction Coefficient")
                    )
                    .changed() 
                {
                    //Update the friction coefficient
                    car_preferences.friction_coefficient= my_f64;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui.add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Settings;
                }

                ui.end_row();
            });
        });
    }
 
    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_terrain_settings_contents(
        &mut self,
        ctx: &egui::Context,
        mut terrain_preferences: ResMut<TerrainPreferences>,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Terrain Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                
                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Generate New Terrain Seed"))
                    .clicked()
                {
                    //Generate a new terrain seed
                    let mut rng = rand::thread_rng();
                    terrain_preferences.seed = rng.gen();
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                //Grab the current value of the seed
                let mut my_string = terrain_preferences.seed.to_string();

                let response = ui.add(egui::TextEdit::singleline(&mut my_string));

                //If the text box is changed or enter is pressed / lost focus
                if response.changed() || 
                    (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    //Generate a random seed if the input string is empty
                    if my_string.is_empty() {
                        let mut rng = rand::thread_rng();
                        let seed: u32 = rng.gen();
                        my_string = seed.to_string();
                    }

                    //Filter out string to just be numbers
                    my_string = my_string.chars().filter(|char| char.is_digit(10)).collect();

                    //We need to truncate the string to size of a u32
                    if my_string.len() > 9 {
                        my_string = my_string[..9].to_string();
                    }

                    //Set the seed
                    terrain_preferences.seed = my_string.parse().unwrap();
                }
                ui.add_space(20.0);
                ui.end_row();

                //Grab the current value of the grid size
                let mut my_f64 = terrain_preferences.grid_size;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 1.0..=4000.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Grid Size")
                    )
                    .changed() 
                {
                    //Update the grid size
                    terrain_preferences.grid_size = my_f64;
                }

                //Grab the current value of the subdivisions
                my_f64 = terrain_preferences.subdivisions;
                if ui.add(
                        egui::Slider::new(&mut my_f64, 1.0..=2048.0)
                        .trailing_fill(true)
                        .step_by(0.01)
                        .text("Grid Subdivisions"))
                    .changed()
                {
                    //Update the subdivisions
                    terrain_preferences.subdivisions = my_f64;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui.add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Main;
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
    car_preferences: ResMut<CarPreferences>,
    terrain_preferences: ResMut<TerrainPreferences>
) {
    let ctx = contexts.ctx_mut();

    // Modify text size for all pages using a custom style
    let mut custom_style = egui::Style::default();

    custom_style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
    ]
    .into();

    ctx.set_style(custom_style);    // Set our custom style to be the default style

    // Show the main menu
    main_menu_struct.show(ctx, app_exit_events, game_state, car_preferences, terrain_preferences);
}

/*
 * Exits the program when called
 */
pub fn exit_program(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
