//----------------------------------------------------------------------------
// IMPORTANT: Catppuccin Style Guide
//     https://github.com/catppuccin/catppuccin/blob/main/docs/style-guide.md
//
// See also:
//     https://www.nordtheme.com/docs/colors-and-palettes
//----------------------------------------------------------------------------
//
// Here is the official Catppuccin palette (as of late 2025, unchanged since 2022) with exact hex values and official names.
//
// There are four flavors (light → dark), but the color names and roles stay 100% identical across all flavors — only the hex values change.
//
// | Name        | Purpose / semantic role                          | Latte (Light)   | Frappé          | Macchiato       | Mocha (Dark)    |
// |-------------|--------------------------------------------------|-----------------|-----------------|-----------------|-----------------|
// | rosewater   | Accent, highlights, rarely used primary          | #dc8a78         | #f2d5cf         | #f4dbd6         | #f5e0dc         |
// | flamingo    | Secondary accent, slightly calmer                | #dd7878         | #eebebe         | #f0c6c6         | #f2cdcd         |
// | pink        | Playful accent, syntax magents, etc.             | #ea76cb         | #f4b8e4         | #f5bde6         | #f5c2e7         |
// | mauve       | Main accent / primary color in most ports        | #8839ef         | #ca9ee6         | #c6a0f6         | #cba6f7         |
// | red         | Errors, deletions, dangerous actions             | #d20f39         | #e78284         | #ed8796         | #f38ba8         |
// | maroon      | Softer red, git changed, etc.                    | #e64553         | #ea999c         | #ee99a0         | #eba0ac         |
// | peach       | Warnings, git added, orange-ish                  | #fe640b         | #ef9f76         | #f5a97f         | #fab387         |
// | yellow      | Search highlights, warnings                      | #df8e1d         | #e5c890         | #eed49f         | #f9e2af         |
// | green       | Success, git added, strings                      | #40a02b         | #a6d189         | #a6da95         | #a6e3a1         |
// | teal        | Calmer green/cyan, types, constants              | #179299         | #81c8be         | #8bd5ca         | #94e2d5         |
// | sky         | Brighter cyan, function calls                    | #04a5e5         | #99d1db         | #91d7e3         | #89dceb         |
// | sapphire    | Deep blue-cyan, keywords                         | #209fb5         | #85c1dc         | #7dc4e4         | #74c7ec         |
// | blue        | Links, primary actions, variables                | #1e66f5         | #8caaee         | #8aadf4         | #89b4fa         |
// | lavender    | Subtle accent, UI highlights                     | #7287fd         | #babbf1         | #b7bdf8         | #b4befe         |
// | text        | Main foreground / body text                      | #4c4f69         | #c6d0f5         | #cad3f5         | #cdd6f4         |
// | subtext1    | Slightly dimmer text                             | #5c5f77         | #b5bfe2         | #b8c0e0         | #bac2de         |
// | subtext0    | Dimmer still                                     | #6c6f85         | #a5adce         | #a5adcb         | #a6adc8         |
// | overlay2    | Borders, dim UI elements                         | #7c7f93         | #949cbb         | #939ab7         | #9399b2         |
// | overlay1    | Even dimmer                                      | #8c8fa1         | #838ba7         | #8087a2         | #7f849c         |
// | overlay0    | Very dim                                         | #9ca0b0         | #737994         | #6e738d         | #6c7086         |
// | surface2    | Active tab borders, highlights                   | #acb0be         | #626880         | #5b6078         | #585b70         |
// | surface1    | Inactive tabs, secondary panels                  | #bcc0cc         | #51576d         | #494d64         | #45475a         |
// | surface0    | Floating panels, menus                           | #ccd0da         | #414559         | #363a4f         | #313244         |
// | base        | Main background                                  | #eff1f5         | #303446         | #24273a         | #1e1e2e         |
// | mantle      | Slightly darker than base (statusline, etc.)     | #e6e9ef         | #292c3c         | #1e2030         | #181825         |
// | crust       | Darkest background (rarely used)                 | #dce0e8         | #232634         | #181926         | #11111b         |
