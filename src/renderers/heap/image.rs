use crate::image::ImageOperations;
use crate::renderers::heap::HeapBuffer;
use crate::renderers::GraphicsRenderer;

// TODO - Optimize this to just memcpy the image directly onto the heap
impl<T: GraphicsRenderer> ImageOperations for HeapBuffer<T> {}
