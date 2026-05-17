use std::{cell::RefCell, path::Path, rc::Rc};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::components::{camera::Camera, world::World};

pub const SIZE_RENDER_CELLS: f32 = 10.0;

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 3],
    pub mass: f32,
}

impl Vertex {
    pub fn new(position: [f32; 3], mass: f32) -> Self {
        Self { position, mass }
    }
}

#[derive(Debug)]
pub struct RenderWorld {
    pub vertex_data: Vec<Vertex>,
    pub vertex_count: u32,
    pub render_pipline: wgpu::RenderPipeline,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl RenderWorld {
    /// `count_data` это есть длина нашей сетки `n*m`.
    pub fn new<'a>(
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        count_data: usize,
        camera_render_data: &'a RenderCamera,
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
                bind_group_layouts: &[Some(
                    &camera_render_data
                        .bind_group_layout
                        .as_ref()
                        .unwrap()
                        .clone(),
                )],
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
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }

    fn read_shader_file<P>(path: P) -> std::io::Result<String>
    where
        P: AsRef<Path>,
    {
        std::fs::read_to_string(path)
    }

    fn create_vertices(x: f32, y: f32, mass: f32) -> [Vertex; 6] {
        [
            Vertex::new([x, y, 0.0], mass),
            Vertex::new([x, y + SIZE_RENDER_CELLS, 0.0], mass),
            Vertex::new([x + SIZE_RENDER_CELLS, y + SIZE_RENDER_CELLS, 0.0], mass),
            Vertex::new([x, y, 0.0], mass),
            Vertex::new([x + SIZE_RENDER_CELLS, y, 0.0], mass),
            Vertex::new([x + SIZE_RENDER_CELLS, y + SIZE_RENDER_CELLS, 0.0], mass),
        ]
    }

    pub fn init_data_world(&mut self, world: &World) {
        for y in 0..world.height() {
            for x in 0..world.width() {
                if let Some(cell) = world.get_grid().get_cell(x, y) {
                    let (x, y) = (x as f32 * SIZE_RENDER_CELLS, y as f32 * SIZE_RENDER_CELLS);
                    self.vertex_data
                        .extend(Self::create_vertices(x, y, cell.mass));
                }
            }
        }

        self.vertex_count = self.vertex_data.len() as u32;
    }

    pub fn update_data(&mut self, world: &World) {
        for idx_cell in 0..world.readonly_grid.cells.len() {
            let new_mass = world.readonly_grid.cells[idx_cell].mass;
            let offset = idx_cell * 6;
            for i in 0..6 {
                let idx_vertex = offset + i;
                self.vertex_data[idx_vertex].mass = new_mass;
            }
        }
    }

    pub fn create_vertex_buffer(&mut self, device: &wgpu::Device) {
        if self.vertex_data.is_empty() {
            return;
        }

        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Buffer for Cells"),
                contents: bytemuck::cast_slice(&self.vertex_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
        );
    }

    pub fn get_vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }
}

pub struct RenderCamera {
    pub camera: Rc<RefCell<Camera>>,
    pub uniform_buffer: Option<wgpu::Buffer>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl RenderCamera {
    pub fn new(camera: Rc<RefCell<Camera>>) -> Self {
        Self {
            camera,
            uniform_buffer: None,
            bind_group_layout: None,
            bind_group: None,
        }
    }

    pub fn create_uniform_buffer(mut self, device: &wgpu::Device) -> Self {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::bytes_of(&*self.camera.borrow()), // Преобразуем структуру в байты
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            // Флаг COPY_DST позволяет нам обновлять буфер в будущем
        });

        self.uniform_buffer = Some(uniform_buffer);
        self
    }

    pub fn create_bind_group_layout(mut self, device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera Bind Group Layout"),
        });

        self.bind_group_layout = Some(bind_group_layout);
        self
    }

    pub fn create_bind_group(mut self, device: &wgpu::Device) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.bind_group_layout.as_ref().unwrap(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.uniform_buffer.as_ref().unwrap().as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        self.bind_group = Some(bind_group);
        self
    }
}
