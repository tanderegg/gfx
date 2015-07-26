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

extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;

use glutin::{HeadlessContext, GlContext, PixelFormat};

use gfx::tex::Size;
use gfx_device_gl::{create};

/// A wrapper around the headless context that implements 'Output'
pub struct Output<R: gfx::Resources> {
    width: Size,
    height: Size,
    pub context: HeadlessContext,
    frame: gfx::handle::FrameBuffer<R>,
    mask: gfx::Mask,
    supports_gamma_convertion: bool,
    gamma: gfx::Gamma
}

impl<R: gfx::Resources> Output<R> {
    /// Try to set the gamma conversion.
    pub fn set_gamma(&mut self, gamma: gfx::Gamma) -> Result<(), ()> {
        if self.supports_gamma_convertion || gamma == gfx::Gamma::Original {
            self.gamma = gamma;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<R: gfx::Resources> gfx::Output<R> for Output<R> {
    fn get_handle(&self) -> Option<&gfx::handle::FrameBuffer<R>> {
        Some(&self.frame)
    }

    fn get_size(&self) -> (Size, Size) {
        (self.width, self.height)
    }

    fn get_mask(&self) -> gfx::Mask {
        self.mask
    }

    fn get_gamma(&self) -> gfx::Gamma {
        self.gamma
    }
}

impl<R: gfx::Resources> gfx::Window<R> for Output<R> {
    fn swap_buffers(&mut self) {
        self.context.swap_buffers();
    }
}

/// Result of successful conttext initilaization
pub type Success = (
    gfx::OwnedStream<
        gfx_device_gl::Device,
        Output<gfx_device_gl::Resources>
    >,
    gfx_device_gl::Device,
    gfx_device_gl::Factory
);

/// Initialize with a headless context
pub fn init(width: u16, height: u16,
            context: glutin::HeadlessContext,
            pixel_fmt: Option<PixelFormat>) -> Success {

    use gfx::traits::StreamFactory;

    unsafe { context.make_current(); }

    // Unimplemented for osmesa in Glutin
    //let format = context.get_pixel_format();

    // Instead, allow the user to provide or setup
    // a reasonable default.
    let format = match pixel_fmt {
        Some(fmt) => fmt,
        None => PixelFormat {
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

    let (mut device, mut factory) = create(|s| context.get_proc_address(s));

    let out = Output {
        width: width,
        height: height,
        context: context,
        frame: factory.get_main_frame_buffer(),
        mask: if format.color_bits != 0 { gfx::COLOR } else { gfx::Mask::empty() } |
              if format.depth_bits != 0 { gfx::DEPTH } else  { gfx::Mask::empty() } |
              if format.stencil_bits != 0 { gfx::STENCIL } else { gfx::Mask::empty() },
        supports_gamma_convertion: format.srgb,
        gamma: gfx::Gamma::Original
    };

    let mut stream = factory.create_stream(out);
    (stream, device, factory)
}
