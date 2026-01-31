# fbgl-rs

This is a rewrite of [fbgl](https://github.com/abc123me/fbgl) in Rust, as FBGL was 8 years old and written in the god forsaken
language of C++.

Rust seems like a perfectly good language to write a graphics library in.

## Features

- Framebuffer support (via the [framebuffer crate](https://docs.rs/framebuffer/latest/))
- Image support (via the [image crate](https://docs.rs/image/latest/))
- Text support (via the [fontdue crate](https://docs.rs/fontdue/latest/))
- Generic BufferedRenderer object
- Generic GraphicsOperations
  - Line drawing
  - Rectangle drawing (all four sides!)
  - Ellipse drawing (TODO)
  - Pixel setting
  - Pixel getting
- Universal color support
  - Out of box 24-bit RGB color support
  - Framebuffer feature adds inate 16-bit RGB color support

## Testing / Memory safety / Stability

The framebuffer feature is not memory safe, and none of the code is tested or stable

## Buildroot support

For a reference package, [see my jlbsp repo](https://github.com/abc123me/jlbsp/tree/develop/packages/fbgl-rs)
