use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Buffer,
    BufferDescriptor, CommandEncoder, CommandEncoderDescriptor, ComputePipeline,
    ComputePipelineDescriptor, Device, Error, ErrorFilter, Features, Limits, Maintain,
    PipelineLayout, PipelineLayoutDescriptor, QuerySet, QuerySetDescriptor, Queue,
    RenderBundleEncoder, RenderBundleEncoderDescriptor, RenderPipeline, RenderPipelineDescriptor,
    Sampler, SamplerDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderModuleDescriptorSpirV,
    Texture, TextureDescriptor, UncapturedErrorHandler,
};

/// This trait adds async versions of the standard methods
///
/// The naming convention is the same as the original name, with an "_async" postfix
#[async_trait::async_trait]
pub trait DeviceAsyncExt {
    /// Async version of the `poll` method
    ///
    /// See [wgpu::Device::poll]
    async fn poll_async(&self, maintain: Maintain);

    /// Async version of the `features` method
    ///
    /// See [wgpu::Device::features]
    async fn features_async(&self) -> Features;

    /// Async version of the `limits` method
    ///
    /// See [wgpu::Device::limits]
    async fn limits_async(&self) -> Limits;

    /// Async version of the `create_shader_module` method
    ///
    /// See [wgpu::Device::create_shader_module]
    async fn create_shader_module_async<'a>(
        &'a self,
        desc: &'a ShaderModuleDescriptor<'a>,
    ) -> ShaderModule;

    /// Async version of the `create_shader_module_unchecked` method
    ///
    /// See [wgpu::Device::create_shader_module_unchecked]
    async unsafe fn create_shader_module_unchecked_async<'a>(
        &'a self,
        desc: &'a ShaderModuleDescriptor<'a>,
    ) -> ShaderModule;

    /// Async version of the `create_shader_module_spirv` method
    ///
    /// See [wgpu::Device::create_shader_module_spirv]
    async unsafe fn create_shader_module_spirv_async<'a>(
        &'a self,
        desc: &'a ShaderModuleDescriptorSpirV<'a>,
    ) -> ShaderModule;

    /// Async version of the `create_command_encoder` method
    ///
    /// See [wgpu::Device::create_command_encoder]
    async fn create_command_encoder_async<'a>(
        &'a self,
        desc: &'a CommandEncoderDescriptor<'a>,
    ) -> CommandEncoder;

    /// Async version of the `create_render_bundle_encoder` method
    ///
    /// See [wgpu::Device::create_render_bundle_encoder]
    async fn create_render_bundle_encoder_async<'a>(
        &'a self,
        desc: &'a RenderBundleEncoderDescriptor<'a>,
    ) -> RenderBundleEncoder<'a>;

    /// Async version of the `create_bind_group` method
    ///
    /// See [wgpu::Device::create_bind_group]
    async fn create_bind_group_async<'a>(&'a self, desc: &'a BindGroupDescriptor<'a>) -> BindGroup;

    /// Async version of the `create_bind_group_layout` method
    ///
    /// See [wgpu::Device::create_bind_group_layout]
    async fn create_bind_group_layout_async<'a>(
        &'a self,
        desc: &'a BindGroupLayoutDescriptor<'a>,
    ) -> BindGroupLayout;

    /// Async version of the `create_pipeline_layout` method
    ///
    /// See [wgpu::Device::create_pipeline_layout]
    async fn create_pipeline_layout_async<'a>(
        &'a self,
        desc: &'a PipelineLayoutDescriptor<'a>,
    ) -> PipelineLayout;

    /// Async version of the `create_render_pipeline` method
    ///
    /// See [wgpu::Device::create_render_pipeline]
    async fn create_render_pipeline_async<'a>(
        &'a self,
        desc: &'a RenderPipelineDescriptor<'a>,
    ) -> RenderPipeline;

    /// Async version of the `create_compute_pipeline` method
    ///
    /// See [wgpu::Device::create_compute_pipeline]
    async fn create_compute_pipeline_async<'a>(
        &'a self,
        desc: &'a ComputePipelineDescriptor<'a>,
    ) -> ComputePipeline;

    /// Async version of the `create_buffer` method
    ///
    /// See [wgpu::Device::create_buffer]
    async fn create_buffer_async<'a>(&'a self, desc: &'a BufferDescriptor<'a>) -> Buffer;

    /// Async version of the `create_texture` method
    ///
    /// See [wgpu::Device::create_texture]
    async fn create_texture_async<'a>(&'a self, desc: &'a TextureDescriptor<'a>) -> Texture;

    /// Async version of the `create_sampler` method
    ///
    /// See [wgpu::Device::create_sampler]
    async fn create_sampler_async<'a>(&'a self, desc: &'a SamplerDescriptor<'a>) -> Sampler;

    /// Async version of the `create_query_set` method
    ///
    /// See [wgpu::Device::create_query_set]
    async fn create_query_set_async<'a>(&'a self, desc: &'a QuerySetDescriptor<'a>) -> QuerySet;

    /// Async version of the `on_uncaptured_error` method
    ///
    /// See [wgpu::Device::on_uncaptured_error]
    async fn on_uncaptured_error_async(&self, handler: impl UncapturedErrorHandler);

    /// Async version of the `push_error_scope` method
    ///
    /// See [wgpu::Device::push_error_scope]
    async fn push_error_scope_async(&self, filter: ErrorFilter);

    /// Async version of the `pop_error_scope` method
    ///
    /// See [wgpu::Device::pop_error_scope]
    async fn pop_error_scope_async(&self) -> Option<Error>;

    /// Async version of the `start_capture` method
    ///
    /// See [wgpu::Device::start_capture]
    async fn start_capture_async(&self);

    /// Async version of the `stop_capture` method
    ///
    /// See [wgpu::Device::stop_capture]
    async fn stop_capture_async(&self);

    /// Async version of the `create_buffer_init` method
    ///
    /// See [wgpu::util::DeviceExt::create_buffer_init]
    async fn create_buffer_init_async<'a>(&'a self, desc: &'a BufferInitDescriptor<'a>) -> Buffer;

    /// Async version of the `create_texture_with_data` method
    ///
    /// See [wgpu::util::DeviceExt::create_texture_with_data]
    async fn create_texture_with_data_async<'a>(
        &'a self,
        queue: &Arc<Queue>,
        desc: &'a TextureDescriptor<'a>,
        data: Vec<u8>,
    ) -> Texture;
}

#[async_trait::async_trait]
impl DeviceAsyncExt for Device {
    async fn poll_async(&self, maintain: Maintain) {
        tokio::task::block_in_place(|| self.poll(maintain));
    }

    async fn features_async(&self) -> Features {
        self.features()
    }

    async fn limits_async(&self) -> Limits {
        self.limits()
    }

    async fn create_shader_module_async<'a>(
        &'a self,
        desc: &'a ShaderModuleDescriptor<'a>,
    ) -> ShaderModule {
        tokio::task::block_in_place(|| self.create_shader_module(desc))
    }

    async unsafe fn create_shader_module_unchecked_async<'a>(
        &'a self,
        desc: &'a ShaderModuleDescriptor<'a>,
    ) -> ShaderModule {
        tokio::task::block_in_place(|| self.create_shader_module_unchecked(desc))
    }

    async unsafe fn create_shader_module_spirv_async<'a>(
        &'a self,
        desc: &'a ShaderModuleDescriptorSpirV<'a>,
    ) -> ShaderModule {
        tokio::task::block_in_place(|| self.create_shader_module_spirv(desc))
    }

    async fn create_command_encoder_async<'a>(
        &'a self,
        desc: &'a CommandEncoderDescriptor<'a>,
    ) -> CommandEncoder {
        tokio::task::block_in_place(|| self.create_command_encoder(desc))
    }

    async fn create_render_bundle_encoder_async<'a>(
        &'a self,
        desc: &'a RenderBundleEncoderDescriptor<'a>,
    ) -> RenderBundleEncoder<'a> {
        tokio::task::block_in_place(|| self.create_render_bundle_encoder(desc))
    }

    async fn create_bind_group_async<'a>(&'a self, desc: &'a BindGroupDescriptor<'a>) -> BindGroup {
        tokio::task::block_in_place(|| self.create_bind_group(desc))
    }

    async fn create_bind_group_layout_async<'a>(
        &'a self,
        desc: &'a BindGroupLayoutDescriptor<'a>,
    ) -> BindGroupLayout {
        tokio::task::block_in_place(|| self.create_bind_group_layout(desc))
    }

    async fn create_pipeline_layout_async<'a>(
        &'a self,
        desc: &'a PipelineLayoutDescriptor<'a>,
    ) -> PipelineLayout {
        tokio::task::block_in_place(|| self.create_pipeline_layout(desc))
    }

    async fn create_render_pipeline_async<'a>(
        &'a self,
        desc: &'a RenderPipelineDescriptor<'a>,
    ) -> RenderPipeline {
        tokio::task::block_in_place(|| self.create_render_pipeline(desc))
    }

    async fn create_compute_pipeline_async<'a>(
        &'a self,
        desc: &'a ComputePipelineDescriptor<'a>,
    ) -> ComputePipeline {
        tokio::task::block_in_place(|| self.create_compute_pipeline(desc))
    }

    async fn create_buffer_async<'a>(&'a self, desc: &'a BufferDescriptor<'a>) -> Buffer {
        tokio::task::block_in_place(|| self.create_buffer(desc))
    }

    async fn create_texture_async<'a>(&'a self, desc: &'a TextureDescriptor<'a>) -> Texture {
        tokio::task::block_in_place(|| self.create_texture(desc))
    }

    async fn create_sampler_async<'a>(&'a self, desc: &'a SamplerDescriptor<'a>) -> Sampler {
        tokio::task::block_in_place(|| self.create_sampler(desc))
    }

    async fn create_query_set_async<'a>(&'a self, desc: &'a QuerySetDescriptor<'a>) -> QuerySet {
        tokio::task::block_in_place(|| self.create_query_set(desc))
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

    async fn create_buffer_init_async<'a>(&'a self, desc: &'a BufferInitDescriptor<'a>) -> Buffer {
        tokio::task::block_in_place(|| self.create_buffer_init(desc))
    }

    async fn create_texture_with_data_async<'a>(
        &'a self,
        queue: &Arc<Queue>,
        desc: &'a TextureDescriptor<'a>,
        data: Vec<u8>,
    ) -> Texture {
        tokio::task::block_in_place(|| self.create_texture_with_data(queue, desc, &data))
    }
}
