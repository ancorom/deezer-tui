**image > images > sub_image**

# Module: images::sub_image

## Contents

**Structs**

- [`SubImage`](#subimage) - A View into another image
- [`SubImageInner`](#subimageinner) - The inner type of `SubImage` that implements `GenericImage{,View}`.

---

## image::images::sub_image::SubImage

*Struct*

A View into another image

Instances of this struct can be created using:
  - [`GenericImage::sub_image`] to create a mutable view,
  - [`GenericImageView::view`] to create an immutable view,
  - [`SubImage::new`] to instantiate the struct directly.

Note that this does _not_ implement `GenericImage`, but it dereferences to one which allows you
to use it as if it did. See [Design Considerations](#Design-Considerations) below for details.

# Design Considerations

For reasons relating to coherence, this is not itself a `GenericImage` or a `GenericImageView`.
In short, we want to reserve the ability of adding traits implemented for _all_ generic images
but in a different manner for `SubImage`. This may be required to ensure that stacking
sub-images comes at no double indirect cost.

If, ultimately, this is not needed then a directly implementation of `GenericImage` can and
will get added. This inconvenience may alternatively get resolved if Rust allows some forms of
specialization, which might make this trick unnecessary and thus also allows for a direct
implementation.

**Generic Parameters:**
- I

**Methods:**

- `fn new(image: I, x: u32, y: u32, width: u32, height: u32) -> SubImage<I>` - Construct a new subimage
- `fn change_bounds(self: & mut Self, x: u32, y: u32, width: u32, height: u32)` - Change the coordinates of this subimage.
- `fn offsets(self: &Self) -> (u32, u32)` - The offsets of this subimage relative to the underlying image.
- `fn to_image(self: &Self) -> ImageBuffer<<<I as Deref>::Target as GenericImageView>::Pixel, Vec<<<<I as Deref>::Target as GenericImageView>::Pixel as Pixel>::Subpixel>>` - Convert this subimage to an `ImageBuffer`
- `fn view(self: &Self, x: u32, y: u32, width: u32, height: u32) -> SubImage<&<I as >::Target>` - Create a sub-view of the image.
- `fn inner(self: &Self) -> &<I as >::Target` - Get a reference to the underlying image.
- `fn sub_image(self: & mut Self, x: u32, y: u32, width: u32, height: u32) -> SubImage<& mut <I as >::Target>` - Create a mutable sub-view of the image.
- `fn inner_mut(self: & mut Self) -> & mut <I as >::Target` - Get a mutable reference to the underlying image.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## image::images::sub_image::SubImageInner

*Struct*

The inner type of `SubImage` that implements `GenericImage{,View}`.

This type is _nominally_ `pub` but it is not exported from the crate. It should be regarded as
an existential type in any case.

**Generic Parameters:**
- I



