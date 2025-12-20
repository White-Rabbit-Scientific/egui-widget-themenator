# Themenator — Themes and Theme Switcher Widget for egui

**Themenator** provides [Catppuccin](https://catppuccin.com/) color palettes for ready-made themes, packaged as a widget for [egui](https://github.com/emilk/egui).

It allows egui applications to switch between Latte, Frappé, Macchiato, and Mocha at runtime, with the selected theme automatically persisted across app restarts.

## Features
* Full Catppuccin palettes (Latte / Frappé / Macchiato / Mocha)
* Complete egui Style and Visuals integration
* Correct light / dark base theme handling
* Runtime theme switching
* Persistent theme storage across app restarts
* Theme selector buttons
* Builder pattern for clean, fluent configuration
* Runs on Linux, Mac, Windows, WebAssembly (wasm) and Raspberry Pi

## Quick Start
Apply a theme at startup or at any time:
```toml
use egui_widget_themenator::ThemeName;
ThemeName::Mocha.set(ctx);
```
The theme automatically:
* Applies all Catppuccin colors
* Switches egui between light and dark mode
* Stores the active theme in egui context memory

## Screenshots

![Texicon screenshot 1](https://raw.githubusercontent.com/White-Rabbit-Scientific/egui-widget-texicon/main/images/Screenshot1.png)
![Texicon screenshot 2](https://raw.githubusercontent.com/White-Rabbit-Scientific/egui-widget-texicon/main/images/Screenshot2.png)

## Theme Selector Widget
* Add a simple runtime theme switcher to your UI:
```rust
use egui_widget_themenator::{ThemeName, ThemeWidget};
ThemeWidget::new().show(ctx);
```
With a label:
```rust
ui.add(
    ThemeWidget::new()
        .label("Theme")
);
```
Hide labels for compact layouts:
```rust
ui.add(
    ThemeWidget::new()
        .show_labels(false)
);
```
The widget:
* Highlights the active theme
* Updates egui styling immediately
* Persists the selected theme automatically

## Using the Palette Directly
Each Catppuccin palette is exposed as strongly-typed Color32 values and can be used directly in custom widgets:

```rust
use egui::Color32;
use egui_themenator::CatppuccinColors;

let palette = Color32::mocha();
let accent = palette.mauve;
```
## How It Works
* Catppuccin palettes are defined as Color32 values
* Themes map palette colors to egui Style and Visuals
* Non-persistent storage is used while running for lower CPU overhead
* Persistent storage saves the selected theme across restarts
* The widget synchronizes style and storage automatically
* No manual state management is required.

## Platform Support
* Linux
* macOS
* Windows
* WebAssembly (wasm)
* Raspberry Pi

## License

Licensed under either of:
* Apache License, Version 2.0 (LICENSE-APACHE)
* MIT License (LICENSE-MIT)

at your option.

## Author
Made with ❤️ by Rob @ White Rabbit Scientific

Inspired by modern design tools and clean UI patterns.

Star this repo if you like it! ⭐️
