struct VertexInput{
    @location(0) position: vec2<f32>
}
struct InstanceInput{
    @location(1) x: f32,
    @location(2) y: f32,
    @location(3) colour: vec3<f32>,
}

struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vertex_position: vec2<f32>,
    @location(1) colour: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct Camera{
    position: vec2<f32>,
    zoom: f32,
    aspect: f32
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;

    out.colour = instance.colour;
    out.vertex_position = vertex.position;
    out.clip_position = vec4<f32>((vertex.position + vec2<f32>(instance.x,instance.y) - camera.position) * camera.zoom * vec2<f32>(1.0,camera.aspect),0.,1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if length(in.vertex_position) >1.0{
        discard;
    }
    return vec4<f32>(in.colour,1.0);
}