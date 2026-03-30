# fbgl-rs

This is a rewrite of [fbgl](https://github.com/abc123me/fbgl) in Rust, as FBGL was 8 years old and written in the god forsaken
language of C++. Since I couldn't get it to compile in buildroot plus it was crap to begin with, it was time to ditch it.

Rust seems like a perfectly good language to write a graphics library in.

## Features

- Framebuffer support (via the [framebuffer crate](https://docs.rs/framebuffer/latest/))
- Image support (via the [image crate](https://docs.rs/image/latest/))
- Text support (via the [fontdue crate](https://docs.rs/fontdue/latest/))
- SDL support (in progress)
- MultiDisplayHorizontalRenderer object
  - A special object optimized for rendering an image on multiple horizontally arranged displays
- Generic BufferedRenderer object
- Generic GraphicsOperations
  - Line drawing
  - Rectangle drawing (all four sides!)
  - Ellipse drawing (mostly works now!)
  - Pixel setting
  - Pixel getting
- Universal color support
  - Out of box 24-bit RGB color support
  - Framebuffer feature adds inate 16-bit 565 RGB color support

## Testing / Memory safety / Stability

NOTE - The framebuffer feature is not memory safe, and none of the code is tested or stable

NOTE - The driver will simply crash anytime you use it improperly, as error handling is slow and on a Zynq 7000 running at 800MHz
I see no reason to waste clock cycles on it, there are debug build only assertions and expectations placed in key sections to aid
in troubleshooting

There is a singular unit test, and it sometimes passes

## Buildroot support

For a reference package, [see my jlbsp repo](https://github.com/abc123me/jlbsp/tree/develop/packages/fbgl-rs)

Bad apple example, [see bad-apple-rs](https://github.com/abc123me/bad-apple-rs/blob/master/src/main.rs)
