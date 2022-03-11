use std::sync::Arc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Buffer,
    BufferDescriptor, CommandEncoder, CommandEncoderDescriptor, ComputePipeline,
    ComputePipelineDescriptor, Device, Error, ErrorFilter, Features, Limits, Maintain,
    PipelineLayout, PipelineLayoutDescriptor, QuerySet, QuerySetDescriptor, RenderBundleEncoder,
    RenderBundleEncoderDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler,
    SamplerDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderModuleDescriptorSpirV, Texture,
    TextureDescriptor, UncapturedErrorHandler,
};

#[async_trait::async_trait]
pub trait DeviceAsyncExt {
    async fn poll_async(&self, maintain: Maintain);
    async fn features_async(&self) -> Features;
    async fn limits_async(&self) -> Limits;
    async fn create_shader_module_async(
        &self,
        desc: ShaderModuleDescriptor<'static>,
    ) -> ShaderModule;
    async unsafe fn create_shader_module_unchecked_async(
        &self,
        desc: ShaderModuleDescriptor<'static>,
    ) -> ShaderModule;
    async unsafe fn create_shader_module_spirv_async(
        &self,
        desc: ShaderModuleDescriptorSpirV<'static>,
    ) -> ShaderModule;
    async fn create_command_encoder_async(
        &self,
        desc: CommandEncoderDescriptor<'static>,
    ) -> CommandEncoder;
    async fn create_render_bundle_encoder_async<'a>(
        &'a self,
        desc: RenderBundleEncoderDescriptor<'a>,
    ) -> RenderBundleEncoder<'a>;
    async fn create_bind_group_async(&self, desc: BindGroupDescriptor<'static>) -> BindGroup;
    async fn create_bind_group_layout_async(
        &self,
        desc: BindGroupLayoutDescriptor<'static>,
    ) -> BindGroupLayout;
    async fn create_pipeline_layout_async(
        &self,
        desc: PipelineLayoutDescriptor<'static>,
    ) -> PipelineLayout;
    async fn create_render_pipeline_async(
        &self,
        desc: RenderPipelineDescriptor<'static>,
    ) -> RenderPipeline;
    async fn create_compute_pipeline_async(
        &self,
        desc: ComputePipelineDescriptor<'static>,
    ) -> ComputePipeline;
    async fn create_buffer_async(&self, desc: BufferDescriptor<'static>) -> Buffer;
    async fn create_texture_async(&self, desc: TextureDescriptor<'static>) -> Texture;
    async fn create_sampler_async(&self, desc: SamplerDescriptor<'static>) -> Sampler;
    async fn create_query_set_async(&self, desc: QuerySetDescriptor<'static>) -> QuerySet;
    async fn on_uncaptured_error_async(&self, handler: impl UncapturedErrorHandler);
    async fn push_error_scope_async(&self, filter: ErrorFilter);
    async fn pop_error_scope_async(&self) -> Option<Error>;
    async fn start_capture_async(&self);
    async fn stop_capture_async(&self);
}

#[async_trait::async_trait]
impl DeviceAsyncExt for Arc<Device> {
    async fn poll_async(&self, maintain: Maintain) {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.poll(maintain));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn features_async(&self) -> Features {
        self.features()
    }

    async fn limits_async(&self) -> Limits {
        self.limits()
    }

    async fn create_shader_module_async(
        &self,
        desc: ShaderModuleDescriptor<'static>,
    ) -> ShaderModule {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_shader_module(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async unsafe fn create_shader_module_unchecked_async(
        &self,
        desc: ShaderModuleDescriptor<'static>,
    ) -> ShaderModule {
        let device = Arc::clone(self);
        let handle =
            tokio::task::spawn_blocking(move || device.create_shader_module_unchecked(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async unsafe fn create_shader_module_spirv_async(
        &self,
        desc: ShaderModuleDescriptorSpirV<'static>,
    ) -> ShaderModule {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_shader_module_spirv(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_command_encoder_async(
        &self,
        desc: CommandEncoderDescriptor<'static>,
    ) -> CommandEncoder {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_command_encoder(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_render_bundle_encoder_async<'a>(
        &'a self,
        desc: RenderBundleEncoderDescriptor<'a>,
    ) -> RenderBundleEncoder<'a> {
        self.create_render_bundle_encoder(&desc)
    }

    async fn create_bind_group_async(&self, desc: BindGroupDescriptor<'static>) -> BindGroup {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_bind_group(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_bind_group_layout_async(
        &self,
        desc: BindGroupLayoutDescriptor<'static>,
    ) -> BindGroupLayout {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_bind_group_layout(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_pipeline_layout_async(
        &self,
        desc: PipelineLayoutDescriptor<'static>,
    ) -> PipelineLayout {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_pipeline_layout(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_render_pipeline_async(
        &self,
        desc: RenderPipelineDescriptor<'static>,
    ) -> RenderPipeline {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_render_pipeline(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_compute_pipeline_async(
        &self,
        desc: ComputePipelineDescriptor<'static>,
    ) -> ComputePipeline {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_compute_pipeline(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_buffer_async(&self, desc: BufferDescriptor<'static>) -> Buffer {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_buffer(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_texture_async(&self, desc: TextureDescriptor<'static>) -> Texture {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_texture(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_sampler_async(&self, desc: SamplerDescriptor<'static>) -> Sampler {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_sampler(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn create_query_set_async(&self, desc: QuerySetDescriptor<'static>) -> QuerySet {
        let device = Arc::clone(self);
        let handle = tokio::task::spawn_blocking(move || device.create_query_set(&desc));

        // If the inner code panics, we just pass the panic along
        handle.await.unwrap()
    }

    async fn on_uncaptured_error_async(&self, handler: impl UncapturedErrorHandler) {
        self.on_uncaptured_error(handler)
    }

    async fn push_error_scope_async(&self, filter: ErrorFilter) {
        self.push_error_scope(filter)
    }

    async fn pop_error_scope_async(&self) -> Option<Error> {
        self.pop_error_scope().await
    }

    async fn start_capture_async(&self) {
        self.start_capture()
    }

    async fn stop_capture_async(&self) {
        self.stop_capture()
    }
}
