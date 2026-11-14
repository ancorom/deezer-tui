**image > imageops > affine**

# Module: imageops::affine

## Contents

**Functions**

- [`flip_horizontal`](#flip_horizontal) - Flip an image horizontally
- [`flip_horizontal_in`](#flip_horizontal_in) - Flip an image horizontally and put the result into the destination [`ImageBuffer`].
- [`flip_horizontal_in_place`](#flip_horizontal_in_place) - Flip an image horizontally in place.
- [`flip_vertical`](#flip_vertical) - Flip an image vertically
- [`flip_vertical_in`](#flip_vertical_in) - Flip an image vertically and put the result into the destination [`ImageBuffer`].
- [`flip_vertical_in_place`](#flip_vertical_in_place) - Flip an image vertically in place.
- [`rotate180`](#rotate180) - Rotate an image 180 degrees clockwise.
- [`rotate180_in`](#rotate180_in) - Rotate an image 180 degrees clockwise and put the result into the destination [`ImageBuffer`].
- [`rotate180_in_place`](#rotate180_in_place) - Rotate an image 180 degrees clockwise in place.
- [`rotate270`](#rotate270) - Rotate an image 270 degrees clockwise.
- [`rotate270_in`](#rotate270_in) - Rotate an image 270 degrees clockwise and put the result into the destination [`ImageBuffer`].
- [`rotate90`](#rotate90) - Rotate an image 90 degrees clockwise.
- [`rotate90_in`](#rotate90_in) - Rotate an image 90 degrees clockwise and put the result into the destination [`ImageBuffer`].

---

## image::imageops::affine::flip_horizontal

*Function*

Flip an image horizontally

```rust
fn flip_horizontal<I>(image: &I) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::affine::flip_horizontal_in

*Function*

Flip an image horizontally and put the result into the destination [`ImageBuffer`].

```rust
fn flip_horizontal_in<I, Container>(image: &I, destination: & mut crate::ImageBuffer<<I as >::Pixel, Container>) -> crate::ImageResult<()>
```



## image::imageops::affine::flip_horizontal_in_place

*Function*

Flip an image horizontally in place.

```rust
fn flip_horizontal_in_place<I>(image: & mut I)
```



## image::imageops::affine::flip_vertical

*Function*

Flip an image vertically

```rust
fn flip_vertical<I>(image: &I) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::affine::flip_vertical_in

*Function*

Flip an image vertically and put the result into the destination [`ImageBuffer`].

```rust
fn flip_vertical_in<I, Container>(image: &I, destination: & mut crate::ImageBuffer<<I as >::Pixel, Container>) -> crate::ImageResult<()>
```



## image::imageops::affine::flip_vertical_in_place

*Function*

Flip an image vertically in place.

```rust
fn flip_vertical_in_place<I>(image: & mut I)
```



## image::imageops::affine::rotate180

*Function*

Rotate an image 180 degrees clockwise.

```rust
fn rotate180<I>(image: &I) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::affine::rotate180_in

*Function*

Rotate an image 180 degrees clockwise and put the result into the destination [`ImageBuffer`].

```rust
fn rotate180_in<I, Container>(image: &I, destination: & mut crate::ImageBuffer<<I as >::Pixel, Container>) -> crate::ImageResult<()>
```



## image::imageops::affine::rotate180_in_place

*Function*

Rotate an image 180 degrees clockwise in place.

```rust
fn rotate180_in_place<I>(image: & mut I)
```



## image::imageops::affine::rotate270

*Function*

Rotate an image 270 degrees clockwise.

```rust
fn rotate270<I>(image: &I) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::affine::rotate270_in

*Function*

Rotate an image 270 degrees clockwise and put the result into the destination [`ImageBuffer`].

```rust
fn rotate270_in<I, Container>(image: &I, destination: & mut crate::ImageBuffer<<I as >::Pixel, Container>) -> crate::ImageResult<()>
```



## image::imageops::affine::rotate90

*Function*

Rotate an image 90 degrees clockwise.

```rust
fn rotate90<I>(image: &I) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::affine::rotate90_in

*Function*

Rotate an image 90 degrees clockwise and put the result into the destination [`ImageBuffer`].

```rust
fn rotate90_in<I, Container>(image: &I, destination: & mut crate::ImageBuffer<<I as >::Pixel, Container>) -> crate::ImageResult<()>
```



