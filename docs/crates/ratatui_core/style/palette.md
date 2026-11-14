**ratatui_core > style > palette**

# Module: style::palette

## Contents

**Modules**

- [`material`](#material) - Material design color palettes.
- [`tailwind`](#tailwind) - Represents the Tailwind CSS [default color palette][palette].

---

## Module: material

Material design color palettes.

Represents the colors from the 2014 [Material design color palettes][palettes] by Google.

[palettes]: https://m2.material.io/design/color/the-color-system.html#tools-for-picking-colors

There are 16 palettes with accent colors, and 3 palettes without accent colors. Each palette
has 10 colors, with variants from 50 to 900. The accent palettes also have 4 accent colors
with variants from 100 to 700. Black and White are also included for completeness and to avoid
being affected by any terminal theme that might be in use.

This module exists to provide a convenient way to use the colors from the
[`matdesign-color` crate] in your application.

<style>
.color { display: flex; align-items: center; }
.color > div { width: 2rem; height: 2rem; }
.color > div.name { width: 150px; !important; }
</style>
<div style="overflow-x: auto">
<div style="display: flex; flex-direction:column; text-align: left">
<div class="color" style="font-size:0.8em">
    <div class="name"></div>
    <div>C50</div>
    <div>C100</div>
    <div>C200</div>
    <div>C300</div>
    <div>C400</div>
    <div>C500</div>
    <div>C600</div>
    <div>C700</div>
    <div>C800</div>
    <div>C900</div>
    <div>A100</div>
    <div>A200</div>
    <div>A400</div>
    <div>A700</div>
</div>
<div class="color">
    <div class="name">

[`RED`]</div>
    <div style="background-color: #FFEBEE"></div>
    <div style="background-color: #FFCDD2"></div>
    <div style="background-color: #EF9A9A"></div>
    <div style="background-color: #E57373"></div>
    <div style="background-color: #EF5350"></div>
    <div style="background-color: #F44336"></div>
    <div style="background-color: #E53935"></div>
    <div style="background-color: #D32F2F"></div>
    <div style="background-color: #C62828"></div>
    <div style="background-color: #B71C1C"></div>
    <div style="background-color: #FF8A80"></div>
    <div style="background-color: #FF5252"></div>
    <div style="background-color: #FF1744"></div>
    <div style="background-color: #D50000"></div>
</div>
<div class="color">
    <div class="name">

[`PINK`]</div>
    <div style="background-color: #FCE4EC"></div>
    <div style="background-color: #F8BBD0"></div>
    <div style="background-color: #F48FB1"></div>
    <div style="background-color: #F06292"></div>
    <div style="background-color: #EC407A"></div>
    <div style="background-color: #E91E63"></div>
    <div style="background-color: #D81B60"></div>
    <div style="background-color: #C2185B"></div>
    <div style="background-color: #AD1457"></div>
    <div style="background-color: #880E4F"></div>
    <div style="background-color: #FF80AB"></div>
    <div style="background-color: #FF4081"></div>
    <div style="background-color: #F50057"></div>
    <div style="background-color: #C51162"></div>
</div>
<div class="color">
    <div class="name">

[`PURPLE`]</div>
    <div style="background-color: #F3E5F5"></div>
    <div style="background-color: #E1BEE7"></div>
    <div style="background-color: #CE93D8"></div>
    <div style="background-color: #BA68C8"></div>
    <div style="background-color: #AB47BC"></div>
    <div style="background-color: #9C27B0"></div>
    <div style="background-color: #8E24AA"></div>
    <div style="background-color: #7B1FA2"></div>
    <div style="background-color: #6A1B9A"></div>
    <div style="background-color: #4A148C"></div>
    <div style="background-color: #EA80FC"></div>
    <div style="background-color: #E040FB"></div>
    <div style="background-color: #D500F9"></div>
    <div style="background-color: #AA00FF"></div>
</div>
<div class="color">
    <div class="name">

[`DEEP_PURPLE`]</div>
    <div style="background-color: #EDE7F6"></div>
    <div style="background-color: #D1C4E9"></div>
    <div style="background-color: #B39DDB"></div>
    <div style="background-color: #9575CD"></div>
    <div style="background-color: #7E57C2"></div>
    <div style="background-color: #673AB7"></div>
    <div style="background-color: #5E35B1"></div>
    <div style="background-color: #512DA8"></div>
    <div style="background-color: #4527A0"></div>
    <div style="background-color: #311B92"></div>
    <div style="background-color: #B388FF"></div>
    <div style="background-color: #7C4DFF"></div>
    <div style="background-color: #651FFF"></div>
    <div style="background-color: #6200EA"></div>
</div>
<div class="color">
    <div class="name">

[`INDIGO`]</div>
    <div style="background-color: #E8EAF6"></div>
    <div style="background-color: #C5CAE9"></div>
    <div style="background-color: #9FA8DA"></div>
    <div style="background-color: #7986CB"></div>
    <div style="background-color: #5C6BC0"></div>
    <div style="background-color: #3F51B5"></div>
    <div style="background-color: #3949AB"></div>
    <div style="background-color: #303F9F"></div>
    <div style="background-color: #283593"></div>
    <div style="background-color: #1A237E"></div>
    <div style="background-color: #8C9EFF"></div>
    <div style="background-color: #536DFE"></div>
    <div style="background-color: #3D5AFE"></div>
    <div style="background-color: #304FFE"></div>
</div>
<div class="color">
    <div class="name">

[`BLUE`]</div>
    <div style="background-color: #E3F2FD"></div>
    <div style="background-color: #BBDEFB"></div>
    <div style="background-color: #90CAF9"></div>
    <div style="background-color: #64B5F6"></div>
    <div style="background-color: #42A5F5"></div>
    <div style="background-color: #2196F3"></div>
    <div style="background-color: #1E88E5"></div>
    <div style="background-color: #1976D2"></div>
    <div style="background-color: #1565C0"></div>
    <div style="background-color: #0D47A1"></div>
    <div style="background-color: #82B1FF"></div>
    <div style="background-color: #448AFF"></div>
    <div style="background-color: #2979FF"></div>
    <div style="background-color: #2962FF"></div>
</div>
<div class="color">
    <div class="name">

[`LIGHT_BLUE`]</div>
    <div style="background-color: #E1F5FE"></div>
    <div style="background-color: #B3E5FC"></div>
    <div style="background-color: #81D4FA"></div>
    <div style="background-color: #4FC3F7"></div>
    <div style="background-color: #29B6F6"></div>
    <div style="background-color: #03A9F4"></div>
    <div style="background-color: #039BE5"></div>
    <div style="background-color: #0288D1"></div>
    <div style="background-color: #0277BD"></div>
    <div style="background-color: #01579B"></div>
    <div style="background-color: #80D8FF"></div>
    <div style="background-color: #40C4FF"></div>
    <div style="background-color: #00B0FF"></div>
    <div style="background-color: #0091EA"></div>
</div>
<div class="color">
    <div class="name">

[`CYAN`]</div>
    <div style="background-color: #E0F7FA"></div>
    <div style="background-color: #B2EBF2"></div>
    <div style="background-color: #80DEEA"></div>
    <div style="background-color: #4DD0E1"></div>
    <div style="background-color: #26C6DA"></div>
    <div style="background-color: #00BCD4"></div>
    <div style="background-color: #00ACC1"></div>
    <div style="background-color: #0097A7"></div>
    <div style="background-color: #00838F"></div>
    <div style="background-color: #006064"></div>
    <div style="background-color: #84FFFF"></div>
    <div style="background-color: #18FFFF"></div>
    <div style="background-color: #00E5FF"></div>
    <div style="background-color: #00B8D4"></div>
</div>
<div class="color">
    <div class="name">

[`TEAL`]</div>
    <div style="background-color: #E0F2F1"></div>
    <div style="background-color: #B2DFDB"></div>
    <div style="background-color: #80CBC4"></div>
    <div style="background-color: #4DB6AC"></div>
    <div style="background-color: #26A69A"></div>
    <div style="background-color: #009688"></div>
    <div style="background-color: #00897B"></div>
    <div style="background-color: #00796B"></div>
    <div style="background-color: #00695C"></div>
    <div style="background-color: #004D40"></div>
    <div style="background-color: #A7FFEB"></div>
    <div style="background-color: #64FFDA"></div>
    <div style="background-color: #1DE9B6"></div>
    <div style="background-color: #00BFA5"></div>
</div>
<div class="color">
    <div class="name">

[`GREEN`]</div>
    <div style="background-color: #E8F5E9"></div>
    <div style="background-color: #C8E6C9"></div>
    <div style="background-color: #A5D6A7"></div>
    <div style="background-color: #81C784"></div>
    <div style="background-color: #66BB6A"></div>
    <div style="background-color: #4CAF50"></div>
    <div style="background-color: #43A047"></div>
    <div style="background-color: #388E3C"></div>
    <div style="background-color: #2E7D32"></div>
    <div style="background-color: #1B5E20"></div>
    <div style="background-color: #B9F6CA"></div>
    <div style="background-color: #69F0AE"></div>
    <div style="background-color: #00E676"></div>
    <div style="background-color: #00C853"></div>
</div>
<div class="color">
    <div class="name">

[`LIGHT_GREEN`]</div>
    <div style="background-color: #F1F8E9"></div>
    <div style="background-color: #DCEDC8"></div>
    <div style="background-color: #C5E1A5"></div>
    <div style="background-color: #AED581"></div>
    <div style="background-color: #9CCC65"></div>
    <div style="background-color: #8BC34A"></div>
    <div style="background-color: #7CB342"></div>
    <div style="background-color: #689F38"></div>
    <div style="background-color: #558B2F"></div>
    <div style="background-color: #33691E"></div>
    <div style="background-color: #CCFF90"></div>
    <div style="background-color: #B2FF59"></div>
    <div style="background-color: #76FF03"></div>
    <div style="background-color: #64DD17"></div>
</div>
<div class="color">
    <div class="name">

[`LIME`]</div>
    <div style="background-color: #F9FBE7"></div>
    <div style="background-color: #F0F4C3"></div>
    <div style="background-color: #E6EE9C"></div>
    <div style="background-color: #DCE775"></div>
    <div style="background-color: #D4E157"></div>
    <div style="background-color: #CDDC39"></div>
    <div style="background-color: #C0CA33"></div>
    <div style="background-color: #AFB42B"></div>
    <div style="background-color: #9E9D24"></div>
    <div style="background-color: #827717"></div>
    <div style="background-color: #F4FF81"></div>
    <div style="background-color: #EEFF41"></div>
    <div style="background-color: #C6FF00"></div>
    <div style="background-color: #AEEA00"></div>
</div>
<div class="color">
    <div class="name">

[`YELLOW`]</div>
    <div style="background-color: #FFFDE7"></div>
    <div style="background-color: #FFF9C4"></div>
    <div style="background-color: #FFF59D"></div>
    <div style="background-color: #FFF176"></div>
    <div style="background-color: #FFEE58"></div>
    <div style="background-color: #FFEB3B"></div>
    <div style="background-color: #FDD835"></div>
    <div style="background-color: #FBC02D"></div>
    <div style="background-color: #F9A825"></div>
    <div style="background-color: #F57F17"></div>
    <div style="background-color: #FFFF8D"></div>
    <div style="background-color: #FFFF00"></div>
    <div style="background-color: #FFEA00"></div>
    <div style="background-color: #FFD600"></div>
</div>
<div class="color">
    <div class="name">

[`AMBER`]</div>
    <div style="background-color: #FFF8E1"></div>
    <div style="background-color: #FFECB3"></div>
    <div style="background-color: #FFE082"></div>
    <div style="background-color: #FFD54F"></div>
    <div style="background-color: #FFCA28"></div>
    <div style="background-color: #FFC107"></div>
    <div style="background-color: #FFB300"></div>
    <div style="background-color: #FFA000"></div>
    <div style="background-color: #FF8F00"></div>
    <div style="background-color: #FF6F00"></div>
    <div style="background-color: #FFE57F"></div>
    <div style="background-color: #FFD740"></div>
    <div style="background-color: #FFC400"></div>
    <div style="background-color: #FFAB00"></div>
</div>
<div class="color">
    <div class="name">

[`ORANGE`]</div>
    <div style="background-color: #FFF3E0"></div>
    <div style="background-color: #FFE0B2"></div>
    <div style="background-color: #FFCC80"></div>
    <div style="background-color: #FFB74D"></div>
    <div style="background-color: #FFA726"></div>
    <div style="background-color: #FF9800"></div>
    <div style="background-color: #FB8C00"></div>
    <div style="background-color: #F57C00"></div>
    <div style="background-color: #EF6C00"></div>
    <div style="background-color: #E65100"></div>
    <div style="background-color: #FFD180"></div>
    <div style="background-color: #FFAB40"></div>
    <div style="background-color: #FF9100"></div>
    <div style="background-color: #FF6D00"></div>
</div>
<div class="color">
    <div class="name">

[`DEEP_ORANGE`]</div>
    <div style="background-color: #FBE9E7"></div>
    <div style="background-color: #FFCCBC"></div>
    <div style="background-color: #FFAB91"></div>
    <div style="background-color: #FF8A65"></div>
    <div style="background-color: #FF7043"></div>
    <div style="background-color: #FF5722"></div>
    <div style="background-color: #F4511E"></div>
    <div style="background-color: #E64A19"></div>
    <div style="background-color: #D84315"></div>
    <div style="background-color: #BF360C"></div>
    <div style="background-color: #FF9E80"></div>
    <div style="background-color: #FF6E40"></div>
    <div style="background-color: #FF3D00"></div>
    <div style="background-color: #DD2C00"></div>
</div>
<div class="color">
    <div class="name">

[`BROWN`]</div>
    <div style="background-color: #EFEBE9"></div>
    <div style="background-color: #D7CCC8"></div>
    <div style="background-color: #BCAAA4"></div>
    <div style="background-color: #A1887F"></div>
    <div style="background-color: #8D6E63"></div>
    <div style="background-color: #795548"></div>
    <div style="background-color: #6D4C41"></div>
    <div style="background-color: #5D4037"></div>
    <div style="background-color: #4E342E"></div>
    <div style="background-color: #3E2723"></div>
</div>
<div class="color">
    <div class="name">

[`GRAY`]</div>
    <div style="background-color: #FAFAFA"></div>
    <div style="background-color: #F5F5F5"></div>
    <div style="background-color: #EEEEEE"></div>
    <div style="background-color: #E0E0E0"></div>
    <div style="background-color: #BDBDBD"></div>
    <div style="background-color: #9E9E9E"></div>
    <div style="background-color: #757575"></div>
    <div style="background-color: #616161"></div>
    <div style="background-color: #424242"></div>
    <div style="background-color: #212121"></div>
</div>
<div class="color">
    <div class="name">

[`BLUE_GRAY`]</div>
    <div style="background-color: #ECEFF1"></div>
    <div style="background-color: #CFD8DC"></div>
    <div style="background-color: #B0BEC5"></div>
    <div style="background-color: #90A4AE"></div>
    <div style="background-color: #78909C"></div>
    <div style="background-color: #607D8B"></div>
    <div style="background-color: #546E7A"></div>
    <div style="background-color: #455A64"></div>
    <div style="background-color: #37474F"></div>
    <div style="background-color: #263238"></div>
</div>
<div class="color">
    <div class="name">

[`BLACK`]</div>
    <div class="bw" style="width: 350px; background-color: #000000"></div>
</div>
<div class="color">
    <div class="name">

[`WHITE`]</div>
    <div style="width: 350px; background-color: #FFFFFF"></div>
</div>
</div>
</div>

# Example

```rust
use ratatui_core::style::Color;
use ratatui_core::style::palette::material::{BLUE, RED};

assert_eq!(RED.c500, Color::Rgb(244, 67, 54));
assert_eq!(BLUE.c500, Color::Rgb(33, 150, 243));
```

[`matdesign-color` crate]: https://crates.io/crates/matdesign-color



## Module: tailwind

Represents the Tailwind CSS [default color palette][palette].

[palette]: https://tailwindcss.com/docs/customizing-colors#default-color-palette

There are 22 palettes. Each palette has 11 colors, with variants from 50 to 950. Black and White
are also included for completeness and to avoid being affected by any terminal theme that might
be in use.

<style>
.color { display: flex; align-items: center; }
.color > div { width: 2rem; height: 2rem; }
.color > div.name { width: 150px; !important; }
</style>
<div style="overflow-x: auto">
<div style="display: flex; flex-direction:column; text-align: left">
<div class="color" style="font-size:0.8em">
    <div class="name"></div>
    <div>C50</div> <div>C100</div> <div>C200</div> <div>C300</div> <div>C400</div>
    <div>C500</div> <div>C600</div> <div>C700</div> <div>C800</div> <div>C900</div>
    <div>C950</div>
</div>
<div class="color">
    <div class="name">

[`SLATE`]</div>
    <div style="background-color: #f8fafc"></div> <div style="background-color: #f1f5f9"></div>
    <div style="background-color: #e2e8f0"></div> <div style="background-color: #cbd5e1"></div>
    <div style="background-color: #94a3b8"></div> <div style="background-color: #64748b"></div>
    <div style="background-color: #475569"></div> <div style="background-color: #334155"></div>
    <div style="background-color: #1e293b"></div> <div style="background-color: #0f172a"></div>
    <div style="background-color: #020617"></div>
</div>
<div class="color">
    <div class="name">

[`GRAY`]</div>
    <div style="background-color: #f9fafb"></div> <div style="background-color: #f3f4f6"></div>
    <div style="background-color: #e5e7eb"></div> <div style="background-color: #d1d5db"></div>
    <div style="background-color: #9ca3af"></div> <div style="background-color: #6b7280"></div>
    <div style="background-color: #4b5563"></div> <div style="background-color: #374151"></div>
    <div style="background-color: #1f2937"></div> <div style="background-color: #111827"></div>
    <div style="background-color: #0a0a0a"></div>
</div>
<div class="color">
    <div class="name">

[`ZINC`]</div>
    <div style="background-color: #fafafa"></div> <div style="background-color: #f5f5f5"></div>
    <div style="background-color: #e5e5e5"></div> <div style="background-color: #d4d4d4"></div>
    <div style="background-color: #a1a1aa"></div> <div style="background-color: #71717a"></div>
    <div style="background-color: #52525b"></div> <div style="background-color: #404040"></div>
    <div style="background-color: #262626"></div> <div style="background-color: #171717"></div>
    <div style="background-color: #0a0a0a"></div>
</div>
<div class="color">
    <div class="name">

[`NEUTRAL`]</div>
    <div style="background-color: #fafafa"></div> <div style="background-color: #f5f5f5"></div>
    <div style="background-color: #e5e5e5"></div> <div style="background-color: #d4d4d4"></div>
    <div style="background-color: #a3a3a3"></div> <div style="background-color: #737373"></div>
    <div style="background-color: #525252"></div> <div style="background-color: #404040"></div>
    <div style="background-color: #262626"></div> <div style="background-color: #171717"></div>
    <div style="background-color: #0a0a0a"></div>
</div>
<div class="color">
    <div class="name">

[`STONE`]</div>
    <div style="background-color: #fafaf9"></div> <div style="background-color: #f5f5f4"></div>
    <div style="background-color: #e7e5e4"></div> <div style="background-color: #d6d3d1"></div>
    <div style="background-color: #a8a29e"></div> <div style="background-color: #78716c"></div>
    <div style="background-color: #57534e"></div> <div style="background-color: #44403c"></div>
    <div style="background-color: #292524"></div> <div style="background-color: #1c1917"></div>
    <div style="background-color: #0c0a09"></div>
</div>
<div class="color">
    <div class="name">

[`RED`]</div>
    <div style="background-color: #fef2f2"></div> <div style="background-color: #fee2e2"></div>
    <div style="background-color: #fecaca"></div> <div style="background-color: #fca5a5"></div>
    <div style="background-color: #f87171"></div> <div style="background-color: #ef4444"></div>
    <div style="background-color: #dc2626"></div> <div style="background-color: #b91c1c"></div>
    <div style="background-color: #991b1b"></div> <div style="background-color: #7f1d1d"></div>
    <div style="background-color: #450a0a"></div>
</div>
<div class="color">
    <div class="name">

[`ORANGE`]</div>
    <div style="background-color: #fff7ed"></div> <div style="background-color: #ffedd5"></div>
    <div style="background-color: #fed7aa"></div> <div style="background-color: #fdba74"></div>
    <div style="background-color: #fb923c"></div> <div style="background-color: #f97316"></div>
    <div style="background-color: #ea580c"></div> <div style="background-color: #c2410c"></div>
    <div style="background-color: #9a3412"></div> <div style="background-color: #7c2d12"></div>
    <div style="background-color: #431407"></div>
</div>
<div class="color">
    <div class="name">

[`AMBER`]</div>
    <div style="background-color: #fffbeb"></div> <div style="background-color: #fef3c7"></div>
    <div style="background-color: #fde68a"></div> <div style="background-color: #fcd34d"></div>
    <div style="background-color: #fbbf24"></div> <div style="background-color: #f59e0b"></div>
    <div style="background-color: #d97706"></div> <div style="background-color: #b45309"></div>
    <div style="background-color: #92400e"></div> <div style="background-color: #78350f"></div>
    <div style="background-color: #451a03"></div>
</div>
<div class="color">
    <div class="name">

[`YELLOW`]</div>
    <div style="background-color: #fefce8"></div> <div style="background-color: #fef9c3"></div>
    <div style="background-color: #fef08a"></div> <div style="background-color: #fde047"></div>
    <div style="background-color: #facc15"></div> <div style="background-color: #eab308"></div>
    <div style="background-color: #ca8a04"></div> <div style="background-color: #a16207"></div>
    <div style="background-color: #854d0e"></div> <div style="background-color: #713f12"></div>
    <div style="background-color: #422006"></div>
</div>
<div class="color">
    <div class="name">

[`LIME`]</div>
    <div style="background-color: #f7fee7"></div> <div style="background-color: #ecfccb"></div>
    <div style="background-color: #d9f99d"></div> <div style="background-color: #bef264"></div>
    <div style="background-color: #a3e635"></div> <div style="background-color: #84cc16"></div>
    <div style="background-color: #65a30d"></div> <div style="background-color: #4d7c0f"></div>
    <div style="background-color: #3f6212"></div> <div style="background-color: #365314"></div>
    <div style="background-color: #1a2e05"></div>
</div>
<div class="color">
    <div class="name">

[`GREEN`]</div>
    <div style="background-color: #f0fdf4"></div> <div style="background-color: #dcfce7"></div>
    <div style="background-color: #bbf7d0"></div> <div style="background-color: #86efac"></div>
    <div style="background-color: #4ade80"></div> <div style="background-color: #22c55e"></div>
    <div style="background-color: #16a34a"></div> <div style="background-color: #15803d"></div>
    <div style="background-color: #166534"></div> <div style="background-color: #14532d"></div>
    <div style="background-color: #052e16"></div>
</div>
<div class="color">
    <div class="name">

[`EMERALD`]</div>
    <div style="background-color: #ecfdf5"></div> <div style="background-color: #d1fae5"></div>
    <div style="background-color: #a7f3d0"></div> <div style="background-color: #6ee7b7"></div>
    <div style="background-color: #34d399"></div> <div style="background-color: #10b981"></div>
    <div style="background-color: #059669"></div> <div style="background-color: #047857"></div>
    <div style="background-color: #065f46"></div> <div style="background-color: #064e3b"></div>
    <div style="background-color: #022c22"></div>
</div>
<div class="color">
    <div class="name">

[`TEAL`]</div>
    <div style="background-color: #f0fdfa"></div> <div style="background-color: #ccfbf1"></div>
    <div style="background-color: #99f6e4"></div> <div style="background-color: #5eead4"></div>
    <div style="background-color: #2dd4bf"></div> <div style="background-color: #14b8a6"></div>
    <div style="background-color: #0d9488"></div> <div style="background-color: #0f766e"></div>
    <div style="background-color: #115e59"></div> <div style="background-color: #134e4a"></div>
    <div style="background-color: #042f2e"></div>
</div>
<div class="color">
    <div class="name">

[`CYAN`]</div>
    <div style="background-color: #ecfeff"></div> <div style="background-color: #cffafe"></div>
    <div style="background-color: #a5f3fc"></div> <div style="background-color: #67e8f9"></div>
    <div style="background-color: #22d3ee"></div> <div style="background-color: #06b6d4"></div>
    <div style="background-color: #0891b2"></div> <div style="background-color: #0e7490"></div>
    <div style="background-color: #155e75"></div> <div style="background-color: #164e63"></div>
    <div style="background-color: #083344"></div>
</div>
<div class="color">
    <div class="name">

[`SKY`]</div>
    <div style="background-color: #f0f9ff"></div> <div style="background-color: #e0f2fe"></div>
    <div style="background-color: #bae6fd"></div> <div style="background-color: #7dd3fc"></div>
    <div style="background-color: #38bdf8"></div> <div style="background-color: #0ea5e9"></div>
    <div style="background-color: #0284c7"></div> <div style="background-color: #0369a1"></div>
    <div style="background-color: #075985"></div> <div style="background-color: #0c4a6e"></div>
    <div style="background-color: #082f49"></div>
</div>
<div class="color">
    <div class="name">

[`BLUE`]</div>
    <div style="background-color: #eff6ff"></div> <div style="background-color: #dbeafe"></div>
    <div style="background-color: #bfdbfe"></div> <div style="background-color: #93c5fd"></div>
    <div style="background-color: #60a5fa"></div> <div style="background-color: #3b82f6"></div>
    <div style="background-color: #2563eb"></div> <div style="background-color: #1d4ed8"></div>
    <div style="background-color: #1e40af"></div> <div style="background-color: #1e3a8a"></div>
    <div style="background-color: #172554"></div>
</div>
<div class="color">
    <div class="name">

[`INDIGO`]</div>
    <div style="background-color: #eef2ff"></div> <div style="background-color: #e0e7ff"></div>
    <div style="background-color: #c7d2fe"></div> <div style="background-color: #a5b4fc"></div>
    <div style="background-color: #818cf8"></div> <div style="background-color: #6366f1"></div>
    <div style="background-color: #4f46e5"></div> <div style="background-color: #4338ca"></div>
    <div style="background-color: #3730a3"></div> <div style="background-color: #312e81"></div>
    <div style="background-color: #1e1b4b"></div>
</div>
<div class="color">
    <div class="name">

[`VIOLET`]</div>
    <div style="background-color: #f5f3ff"></div> <div style="background-color: #ede9fe"></div>
    <div style="background-color: #ddd6fe"></div> <div style="background-color: #c4b5fd"></div>
    <div style="background-color: #a78bfa"></div> <div style="background-color: #8b5cf6"></div>
    <div style="background-color: #7c3aed"></div> <div style="background-color: #6d28d9"></div>
    <div style="background-color: #5b21b6"></div> <div style="background-color: #4c1d95"></div>
    <div style="background-color: #2e1065"></div>
</div>
<div class="color">
    <div class="name">

[`PURPLE`]</div>
    <div style="background-color: #faf5ff"></div> <div style="background-color: #f3e8ff"></div>
    <div style="background-color: #e9d5ff"></div> <div style="background-color: #d8b4fe"></div>
    <div style="background-color: #c084fc"></div> <div style="background-color: #a855f7"></div>
    <div style="background-color: #9333ea"></div> <div style="background-color: #7e22ce"></div>
    <div style="background-color: #6b21a8"></div> <div style="background-color: #581c87"></div>
    <div style="background-color: #4c136e"></div>
</div>
<div class="color">
    <div class="name">

[`FUCHSIA`]</div>
    <div style="background-color: #fdf4ff"></div> <div style="background-color: #fae8ff"></div>
    <div style="background-color: #f5d0fe"></div> <div style="background-color: #f0abfc"></div>
    <div style="background-color: #e879f9"></div> <div style="background-color: #d946ef"></div>
    <div style="background-color: #c026d3"></div> <div style="background-color: #a21caf"></div>
    <div style="background-color: #86198f"></div> <div style="background-color: #701a75"></div>
    <div style="background-color: #4e145b"></div>
</div>
<div class="color">
    <div class="name">

[`PINK`]</div>
    <div style="background-color: #fdf2f8"></div> <div style="background-color: #fce7f3"></div>
    <div style="background-color: #fbcfe8"></div> <div style="background-color: #f9a8d4"></div>
    <div style="background-color: #f472b6"></div> <div style="background-color: #ec4899"></div>
    <div style="background-color: #db2777"></div> <div style="background-color: #be185d"></div>
    <div style="background-color: #9d174d"></div> <div style="background-color: #831843"></div>
    <div style="background-color: #5f0b37"></div>
</div>
<div class="color">
   <div class="name">

[`BLACK`]</div>
    <div style="background-color: #000000; width:22rem"></div>
</div>
<div class="color">
    <div class="name">

[`WHITE`]</div>
    <div style="background-color: #ffffff; width:22rem"></div>
</div>
</div>
</div>

# Example

```rust
use ratatui_core::style::Color;
use ratatui_core::style::palette::tailwind::{BLUE, RED};

assert_eq!(RED.c500, Color::Rgb(239, 68, 68));
assert_eq!(BLUE.c500, Color::Rgb(59, 130, 246));
```



