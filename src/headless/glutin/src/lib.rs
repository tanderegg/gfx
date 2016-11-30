// Copyright 2015 The Gfx-rs Developers.
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

extern crate gfx_core;
extern crate gfx_device_gl;
extern crate glutin;

use gfx_core::{format, handle, texture};
use gfx_core::memory::Typed;
use gfx_device_gl::Resources;

/// Initialize with a headless context
pub fn init<Cf, Df>(
    builder: glutin::HeadlessRendererBuilder,
    pixel_fmt: Option<glutin::PixelFormat>) ->
    (gfx_device_gl::Device, gfx_device_gl::Factory,
     handle::RenderTargetView<Resources, Cf>, handle::DepthStencilView<Resources, Df>)
where
    Cf: format::RenderFormat,
    Df: format::DepthFormat,
{
    // Unimplemented for osmesa in Glutin
    //let format = context.get_pixel_format();

    // Instead, allow the user to provide or setup
    // a reasonable default.

    let format = match pixel_fmt {
        Some(fmt) => fmt,
        None => glutin::PixelFormat {
            hardware_accelerated: false,
            color_bits: 24,
            alpha_bits: 8,
            depth_bits: 24,
            stencil_bits: 8,
            stereoscopy: false,
            double_buffer: true,
            multisampling: None,
            srgb: false
        }
    };

    let color_format = Cf::get_format();
    let ds_format = Df::get_format();

    // Extract info needed to create views
    let (width, height) = builder.dimensions;
    let aa = format.multisampling.unwrap_or(0) as texture::NumSamples;

    // Obtain the context
    let context = builder.build().unwrap();
    unsafe { context.make_current().unwrap(); }

    // Create the gfx objects
    let (device, factory) = gfx_device_gl::create(|s| context.get_proc_address(s) as *const std::os::raw::c_void);

    let (color_view, ds_view) = gfx_device_gl::create_main_targets_raw((width as u16, height as u16, 1, aa.into()), color_format.0, ds_format.0);
    (device, factory, Typed::new(color_view), Typed::new(ds_view))
}
