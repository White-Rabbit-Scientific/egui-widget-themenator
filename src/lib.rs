use egui::{
    epaint::AlphaFromCoverage,
    style::{HandleShape, NumericColorSpace, TextCursorStyle, WidgetVisuals, Widgets},
    Color32, CornerRadius, Id, Stroke, Style, Widget,
};
use serde::{Deserialize, Serialize};
// use std::time::Duration; // TODO cursor flashing

// ----------------------------------------------------------
// Extension trait to add Catppuccin colors to egui's Color32
// ----------------------------------------------------------
#[rustfmt::skip]
pub trait CatppuccinColors {
    fn latte() ->     CatppuccinPalette;
    fn frappe() ->    CatppuccinPalette;
    fn macchiato() -> CatppuccinPalette;
    fn mocha() ->     CatppuccinPalette;
}

// ---------------------------------
// Extended Catppuccin Color Palette
// ---------------------------------
#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub struct CatppuccinPalette {
    pub rosewater: Color32,
    pub flamingo:  Color32,
    pub pink:      Color32,
    pub mauve:     Color32,
    pub red:       Color32,
    pub maroon:    Color32,
    pub peach:     Color32,
    pub yellow:    Color32,
    pub green:     Color32,
    pub teal:      Color32,
    pub sky:       Color32,
    pub sapphire:  Color32,
    pub blue:      Color32,
    pub lavender:  Color32,
    pub text:      Color32,
    pub subtext1:  Color32,
    pub subtext0:  Color32,
    pub overlay2:  Color32,
    pub overlay1:  Color32,
    pub overlay0:  Color32,
    pub surface2:  Color32,
    pub surface1:  Color32,
    pub surface0:  Color32,
    pub base:      Color32,
    pub mantle:    Color32,
    pub crust:     Color32,
}

#[rustfmt::skip]
// ---------------------------
// Implement trait for Color32
// ---------------------------
impl CatppuccinColors for Color32 {
    fn latte() -> CatppuccinPalette {
        CatppuccinPalette {
            rosewater: Color32::from_rgb(0xDC, 0x8A, 0x78),
            flamingo:  Color32::from_rgb(0xDD, 0x78, 0x78),
            pink:      Color32::from_rgb(0xEA, 0x76, 0xCB),
            mauve:     Color32::from_rgb(0x88, 0x39, 0xEF),
            red:       Color32::from_rgb(0xD2, 0x0F, 0x39),
            maroon:    Color32::from_rgb(0xE6, 0x45, 0x53),
            peach:     Color32::from_rgb(0xFE, 0x64, 0x0B),
            yellow:    Color32::from_rgb(0xDF, 0x8E, 0x1D),
            green:     Color32::from_rgb(0x40, 0xA0, 0x2B),
            teal:      Color32::from_rgb(0x17, 0x92, 0x99),
            sky:       Color32::from_rgb(0x04, 0xA5, 0xE5),
            sapphire:  Color32::from_rgb(0x20, 0x9F, 0xB5),
            blue:      Color32::from_rgb(0x1E, 0x66, 0xF5),
            lavender:  Color32::from_rgb(0x72, 0x87, 0xFD),
            text:      Color32::from_rgb(0x4C, 0x4F, 0x69),
            subtext1:  Color32::from_rgb(0x5C, 0x5F, 0x77),
            subtext0:  Color32::from_rgb(0x6C, 0x6F, 0x85),
            overlay2:  Color32::from_rgb(0x7C, 0x7F, 0x93),
            overlay1:  Color32::from_rgb(0x8C, 0x8F, 0xA1),
            overlay0:  Color32::from_rgb(0x9C, 0xA0, 0xB0),
            surface2:  Color32::from_rgb(0xAC, 0xB0, 0xBE),
            surface1:  Color32::from_rgb(0xBC, 0xC0, 0xCC),
            surface0:  Color32::from_rgb(0xCC, 0xD0, 0xDA),
            base:      Color32::from_rgb(0xEF, 0xF1, 0xF5),
            mantle:    Color32::from_rgb(0xE6, 0xE9, 0xEF),
            crust:     Color32::from_rgb(0xDC, 0xE0, 0xE8),
        }
    }

    #[rustfmt::skip]
    fn frappe() -> CatppuccinPalette {
        CatppuccinPalette {
            rosewater: Color32::from_rgb(0xF2, 0xD5, 0xCF),
            flamingo:  Color32::from_rgb(0xEE, 0xBE, 0xBE),
            pink:      Color32::from_rgb(0xF4, 0xB8, 0xE4),
            mauve:     Color32::from_rgb(0xCA, 0x9E, 0xE6),
            red:       Color32::from_rgb(0xE7, 0x82, 0x84),
            maroon:    Color32::from_rgb(0xEA, 0x99, 0x9C),
            peach:     Color32::from_rgb(0xEF, 0x9F, 0x76),
            yellow:    Color32::from_rgb(0xE5, 0xC8, 0x90),
            green:     Color32::from_rgb(0xA6, 0xD1, 0x89),
            teal:      Color32::from_rgb(0x81, 0xC8, 0xBE),
            sky:       Color32::from_rgb(0x99, 0xD1, 0xDB),
            sapphire:  Color32::from_rgb(0x85, 0xC1, 0xDC),
            blue:      Color32::from_rgb(0x8C, 0xAA, 0xEE),
            lavender:  Color32::from_rgb(0xBA, 0xBB, 0xF1),
            text:      Color32::from_rgb(0xC6, 0xD0, 0xF5),
            subtext1:  Color32::from_rgb(0xB5, 0xBF, 0xE2),
            subtext0:  Color32::from_rgb(0xA5, 0xAD, 0xCE),
            overlay2:  Color32::from_rgb(0x94, 0x9C, 0xBB),
            overlay1:  Color32::from_rgb(0x83, 0x8B, 0xA7),
            overlay0:  Color32::from_rgb(0x73, 0x79, 0x94),
            surface2:  Color32::from_rgb(0x62, 0x68, 0x80),
            surface1:  Color32::from_rgb(0x51, 0x57, 0x6D),
            surface0:  Color32::from_rgb(0x41, 0x45, 0x59),
            base:      Color32::from_rgb(0x30, 0x34, 0x46),
            mantle:    Color32::from_rgb(0x29, 0x2C, 0x3C),
            crust:     Color32::from_rgb(0x23, 0x26, 0x34),
        }
    }

    #[rustfmt::skip]
    fn macchiato() -> CatppuccinPalette {
        CatppuccinPalette {
            rosewater: Color32::from_rgb(0xF4, 0xDB, 0xD6),
            flamingo:  Color32::from_rgb(0xF0, 0xC6, 0xC6),
            pink:      Color32::from_rgb(0xF5, 0xBD, 0xE6),
            mauve:     Color32::from_rgb(0xC6, 0xA0, 0xF6),
            red:       Color32::from_rgb(0xED, 0x87, 0x96),
            maroon:    Color32::from_rgb(0xEE, 0x99, 0xA0),
            peach:     Color32::from_rgb(0xF5, 0xA9, 0x7F),
            yellow:    Color32::from_rgb(0xEE, 0xD4, 0x9F),
            green:     Color32::from_rgb(0xA6, 0xDA, 0x95),
            teal:      Color32::from_rgb(0x8B, 0xD5, 0xCA),
            sky:       Color32::from_rgb(0x91, 0xD7, 0xE3),
            sapphire:  Color32::from_rgb(0x7D, 0xC4, 0xE4),
            blue:      Color32::from_rgb(0x8A, 0xAD, 0xF4),
            lavender:  Color32::from_rgb(0xB7, 0xBD, 0xF8),
            text:      Color32::from_rgb(0xCA, 0xD3, 0xF5),
            subtext1:  Color32::from_rgb(0xB8, 0xC0, 0xE0),
            subtext0:  Color32::from_rgb(0xA5, 0xAD, 0xCB),
            overlay2:  Color32::from_rgb(0x93, 0x9A, 0xB7),
            overlay1:  Color32::from_rgb(0x80, 0x87, 0xA2),
            overlay0:  Color32::from_rgb(0x6E, 0x73, 0x8D),
            surface2:  Color32::from_rgb(0x5B, 0x60, 0x78),
            surface1:  Color32::from_rgb(0x49, 0x4D, 0x64),
            surface0:  Color32::from_rgb(0x36, 0x3A, 0x4F),
            base:      Color32::from_rgb(0x24, 0x27, 0x3A),
            mantle:    Color32::from_rgb(0x1E, 0x20, 0x30),
            crust:     Color32::from_rgb(0x18, 0x19, 0x26),
        }
    }

    #[rustfmt::skip]
    fn mocha() -> CatppuccinPalette {
        CatppuccinPalette {
            rosewater: Color32::from_rgb(0xF5, 0xE0, 0xDC),
            flamingo:  Color32::from_rgb(0xF2, 0xCD, 0xCD),
            pink:      Color32::from_rgb(0xF5, 0xC2, 0xE7),
            mauve:     Color32::from_rgb(0xCB, 0xA6, 0xF7),
            red:       Color32::from_rgb(0xF3, 0x8B, 0xA8),
            maroon:    Color32::from_rgb(0xEB, 0xA0, 0xAC),
            peach:     Color32::from_rgb(0xFA, 0xB3, 0x87),
            yellow:    Color32::from_rgb(0xF9, 0xE2, 0xAF),
            green:     Color32::from_rgb(0xA6, 0xE3, 0xA1),
            teal:      Color32::from_rgb(0x94, 0xE2, 0xD5),
            sky:       Color32::from_rgb(0x89, 0xDC, 0xEB),
            sapphire:  Color32::from_rgb(0x74, 0xC7, 0xEC),
            blue:      Color32::from_rgb(0x89, 0xB4, 0xFA),
            lavender:  Color32::from_rgb(0xB4, 0xBE, 0xFE),
            text:      Color32::from_rgb(0xCD, 0xD6, 0xF4),
            subtext1:  Color32::from_rgb(0xBA, 0xC2, 0xDE),
            subtext0:  Color32::from_rgb(0xA6, 0xAD, 0xC8),
            overlay2:  Color32::from_rgb(0x93, 0x99, 0xB2),
            overlay1:  Color32::from_rgb(0x7F, 0x84, 0x9C),
            overlay0:  Color32::from_rgb(0x6C, 0x70, 0x86),
            surface2:  Color32::from_rgb(0x58, 0x5B, 0x70),
            surface1:  Color32::from_rgb(0x45, 0x47, 0x5A),
            surface0:  Color32::from_rgb(0x31, 0x32, 0x44),
            base:      Color32::from_rgb(0x1E, 0x1E, 0x2E),
            mantle:    Color32::from_rgb(0x18, 0x18, 0x25),
            crust:     Color32::from_rgb(0x11, 0x11, 0x1B),
        }
    }
}
// -----------------
// Catppuccin themes
// -----------------
#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Default, PartialEq)]
pub enum ThemeName {
    #[default]
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl ThemeName {
    // ------------------------------
    // Return theme's light/dark mode
    // ------------------------------
    pub fn is_dark(&self) -> bool {
        !matches!(self, ThemeName::Latte)
    }

    #[rustfmt::skip]
    // ----------------------------------------------
    // Call trait's associated method through Color32
    // ----------------------------------------------
    pub fn palette(&self) -> CatppuccinPalette {
        match self {
            ThemeName::Latte =>     Color32::latte(),
            ThemeName::Frappe =>    Color32::frappe(),
            ThemeName::Macchiato => Color32::macchiato(),
            ThemeName::Mocha =>     Color32::mocha(),
        }
    }

    pub fn apply(&self, style: &mut Style) {
        let palette = self.palette();
        apply_catppuccin_theme(style, self.is_dark(), palette);
    }

    // ------------------------------------------------------
    // Get the currently active Catppuccin theme from context
    // ------------------------------------------------------
    pub fn current(ctx: &egui::Context) -> Self {
        ctx.data(|d| d.get_temp(egui::Id::new("catppuccin_theme")))
            .unwrap_or_default()
    }

    // -----------------------------------------------------
    // Set the current theme in both egui and context memory
    // -----------------------------------------------------
    pub fn set(self, ctx: &egui::Context) {
        let base_theme = if self.is_dark() {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        };

        ctx.style_mut_of(base_theme, |style| {
            self.apply(style);
        });
        ctx.set_theme(base_theme);

        // ------------------------------
        // Store in context for retrieval
        // ------------------------------
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("catppuccin_theme"), self));
    }

    // Returns static slice of all enum variants
    pub fn all() -> &'static [ThemeName] {
        &[
            ThemeName::Latte,
            ThemeName::Frappe,
            ThemeName::Macchiato,
            ThemeName::Mocha,
        ]
    }

    #[rustfmt::skip]
    pub fn theme_lbl_txt(&self) -> &'static str {
        match self {
            ThemeName::Latte =>     "Light",
            ThemeName::Frappe =>    "Dark 1",
            ThemeName::Macchiato => "Dark 2",
            ThemeName::Mocha =>     "Dark 3",
        }
    }
}

// -----------------
// Theme Application
// -----------------
fn apply_catppuccin_theme(style: &mut Style, dark_mode: bool, p: CatppuccinPalette) {
    const CORNER_RADIUS: u8 = 0;

    #[rustfmt::skip]
    let widgets = Widgets {
        // The style of a widget that you cannot interact with.
        noninteractive: WidgetVisuals {
            bg_fill:       p.overlay0,
            weak_bg_fill:  p.overlay0,
            bg_stroke:     Stroke::new(1.0, p.overlay0),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        // The style of an interactive widget, such as a button at rest (not selected).
        inactive: WidgetVisuals {
            bg_fill:       p.mantle,
            weak_bg_fill:  p.mantle,
            bg_stroke:     Stroke::new(1.0, p.overlay1),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        // The style of an interactive widget while you hover it, or when it is highlighted.
        hovered: WidgetVisuals {
            bg_fill:       p.mantle,
            weak_bg_fill:  p.mantle,
            bg_stroke:     Stroke::new(1.0, p.overlay2),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        // The style of an interactive widget as you are clicking or dragging it (selected).
        active: WidgetVisuals {
            bg_fill:       p.base,
            weak_bg_fill:  p.base,
            bg_stroke:     Stroke::new(1.0, p.overlay2),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        // The style of a button that has an open menu beneath it (e.g. a combo-box).
        open: WidgetVisuals {
            bg_fill:       p.mantle,
            weak_bg_fill:  p.mantle,
            bg_stroke:     Stroke::new(1.0, p.text),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
    };

    style.visuals.dark_mode = dark_mode;
    style.visuals.text_alpha_from_coverage = if dark_mode {
        AlphaFromCoverage::TwoCoverageMinusCoverageSq
    } else {
        AlphaFromCoverage::Linear
    };
    style.visuals.weak_text_alpha = 0.4;
    style.visuals.weak_text_color = Some(p.surface2);
    style.visuals.widgets = widgets;
    style.visuals.hyperlink_color = p.blue;
    style.visuals.faint_bg_color = p.surface0;
    style.visuals.extreme_bg_color = p.crust;
    style.visuals.text_edit_bg_color = Some(p.mantle);
    style.visuals.code_bg_color = p.mantle;
    style.visuals.warn_fg_color = p.yellow;
    style.visuals.error_fg_color = p.red;
    style.visuals.window_fill = p.base;
    style.visuals.panel_fill = p.base;
    style.visuals.text_cursor = TextCursorStyle {
        stroke: Stroke::new(2.0, p.rosewater),
        ..Default::default()
    };
    style.visuals.button_frame = true;
    style.visuals.handle_shape = HandleShape::Circle;
    style.visuals.numeric_color_space = NumericColorSpace::GammaByte;
}

// ---------------------------------
// Theme Widget with Builder Pattern
// ---------------------------------
#[derive(Default)]
pub struct ThemeWidget {
    label: Option<String>,
    show_labels: bool,
}

impl ThemeWidget {
    pub fn new() -> Self {
        Self {
            label: None,
            show_labels: true,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }
}

// Store the theme name in egui's persistent storage.
// This saves the current theme across app restarts.
// The theme is loaded and saved automatically (serde).
#[derive(Default, Clone, Serialize, Deserialize, Copy, PartialEq)]
struct PersistedStorage {
    theme_name: ThemeName,
}

// Store the theme name in egui's non-persistent storage.
// Non-persistent has lower CPU overhead than persistent
// so we store the theme there while running.
#[derive(Default, Clone, Copy, Debug)]
struct NonPersistedStorage {
    run_once: bool,
    theme_name: ThemeName,
}

impl Widget for ThemeWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        const ID_THEME_NAME: &str = "theme_name";

        // -------------------------------
        // Create a hashed id for run_once
        // -------------------------------
        let non_persisted_id = Id::new("run_once");

        // -------------------------
        // Get non-persisted storage
        // -------------------------
        let mut non_persisted_storage = ui
            .ctx()
            .data_mut(|d| d.get_temp::<NonPersistedStorage>(non_persisted_id))
            .unwrap_or_default();

        // ----------------------------
        // Run once, first time through
        // ----------------------------
        if non_persisted_storage.run_once == false {
            // First time, so capture persisted theme
            let persisted_id = Id::new(ID_THEME_NAME);
            let persisted_storage = ui
                .ctx()
                .data_mut(|d| d.get_persisted::<PersistedStorage>(persisted_id))
                .unwrap_or_default();

            // ----------------------
            // Populate struct fields
            // ----------------------
            non_persisted_storage.run_once = true;
            non_persisted_storage.theme_name = persisted_storage.theme_name;

            // --------------------------------------------
            // Save current theme to non-persistent storage
            // --------------------------------------------
            ui.ctx()
                .data_mut(|d| d.insert_temp(non_persisted_id, non_persisted_storage));

            // ----------------------------
            // Update non-persisted storage
            // ----------------------------
            non_persisted_storage.theme_name.set(ui.ctx());
        };

        // ---------------
        // Draw the widget
        // ---------------
        ui.horizontal(|ui| {
            if let Some(label) = self.label {
                ui.label(label);
            }
            // Get the system theme
            // let system_theme = ui.ctx().input(|i| i.raw.system_theme);
            // System theme icon
            // ui.label("\u{1F4BB}")

            let next_state;
            //---------------------------------------------------------------------
            // Display current state and get its response, returning the next state
            // --------------------------------------------------------------------
            let resp = match non_persisted_storage.theme_name {
                ThemeName::Latte => {
                    next_state = ThemeName::Frappe;
                    ui.label("\u{2600}")
                        .on_hover_text("Latte theme")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                }
                ThemeName::Frappe => {
                    next_state = ThemeName::Macchiato;
                    ui.label("\u{1F319}")
                        .on_hover_text("Frappe theme")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                }
                ThemeName::Macchiato => {
                    next_state = ThemeName::Mocha;
                    ui.label("\u{1F319}")
                        .on_hover_text("Macchiato theme")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                }
                ThemeName::Mocha => {
                    next_state = ThemeName::Latte;
                    ui.label("\u{1F319}")
                        .on_hover_text("Mocha theme")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                }
            };

            //-------------------------------
            // If clicked, move to next state
            // ------------------------------
            if resp.clicked() {
                non_persisted_storage.theme_name = next_state;

                //---------------------------------
                // Update egui to use the new theme
                // --------------------------------
                match non_persisted_storage.theme_name {
                    ThemeName::Latte => {
                        ThemeName::Latte.set(ui.ctx());
                    }
                    ThemeName::Frappe => {
                        ThemeName::Frappe.set(ui.ctx());
                    }
                    ThemeName::Macchiato => {
                        ThemeName::Macchiato.set(ui.ctx());
                    }
                    ThemeName::Mocha => {
                        ThemeName::Mocha.set(ui.ctx());
                    }
                };
                // ---------------------------------------------------
                // Save current theme to egui's non-persistent storage
                // ---------------------------------------------------
                let nps = NonPersistedStorage {
                    theme_name: next_state,
                    run_once: true,
                };
                ui.ctx().data_mut(|d| d.insert_temp(non_persisted_id, nps));

                // --------------------------------------------------------------------------
                // Update non-persistent storage only when theme changed (a little expensive)
                // --------------------------------------------------------------------------
                let persisted_id = Id::new(ID_THEME_NAME);
                ui.ctx().data_mut(|d| {
                    d.insert_persisted(
                        persisted_id,
                        PersistedStorage {
                            theme_name: next_state,
                        },
                    )
                });
            }
        })
        .response
    }
}
