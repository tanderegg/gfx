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

use comptr::ComPtr;
use std::ptr;
use winapi;
use winapi::*;

use core::{self, command, pso, state, target, IndexType, VertexCount};
use native::{self, CommandBuffer, GeneralCommandBuffer, GraphicsCommandBuffer, ComputeCommandBuffer, TransferCommandBuffer, SubpassCommandBuffer};
use {Resources as R};

pub struct SubmitInfo(pub ComPtr<winapi::ID3D12GraphicsCommandList>);

impl CommandBuffer {
    fn end(&mut self) -> SubmitInfo {
        unsafe { self.inner.Close(); }
        SubmitInfo(self.inner.clone())
    }

    fn pipeline_barrier(&mut self) {
        unimplemented!()
    }

    fn execute_commands(&mut self) {
        unimplemented!()
    }

    fn update_buffer(&mut self, buffer: &native::Buffer, data: &[u8], offset: usize) {
        unimplemented!()
    }

    fn copy_buffer(&mut self, src: &native::Buffer, dst: &native::Buffer, regions: Option<&[command::BufferCopy]>) {
        match regions {
            Some(regions) => {
                // copy each region
                for region in regions {
                    unsafe {
                        self.inner.CopyBufferRegion(
                            dst.resource.as_mut_ptr(), // pDstResource
                            region.dst as UINT64,      // DstOffset
                            src.resource.as_mut_ptr(), // pSrcResource
                            region.src as UINT64,      // SrcOffset
                            region.size as UINT64,     // NumBytes
                        );
                    }
                }
            }
            None => {
                // copy the whole resource
                unsafe {
                    self.inner.CopyResource(
                        dst.resource.as_mut_ptr(), // pDstResource
                        src.resource.as_mut_ptr(), // pSrcResource
                    );
                }
            }
        }
    }

    fn copy_image(&mut self, src: &native::Image, dest: &native::Image) {
        unimplemented!()
    }

    fn copy_buffer_to_image(&mut self) {
        unimplemented!()
    }

    fn copy_image_to_buffer(&mut self) {
        unimplemented!()
    }

    fn clear_color(&mut self, rtv: &native::RenderTargetView, value: command::ClearColor) {
        let clear_color = match value {
            command::ClearColor::Float(c) => c,
            command::ClearColor::Int(c) => [c[0] as FLOAT, c[1] as FLOAT, c[2] as FLOAT, c[3] as FLOAT], // TODO: error?
            command::ClearColor::Uint(c) => [c[0] as FLOAT, c[1] as FLOAT, c[2] as FLOAT, c[3] as FLOAT], // TODO: error?
        };

        unsafe {
            self.inner.ClearRenderTargetView(
                rtv.handle,      // RenderTargetView
                &clear_color,     // ColorRGBA
                0,               // NumRects
                ptr::null_mut(), // pRects
            );
        }
    }

    fn clear_buffer(&mut self) {
        unimplemented!()
    }

    fn bind_pipeline(&mut self, pso: &native::Pipeline) {
        unimplemented!()
    }

    fn bind_descriptor_sets(&mut self) {
        unimplemented!()
    }

    fn push_constants(&mut self) {
        unimplemented!()
    }

    fn clear_attachment(&mut self) {
        unimplemented!()
    }

    fn draw(&mut self, start: VertexCount, count: VertexCount, instances: Option<command::InstanceParams>) {
        let (num_instances, start_instance) = match instances {
            Some((num_instances, start_instance)) => (num_instances, start_instance),
            None => (1, 0),
        };

        unsafe {
            self.inner.DrawInstanced(
                count,          // VertexCountPerInstance
                num_instances,  // InstanceCount
                start,          // StartVertexLocation
                start_instance, // StartInstanceLocation
            );
        }
    }

    fn draw_indexed(&mut self, start: VertexCount, count: VertexCount, base: VertexCount, instances: Option<command::InstanceParams>) {
        let (num_instances, start_instance) = match instances {
            Some((num_instances, start_instance)) => (num_instances, start_instance),
            None => (1, 0),
        };

        unsafe {
            self.inner.DrawIndexedInstanced(
                count,          // IndexCountPerInstance
                num_instances,  // InstanceCount
                start,          // StartIndexLocation
                base as INT,    // BaseVertexLocation
                start_instance, // StartInstanceLocation
            );
        }
    }

    fn draw_indirect(&mut self) {
        unimplemented!()
    }

    fn draw_indexed_indirect(&mut self) {
        unimplemented!()
    }

    fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        unsafe {
            self.inner.Dispatch(
                x, // ThreadGroupCountX
                y, // ThreadGroupCountY
                z, // ThreadGroupCountZ
            );
        }
    }

    fn dispatch_indirect(&mut self) {
        unimplemented!()
    }

    fn bind_index_buffer(&mut self, ib: &native::Buffer, index_type: IndexType) {
        let format = match index_type {
            IndexType::U16 => winapi::DXGI_FORMAT_R16_UINT,
            IndexType::U32 => winapi::DXGI_FORMAT_R32_UINT,
        };
        let location = unsafe { (*ib.resource.as_mut_ptr()).GetGPUVirtualAddress() };

        let mut ibv = winapi::D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: location,
            SizeInBytes: ib.size,
            Format: format,
        };

        unsafe {
            self.inner.IASetIndexBuffer(&mut ibv);
        }
    }

    fn bind_vertex_buffers(&mut self, _: pso::VertexBufferSet<R>) {
        unimplemented!()
    }

    fn set_viewports(&mut self, viewports: &[target::Rect]) {
        let viewports = viewports.iter().map(|viewport| {
            winapi::D3D12_VIEWPORT {
                TopLeftX: viewport.x as FLOAT,
                TopLeftY: viewport.y as FLOAT,
                Width: viewport.w as FLOAT,
                Height: viewport.h as FLOAT,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }
        }).collect::<Vec<_>>();

        unsafe {
            self.inner.RSSetViewports(
                viewports.len() as UINT, // NumViewports
                viewports.as_ptr(),      // pViewports
            );
        }
    }

    fn set_scissors(&mut self, scissors: &[target::Rect]) {
        unimplemented!()
    }

    fn set_ref_values(&mut self, _: state::RefValues) {
        unimplemented!()
    }

    fn clear_depth_stencil(&mut self, _: &(), depth: Option<target::Depth>, stencil: Option<target::Stencil>) {
        unimplemented!()
    }

    fn resolve_image(&mut self) {
        unimplemented!()
    }
}

// CommandBuffer trait implementation
macro_rules! impl_cmd_buffer {
    ($buffer:ident) => (
        impl command::CommandBuffer for $buffer {
            type SubmitInfo = SubmitInfo;
            unsafe fn end(&mut self) -> SubmitInfo {
                self.0.end()
            }
        }
    )
}

impl_cmd_buffer!(GeneralCommandBuffer);
impl_cmd_buffer!(GraphicsCommandBuffer);
impl_cmd_buffer!(ComputeCommandBuffer);
impl_cmd_buffer!(TransferCommandBuffer);
impl_cmd_buffer!(SubpassCommandBuffer);

// PrimaryCommandBuffer trait implementation
macro_rules! impl_primary_cmd_buffer {
    ($buffer:ident) => (
        impl core::PrimaryCommandBuffer<R> for $buffer {
            fn pipeline_barrier(&mut self) {
                self.0.pipeline_barrier()
            }

            fn execute_commands(&mut self) {
                self.0.execute_commands()
            }
        }
    )
}

impl_primary_cmd_buffer!(GeneralCommandBuffer);
impl_primary_cmd_buffer!(GraphicsCommandBuffer);
impl_primary_cmd_buffer!(ComputeCommandBuffer);
impl_primary_cmd_buffer!(TransferCommandBuffer);

// ProcessingCommandBuffer trait implementation
macro_rules! impl_processing_cmd_buffer {
    ($buffer:ident) => (
        impl core::ProcessingCommandBuffer<R> for $buffer {
            fn clear_color(&mut self, rtv: &native::RenderTargetView, value: command::ClearColor) {
                self.0.clear_color(rtv, value)
            }

            fn clear_buffer(&mut self) {
                self.0.clear_buffer()
            }

            fn bind_pipeline(&mut self, pso: &native::Pipeline) {
                self.0.bind_pipeline(pso)
            }

            fn bind_descriptor_sets(&mut self) {
                self.0.bind_descriptor_sets()
            }

            fn push_constants(&mut self) {
                self.0.push_constants()
            }
        }
    )
}

impl_processing_cmd_buffer!(GeneralCommandBuffer);
impl_processing_cmd_buffer!(GraphicsCommandBuffer);
impl_processing_cmd_buffer!(ComputeCommandBuffer);

// TransferCommandBuffer trait implementation
macro_rules! impl_transfer_cmd_buffer {
    ($buffer:ident) => (
        impl core::TransferCommandBuffer<R> for $buffer {
            fn update_buffer(&mut self, buffer: &native::Buffer, data: &[u8], offset: usize) {
                self.0.update_buffer(buffer, data, offset)
            }

            fn copy_buffer(&mut self, src: &native::Buffer, dest: &native::Buffer, regions: Option<&[command::BufferCopy]>) {
                self.0.copy_buffer(src, dest, regions)
            }

            fn copy_image(&mut self, src: &native::Image, dest: &native::Image) {
                self.0.copy_image(src, dest)
            }

            fn copy_buffer_to_image(&mut self) {
                self.0.copy_buffer_to_image()
            }

            fn copy_image_to_buffer(&mut self) {
                self.0.copy_image_to_buffer()
            } 
        }
    )
}

impl_transfer_cmd_buffer!(GeneralCommandBuffer);
impl_transfer_cmd_buffer!(GraphicsCommandBuffer);
impl_transfer_cmd_buffer!(ComputeCommandBuffer);
impl_transfer_cmd_buffer!(TransferCommandBuffer);

// GraphicsCommandBuffer trait implementation
macro_rules! impl_graphics_cmd_buffer {
    ($buffer:ident) => (
        impl core::GraphicsCommandBuffer<R> for $buffer {
            fn clear_depth_stencil(&mut self, dsv: &(), depth: Option<target::Depth>, stencil: Option<target::Stencil>) {
                self.0.clear_depth_stencil(dsv, depth, stencil)
            }

            fn resolve_image(&mut self) {
                self.0.resolve_image()
            }

            fn bind_index_buffer(&mut self, buffer: &native::Buffer, index_type: IndexType) {
                self.0.bind_index_buffer(buffer, index_type)
            }

            fn bind_vertex_buffers(&mut self, vbs: pso::VertexBufferSet<R>) {
                self.0.bind_vertex_buffers(vbs)
            }

            fn set_viewports(&mut self, viewports: &[target::Rect]) {
                self.0.set_viewports(viewports)
            }

            fn set_scissors(&mut self, scissors: &[target::Rect]) {
                self.0.set_scissors(scissors)
            }

            fn set_ref_values(&mut self, rv: state::RefValues) {
                self.0.set_ref_values(rv)
            }
        }
    )
}

impl_graphics_cmd_buffer!(GeneralCommandBuffer);
impl_graphics_cmd_buffer!(GraphicsCommandBuffer);

// ComputeCommandBuffer trait implementation
macro_rules! impl_graphics_cmd_buffer {
    ($buffer:ident) => (
        impl core::ComputeCommandBuffer<R> for $buffer {
            fn dispatch(&mut self, x: u32, y: u32, z: u32) {
                self.0.dispatch(x, y, z)
            }

            fn dispatch_indirect(&mut self) {
                self.0.dispatch_indirect()
            }
        }
    )
}

impl_graphics_cmd_buffer!(GeneralCommandBuffer);
impl_graphics_cmd_buffer!(ComputeCommandBuffer);

// TODO: subpass command buffer
