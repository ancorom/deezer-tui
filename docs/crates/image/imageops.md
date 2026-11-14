**image > imageops**

# Module: imageops

## Contents

**Modules**

- [`colorops`](#colorops) - Functions for altering and converting the color of pixelbufs

**Functions**

- [`crop`](#crop) - Return a mutable view into an image
- [`crop_imm`](#crop_imm) - Return an immutable view into an image
- [`horizontal_gradient`](#horizontal_gradient) - Fill the image with a linear horizontal gradient
- [`overlay`](#overlay) - Overlay an image at a given coordinate (x, y)
- [`overlay_bounds`](#overlay_bounds) - Calculate the region that can be copied from top to bottom.
- [`replace`](#replace) - Replace the contents of an image at a given coordinate (x, y)
- [`tile`](#tile) - Tile an image by repeating it multiple times
- [`vertical_gradient`](#vertical_gradient) - Fill the image with a linear vertical gradient

---

## Module: colorops

Functions for altering and converting the color of pixelbufs



## image::imageops::crop

*Function*

Return a mutable view into an image
The coordinates set the position of the top left corner of the crop.

```rust
fn crop<I>(image: & mut I, x: u32, y: u32, width: u32, height: u32) -> crate::SubImage<& mut I>
```



## image::imageops::crop_imm

*Function*

Return an immutable view into an image
The coordinates set the position of the top left corner of the crop.

```rust
fn crop_imm<I>(image: &I, x: u32, y: u32, width: u32, height: u32) -> crate::SubImage<&I>
```



## image::imageops::horizontal_gradient

*Function*

Fill the image with a linear horizontal gradient

This function assumes a linear color space.

# Examples
```no_run
use image::{Rgba, RgbaImage, Pixel};

let mut img = RgbaImage::new(100, 100);
let start = Rgba::from_slice(&[0, 128, 0, 0]);
let end = Rgba::from_slice(&[255, 255, 255, 255]);

image::imageops::horizontal_gradient(&mut img, start, end);
img.save("horizontal_gradient.png").unwrap();

```rust
fn horizontal_gradient<S, P, I>(img: & mut I, start: &P, stop: &P)
```



## image::imageops::overlay

*Function*

Overlay an image at a given coordinate (x, y)

```rust
fn overlay<I, J>(bottom: & mut I, top: &J, x: i64, y: i64)
```



## image::imageops::overlay_bounds

*Function*

Calculate the region that can be copied from top to bottom.

Given image size of bottom and top image, and a point at which we want to place the top image
onto the bottom image, how large can we be? Have to wary of the following issues:
* Top might be larger than bottom
* Overflows in the computation
* Coordinates could be completely out of bounds

The main idea is to make use of inequalities provided by the nature of `saturating_add` and
`saturating_sub`. These intrinsically validate that all resulting coordinates will be in bounds
for both images.

We want that all these coordinate accesses are safe:
1. `bottom.get_pixel(x + [0..x_range), y + [0..y_range))`
2. `top.get_pixel([0..x_range), [0..y_range))`

Proof that the function provides the necessary bounds for width. Note that all unaugmented math
operations are to be read in standard arithmetic, not integer arithmetic. Since no direct
integer arithmetic occurs in the implementation, this is unambiguous.

```text
Three short notes/lemmata:
- Iff `(a - b) <= 0` then `a.saturating_sub(b) = 0`
- Iff `(a - b) >= 0` then `a.saturating_sub(b) = a - b`
- If  `a <= c` then `a.saturating_sub(b) <= c.saturating_sub(b)`

1.1 We show that if `bottom_width <= x`, then `x_range = 0` therefore `x + [0..x_range)` is empty.

x_range
 = (top_width.saturating_add(x).min(bottom_width)).saturating_sub(x)
<= bottom_width.saturating_sub(x)

bottom_width <= x
<==> bottom_width - x <= 0
<==> bottom_width.saturating_sub(x) = 0
 ==> x_range <= 0
 ==> x_range  = 0

1.2 If `x < bottom_width` then `x + x_range < bottom_width`

x + x_range
<= x + bottom_width.saturating_sub(x)
 = x + (bottom_width - x)
 = bottom_width

2. We show that `x_range <= top_width`

x_range
 = (top_width.saturating_add(x).min(bottom_width)).saturating_sub(x)
<= top_width.saturating_add(x).saturating_sub(x)
<= (top_wdith + x).saturating_sub(x)
 = top_width (due to `top_width >= 0` and `x >= 0`)
```

Proof is the same for height.

```rust
fn overlay_bounds((bottom_width, bottom_height): (u32, u32), (top_width, top_height): (u32, u32), x: u32, y: u32) -> (u32, u32)
```



## image::imageops::replace

*Function*

Replace the contents of an image at a given coordinate (x, y)

```rust
fn replace<I, J>(bottom: & mut I, top: &J, x: i64, y: i64)
```



## image::imageops::tile

*Function*

Tile an image by repeating it multiple times

# Examples
```no_run
use image::RgbaImage;

let mut img = RgbaImage::new(1920, 1080);
let tile = image::open("tile.png").unwrap();

image::imageops::tile(&mut img, &tile);
img.save("tiled_wallpaper.png").unwrap();
```

```rust
fn tile<I, J>(bottom: & mut I, top: &J)
```



## image::imageops::vertical_gradient

*Function*

Fill the image with a linear vertical gradient

This function assumes a linear color space.

# Examples
```no_run
use image::{Rgba, RgbaImage, Pixel};

let mut img = RgbaImage::new(100, 100);
let start = Rgba::from_slice(&[0, 128, 0, 0]);
let end = Rgba::from_slice(&[255, 255, 255, 255]);

image::imageops::vertical_gradient(&mut img, start, end);
img.save("vertical_gradient.png").unwrap();

```rust
fn vertical_gradient<S, P, I>(img: & mut I, start: &P, stop: &P)
```



