struct CameraUniform {
    scale: f32,
    position: vec2<f32>,
    screen_size: vec2<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) mass: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) mass: f32,
}

fn orthographic_matrix(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> mat4x4<f32> {
    let inv_w = 1.0 / (right - left);
    let inv_h = 1.0 / (top - bottom);
    let inv_d = 1.0 / (far - near);

    return mat4x4<f32>(
        vec4<f32>(2.0 * inv_w, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 2.0 * inv_h, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, -2.0 * inv_d, 0.0),
        vec4<f32>(-(right + left) * inv_w, -(top + bottom) * inv_h, -(far + near) * inv_d, 1.0)
    );
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let aspect = camera.screen_size.x / camera.screen_size.y;

    // Корректируем границы проекции так, чтобы левая/правая зависели от aspect
    let left = -aspect;
    let right = aspect;
    let bottom = -1.0;
    let top = 1.0;

    let ortho_matrix = orthographic_matrix(left, right, bottom, top, 0.1, 10.0);
    
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position / 200.0, 1.0) * ortho_matrix;
    output.mass = input.mass;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.mass, input.mass, input.mass, 1.0);
}
