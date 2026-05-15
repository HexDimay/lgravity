struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) mass: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) mass: f32,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position / 200.0, 1.0);
    output.mass = input.mass;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.mass, input.mass, input.mass, 1.0);
}
