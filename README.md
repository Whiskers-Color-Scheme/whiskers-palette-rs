<div align="center">

<img src="https://raw.githubusercontent.com/Whiskers-Color-Scheme/assets/main/logos/logo-rounded.webp" width="100">

### Whiskers for [Rust](https://www.rust-lang.org/)

<div>
    <img src="https://raw.githubusercontent.com/Whiskers-Color-Scheme/assets/f73d25d4aa4480b7c4d593fb6ae8f4288f3fb5c0/previews/panther-preview.svg" width="300">
    <img src="https://raw.githubusercontent.com/Whiskers-Color-Scheme/assets/f73d25d4aa4480b7c4d593fb6ae8f4288f3fb5c0/previews/tiger-preview.svg" width="300">
</div>
</div>

## 👷‍♂️ Install

- Import the crate in your `cargo.toml` like 
```toml
whiskers_palette_rs = { git = "https://github.com/Whiskers-Color-Scheme/whiskers-palette-rs" }
```

## 🧠 How to use 

To get all the color palette use:
```rust
let palette = get_whiskers_palette();
```

To get the panther palette use:
```rust
let palette = get_panther_palette();
```

To get the tiger palette use:
```rust
let palette = get_tiger_palette();
```

To get a color:
```rust
let color = get_whiskers_palette().panther.banana
let color = get_panther_palette().banana;
let color = get_color(WhiskersColor::PantherBanana);
```

To get values from a color:
```rust
let hex = get_color(WhiskersColor::PantherBanana).hex;

let rgb = get_color(WhiskersColor::PantherBanana).rgb.rgb;
let r = get_color(WhiskersColor::PantherBanana).rgb.r;
let g = get_color(WhiskersColor::PantherBanana).rgb.g;
let b = get_color(WhiskersColor::PantherBanana).rgb.b;

let hsl = get_color(WhiskersColor::PantherBanana).hsl.hsl;
let h = get_color(WhiskersColor::PantherBanana).hsl.h;
let s = get_color(WhiskersColor::PantherBanana).hsl.s;
let l = get_color(WhiskersColor::PantherBanana).hsl.l;
```


## 💻 Maintainers

- [lighttigerXIV](https://github.com/lighttigerxiv)
