// TODO - Optimize this to just memcpy the image directly onto the heap
impl<T: GraphicsRenderer> crate::image::ImageOperations for crate::renderers::HeapBuffer<T> {}
