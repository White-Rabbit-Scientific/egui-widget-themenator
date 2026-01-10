use egui::{
    epaint::AlphaFromCoverage,
    style::{HandleShape, NumericColorSpace, TextCursorStyle, WidgetVisuals, Widgets},
    Color32, CornerRadius, Id, Stroke, Style, Widget,
};
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants
// ============================================================================

// IDs for storing app state within egui's Context.
const PERSISTED_THEME_ID: &str = "catppuccin_persisted_theme";
const SESSION_INIT_ID: &str = "catppuccin_initialized";

// Default values if no theme data is supplied. e.g. when using, for example:
//   ui.add(Themenator::new().default_themes_two());
//   ui.add(Themenator::new().default_themes_four());
const LATTE: ThemeConfig = ThemeConfig {
    variant: ThemeVariant::Latte,
    title: "",
    description: "",
    icon: "\u{2600}",
};

const FRAPPE: ThemeConfig = ThemeConfig {
    variant: ThemeVariant::Frappe,
    title: "",
    description: "",
    icon: "\u{1F319}",
};

const MACCHIATO: ThemeConfig = ThemeConfig {
    variant: ThemeVariant::Macchiato,
    title: "",
    description: "",
    icon: "\u{1F319}",
};

const MOCHA: ThemeConfig = ThemeConfig {
    variant: ThemeVariant::Mocha,
    title: "",
    description: "",
    icon: "\u{1F319}",
};

// ============================================================================
// Theme / palette configuration, application and accessor
// ============================================================================

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeVariant {
    #[default]
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

#[rustfmt::skip]
impl ThemeVariant {
    pub fn is_dark(&self) -> bool {
        *self != ThemeVariant::Latte
    }

    // Retrieve the color palette for the active theme
    #[rustfmt::skip]
    fn palette(&self) -> CatppuccinPalette {
        match self {
            Self::Latte     => Color32::latte(),
            Self::Frappe    => Color32::frappe(),
            Self::Macchiato => Color32::macchiato(),
            Self::Mocha     => Color32::mocha(),
        }
    }

    // Retrieve the color palette for the active theme.
    // This application calls this function to get the palette for the active theme.
    pub fn get_current_palette(ctx: &egui::Context) -> CatppuccinPalette {
        let variant = ctx.data_mut(|d| d.get_persisted::<Self>(Id::new(PERSISTED_THEME_ID)))
            .unwrap_or_default();
        variant.palette()
    }

    // Apply the theme to the egui Context
    pub fn apply(&self, ctx: &egui::Context) {
        let palette = self.palette();

        let mode = if self.is_dark() {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        };
        // Set theme light/dark
        ctx.set_theme(mode);

        // Overwrite specific styles with Catppuccin colors
        ctx.style_mut_of(mode, |style| {
            apply_catppuccin_style(style, self.is_dark(), palette);
        });
    }
}

// ============================================================================
// Widget builder, configuration and implementation
// ============================================================================

// Metadata for how a theme looks in the UI (Icon, Description)
#[derive(Clone, Debug)]
pub struct ThemeConfig {
    pub variant: ThemeVariant,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

// Theme metadata is stored in themes vector
#[derive(Clone)]
pub struct Themenator {
    themes: Vec<ThemeConfig>,
}

impl Themenator {
    pub fn new() -> Self {
        Self { themes: Vec::new() }
    }

    // Add a theme configuration to the themes vector
    pub fn add(mut self, tc: ThemeConfig) -> Self {
        self.themes.push(tc);
        self
    }

    // Add light and dark default themes to the themes vector
    pub fn default_themes_two(mut self) -> Self {
        let latte = LATTE;
        let mocha = MOCHA;
        let default_themes = vec![latte, mocha];
        self.themes = default_themes;
        self
    }

    // Add light and dark default themes to the themes vector
    #[rustfmt::skip]
    pub fn default_themes_four(mut self) -> Self {
        let latte     = LATTE;
        let frappe    = FRAPPE;
        let macchiato = MACCHIATO;
        let mocha     = MOCHA;
        let default_themes = vec![latte, frappe, macchiato, mocha];
        self.themes = default_themes;
        self
    }

    // Get the indices of the current and next theme in the themes vector
    fn get_themes_vec_indices(&self, variant: ThemeVariant) -> (usize, usize) {
        let divisor = self.themes.len();
        if divisor == 0 {
            // We should NEVER get here!
            panic!("Vector length = 0. Division by zero is not allowed here!");
        }

        let current_idx = self
            .themes
            .iter()
            .position(|t| t.variant == variant)
            .unwrap_or(0);

        // New index calculation
        let next_idx = (current_idx + 1) % divisor;
        (current_idx, next_idx)
    }
}

impl Widget for Themenator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // If themes vector is empty, display label and return a label response
        if self.themes.is_empty() {
            return ui.label("No themes configured");
        }

        // Read the stored theme and the "initialized" flag.
        let (mut active_variant, is_initialized) = ui.ctx().data_mut(|d| {
            let variant = d
                .get_persisted::<ThemeVariant>(Id::new(PERSISTED_THEME_ID))
                .unwrap_or_default();
            let init = d
                .get_temp::<bool>(Id::new(SESSION_INIT_ID))
                .unwrap_or(false);
            (variant, init)
        });

        // Ensure the active variant is actually in our list of themes.
        if !self.themes.iter().any(|t| t.variant == active_variant) {
            // Not found, so default to the first theme in supplied vector
            active_variant = self.themes[0].variant;
        }

        // If this is the first time the widget is seen this session, apply the theme.
        if !is_initialized {
            active_variant.apply(ui.ctx());
            ui.ctx()
                .data_mut(|d| d.insert_temp(Id::new(SESSION_INIT_ID), true));
        }

        // Render UI
        let (cur_idx, next_idx) = self.get_themes_vec_indices(active_variant);
        let theme_config = &self.themes[cur_idx];

        let resp = ui.horizontal(|ui| {
            let button = egui::Button::new(
                egui::RichText::new(theme_config.icon)
                    .color(ui.visuals().warn_fg_color)
                    .size(18.0),
            )
            .frame(false);

            let btn_resp = ui.add(button);
            ui.label(format!(
                "{} {}",
                theme_config.title, theme_config.description
            ));
            btn_resp
        });

        // Update
        if resp.inner.clicked() {
            let next_variant = self.themes[next_idx].variant;
            next_variant.apply(ui.ctx());

            // Save the new state to persisted storage.
            ui.ctx().data_mut(|d| {
                d.insert_persisted(Id::new(PERSISTED_THEME_ID), next_variant);
            });
        }
        resp.response
    }
}

// ============================================================================
// Style application and logic
// ============================================================================

#[rustfmt::skip]
fn apply_catppuccin_style(style: &mut Style, dark_mode: bool, p: CatppuccinPalette) {
    const CORNER_RADIUS: u8 = 0;

    let widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill:       p.overlay0,
            weak_bg_fill:  p.overlay0,
            bg_stroke:     Stroke::new(1.0, p.overlay0),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        inactive: WidgetVisuals {
            bg_fill:       p.mantle,
            weak_bg_fill:  p.mantle,
            bg_stroke:     Stroke::new(1.0, p.overlay1),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        hovered: WidgetVisuals {
            bg_fill:       p.mantle,
            weak_bg_fill:  p.mantle,
            bg_stroke:     Stroke::new(1.0, p.overlay2),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        active: WidgetVisuals {
            bg_fill:       p.base,
            weak_bg_fill:  p.base,
            bg_stroke:     Stroke::new(1.0, p.overlay2),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
        open: WidgetVisuals {
            bg_fill:       p.mantle,
            weak_bg_fill:  p.mantle,
            bg_stroke:     Stroke::new(1.0, p.text),
            corner_radius: CornerRadius::same(CORNER_RADIUS),
            fg_stroke:     Stroke::new(1.0, p.text),
            expansion:     0.0,
        },
    };

    style.visuals.dark_mode                = dark_mode;
    style.visuals.text_alpha_from_coverage = if dark_mode { AlphaFromCoverage::TwoCoverageMinusCoverageSq } else { AlphaFromCoverage::Linear };
    style.visuals.weak_text_alpha          = 0.4;
    style.visuals.weak_text_color          = Some(p.surface2);
    style.visuals.widgets                  = widgets;
    style.visuals.hyperlink_color          = p.blue;
    style.visuals.faint_bg_color           = p.surface0;
    style.visuals.extreme_bg_color         = p.crust;
    style.visuals.text_edit_bg_color       = Some(p.mantle);
    style.visuals.code_bg_color            = p.mantle;
    style.visuals.warn_fg_color            = p.yellow;
    style.visuals.error_fg_color           = p.red;
    style.visuals.window_fill              = p.base;
    style.visuals.panel_fill               = p.base;
    style.visuals.text_cursor              = TextCursorStyle { stroke: Stroke::new(2.0, p.rosewater), ..Default::default() };
    style.visuals.button_frame             = true;
    style.visuals.handle_shape             = HandleShape::Circle;
    style.visuals.numeric_color_space      = NumericColorSpace::GammaByte;
}

// ============================================================================
// Color Palette Data
// ============================================================================

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
pub trait CatppuccinColors {
    fn latte()     -> CatppuccinPalette;
    fn frappe()    -> CatppuccinPalette;
    fn macchiato() -> CatppuccinPalette;
    fn mocha()     -> CatppuccinPalette;
}

#[rustfmt::skip]
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
