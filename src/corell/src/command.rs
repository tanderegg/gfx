// Copyright 2017 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Command Buffer device interface

use std::ops::{Deref, DerefMut};
use {state, pso, target};
use {IndexType, InstanceCount, VertexCount, Resources};

/// A universal clear color supporting integet formats
/// as well as the standard floating-point.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ClearColor {
    /// Standard floating-point vec4 color
    Float([f32; 4]),
    /// Integer vector to clear ivec4 targets.
    Int([i32; 4]),
    /// Unsigned int vector to clear uvec4 targets.
    Uint([u32; 4]),
}

/// Region of two buffers for copying.
pub struct BufferCopy {
    /// Buffer region source offset.
    pub src: usize,
    /// Buffer region destionation offset.
    pub dst: usize,
    /// Region size.
    pub size: usize,
}

/// Optional instance parameters: (instance count, buffer offset)
pub type InstanceParams = (InstanceCount, VertexCount);

/// Encoder wrapper for a command buffer, providing a safe interface.
///
/// After finishing recording the encoder will be consumed and returns a thread-free `Submit` handle.
/// This handle can be sent to a command queue for execution.
pub struct Encoder<'a, C: CommandBuffer + 'a>(&'a mut C);

impl<'a, C: CommandBuffer> Encoder<'a, C> {
    #[doc(hidden)]
    pub unsafe fn new(buffer: &'a mut C) -> Self {
        Encoder(buffer)
    }

    /// Finish recording commands to the command buffers.
    pub fn finish(self) -> Submit<C> {
        Submit(unsafe { self.0.end() })
    }
}

impl<'a, C> Deref for Encoder<'a, C>
    where C: CommandBuffer
{
    type Target = C;

    fn deref(&self) -> &C {
        self.0
    }
}

impl<'a, C> DerefMut for Encoder<'a, C>
    where C: CommandBuffer
{
    fn deref_mut(&mut self) -> &mut C {
        self.0
    }
}

pub struct Submit<C: CommandBuffer>(C::SubmitInfo);
impl<C: CommandBuffer> Submit<C> {
    #[doc(hidden)]
    pub unsafe fn get_info(&self) -> &C::SubmitInfo {
        &self.0
    }
}

pub trait GraphicsCommandBuffer<R: Resources> : PrimaryCommandBuffer<R> {
    fn clear_depth_stencil(&mut self, &R::DepthStencilView, Option<target::Depth>, Option<target::Stencil>);

    // TODO: investigate how `blit_image` can be emulated on d3d12 e.g compute shader. (useful for mipmap generation)
    fn resolve_image(&mut self);

    fn bind_index_buffer(&mut self, &R::Buffer, IndexType);
    fn bind_vertex_buffers(&mut self, pso::VertexBufferSet<R>);

    fn set_viewports(&mut self, &[target::Rect]);
    fn set_scissors(&mut self, &[target::Rect]);
    fn set_ref_values(&mut self, state::RefValues);
}

pub trait RenderPassEncoder<C: GraphicsCommandBuffer<R>, R: Resources> {
    fn begin(&mut C, &R::RenderPass) -> Self;
    fn next_subpass(&mut self);
}

pub trait SubpassCommandBuffer<R: Resources> : SecondaryCommandBuffer<R> {
    fn clear_attachment(&mut self);
    fn draw(&mut self, start: VertexCount, count: VertexCount, Option<InstanceParams>);
    fn draw_indexed(&mut self, start: VertexCount, count: VertexCount, base: VertexCount, Option<InstanceParams>);
    fn draw_indirect(&mut self);
    fn draw_indexed_indirect(&mut self);

    fn bind_index_buffer(&mut self, &R::Buffer, IndexType);
    fn bind_vertex_buffers(&mut self, pso::VertexBufferSet<R>);

    fn set_viewports(&mut self, &[target::Rect]);
    fn set_scissors(&mut self, &[target::Rect]);
    fn set_ref_values(&mut self, state::RefValues);

    fn bind_pipeline(&mut self, &R::PipelineStateObject);
    fn bind_descriptor_sets(&mut self);
    fn push_constants(&mut self);
}

pub trait ComputeCommandBuffer<R: Resources> : ProcessingCommandBuffer<R> {
    fn dispatch(&mut self, u32, u32, u32);
    fn dispatch_indirect(&mut self);
}

pub trait ProcessingCommandBuffer<R: Resources> : TransferCommandBuffer<R> {
    fn clear_color(&mut self, &R::RenderTargetView, ClearColor);
    fn clear_buffer(&mut self);

    // TODO: consider splitting compute and graphics pso
    fn bind_pipeline(&mut self, &R::PipelineStateObject);
    fn bind_descriptor_sets(&mut self);
    fn push_constants(&mut self);
}

pub trait TransferCommandBuffer<R: Resources> : PrimaryCommandBuffer<R> {
    fn update_buffer(&mut self, &R::Buffer, data: &[u8], offset: usize);

    // TODO: memory aliasing or overlapping regions will result in undefined behavior!
    fn copy_buffer(&mut self, src: &R::Buffer, dest: &R::Buffer, regions: Option<&[BufferCopy]>);
    fn copy_image(&mut self, src: &R::Image, dest: &R::Image);
    fn copy_buffer_to_image(&mut self);
    fn copy_image_to_buffer(&mut self); 
}

pub trait PrimaryCommandBuffer<R: Resources>: CommandBuffer {
    fn pipeline_barrier(&mut self);
    fn execute_commands(&mut self);
}

pub trait SecondaryCommandBuffer<R: Resources>: CommandBuffer {
    fn pipeline_barrier(&mut self);
}

pub trait CommandBuffer {
    type SubmitInfo;

    #[doc(hidden)]
    unsafe fn end(&mut self) -> Self::SubmitInfo;
}

// Ignore for the moment (:
/*
// vk: primary/seconday | outside
fn set_event(&mut self); // vk: Graphics/Compute // d3d12:! emulation needed
// vk: primary/seconday | outside
fn reset_event(&mut self); // vk: Graphics/Compute
// vk: primary/seconday | inside/outside
fn wait_event(&mut self); // vk: Graphics/Compute

// vk: primary/seconday | inside/outside // d3d12: primary
fn begin_query(&mut self); // vk: Graphics/Compute // d3d12: BeginQuery
// vk: primary/seconday | inside/outside // d3d12: primary
fn end_query(&mut self); // vk: Graphics/Compute // d3d12: EndQuery
// vk: primary/seconday | outside
fn reset_query_pool(&mut self); // vk: Graphics/Compute
// vk: primary/seconday | inside/outside
fn write_timestamp(&mut self); // vk: Graphics/Compute
// vk: primary/seconday | outside
fn copy_query_pool_results(&mut self); // vk: Graphics/Compute
*/
