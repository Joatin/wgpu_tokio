use wgpu::{ImageCopyTexture, Texture, TextureView, TextureViewDescriptor};

/// A trait that adds async methods to the [wgpu::Texture] struct
#[async_trait::async_trait]
pub trait TextureAsyncExt {
    /// Async version of the `create_view` method
    ///
    /// See [wgpu::Texture::create_view]
    async fn create_view_async(&self, desc: &TextureViewDescriptor) -> TextureView;

    /// Async version of the `destroy` method
    ///
    /// See [wgpu::Texture::destroy]
    async fn destroy_async(&self);

    /// Async version of the `as_image_copy` method
    ///
    /// See [wgpu::Texture::as_image_copy]
    async fn as_image_copy_async(&self) -> ImageCopyTexture;
}

#[async_trait::async_trait]
impl TextureAsyncExt for Texture {
    async fn create_view_async(&self, desc: &TextureViewDescriptor) -> TextureView {
        tokio::task::block_in_place(|| self.create_view(desc))
    }

    async fn destroy_async(&self) {
        tokio::task::block_in_place(|| self.destroy())
    }

    async fn as_image_copy_async(&self) -> ImageCopyTexture {
        tokio::task::block_in_place(|| self.as_image_copy())
    }
}
