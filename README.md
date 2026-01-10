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

## Live wasm demo app

The wasm [demo app](https://dreamy-meringue-f98d25.netlify.app/) runs in your web browser.

## Screenshots
![Texicon screenshot 1](https://raw.githubusercontent.com/White-Rabbit-Scientific/egui-widget-texicon/main/images/Screenshot1.png)
![Texicon screenshot 2](https://raw.githubusercontent.com/White-Rabbit-Scientific/egui-widget-texicon/main/images/Screenshot2.png)
![Texicon screenshot 3](https://raw.githubusercontent.com/White-Rabbit-Scientific/egui-widget-texicon/main/images/Screenshot3.png)

## Quick Start

Add ```egui-widget-themenator``` as a dependency in your Cargo.toml file:
```toml
[dependencies]
egui_widget_themenator = 0.2
```

Add the theme widget:
```rust
ui.add(ThemeWidget::new().label("Theme selector:"));
);
```

The theme automatically:
* Adds an icon indicating light or dark mode
* Icon is clickable and cycles through themes
Applies all Catppuccin color styling
* Switches between light and dark mode
* Persists the selected theme across restarts

## Platform Support
* Linux
* macOS
* Windows
* WebAssembly (wasm)
* Raspberry Pi 5 with Touchscreen

## License
Licensed under either of:
* Apache License, Version 2.0 (LICENSE-APACHE)
* MIT License (LICENSE-MIT)

at your option.

## Author
Made with ❤️ by Rob @ White Rabbit Scientific.

Inspired by modern design tools and clean UI patterns.

Star this repo if you like it! ⭐️
