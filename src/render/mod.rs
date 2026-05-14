use std::path::Path;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::components::world::World;

pub const SIZE_RENDER_CELLS: f32 = 10.0;

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position }
    }
}

#[derive(Debug)]
pub struct RenderWorld {
    vertex_data: Vec<Vertex>,
    pub vertex_count: u32,
    pub render_pipline: wgpu::RenderPipeline,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl RenderWorld {
    /// `count_data` это есть длина нашей сетки `n*m`.
    pub fn new(
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        count_data: usize,
    ) -> anyhow::Result<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader for World (for Cells of Grid)"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(Self::read_shader_file(
                "./shaders/shader_cells.wgsl",
            )?)),
        });

        let render_pipline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipline layout for World"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let render_pipline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipline for World"),
            layout: Some(&render_pipline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Self::create_vertex_buffer_layout_for_cells_vertices()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Ok(Self {
            render_pipline,
            vertex_data: Vec::with_capacity(count_data),
            vertex_count: 0,
            vertex_buffer: None,
        })
    }

    fn create_vertex_buffer_layout_for_cells_vertices() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }

    fn read_shader_file<P>(path: P) -> std::io::Result<String>
    where
        P: AsRef<Path>,
    {
        std::fs::read_to_string(path)
    }

    fn create_vertices(x: f32, y: f32) -> [Vertex; 6] {
        [
            Vertex::new([x, y, 0.0]),
            Vertex::new([x, y + SIZE_RENDER_CELLS, 0.0]),
            Vertex::new([x + SIZE_RENDER_CELLS, y + SIZE_RENDER_CELLS, 0.0]),
            Vertex::new([x, y, 0.0]),
            Vertex::new([x + SIZE_RENDER_CELLS, y, 0.0]),
            Vertex::new([x + SIZE_RENDER_CELLS, y + SIZE_RENDER_CELLS, 0.0]),
        ]
    }

    pub fn init_data_world(&mut self, world: &World) {
        for y in 0..world.height() {
            for x in 0..world.width() {
                let (x, y) = (x as f32 * SIZE_RENDER_CELLS, y as f32 * SIZE_RENDER_CELLS);
                self.vertex_data.extend(Self::create_vertices(x, y));
            }
        }

        self.vertex_count = self.vertex_data.len() as u32;
    }

    pub fn create_vertex_buffer(&mut self, device: &wgpu::Device) {
        if self.vertex_data.is_empty() {
            return;
        }

        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Buffer for Cells"),
                contents: bytemuck::cast_slice(&self.vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
    }

    pub fn get_vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }
}
