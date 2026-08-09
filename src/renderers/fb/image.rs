use crate::image::ImageOperations;
use crate::renderers::fb::{DirectFramebufferRenderer, FixedFramebufferColor};

// TODO - Optimize this to just memcpy the image directly onto the heap
impl<C: FixedFramebufferColor> ImageOperations for DirectFramebufferRenderer<C>  {}
