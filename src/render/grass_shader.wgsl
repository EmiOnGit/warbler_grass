#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct Config {
    color: vec4<f32>,
    wind: vec2<f32>,
};
@group(1) @binding(0)
var<uniform> mesh: Mesh;

@group(2) @binding(0)
var<uniform> color: vec4<f32>;

@group(2) @binding(1)
var<uniform> wind: vec2<f32>;
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec4<f32>,
    @location(1) position_field_offset: vec3<f32>,
    @location(2) heigth: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var position = vertex.position.xyz + vertex.position_field_offset;
    var out: VertexOutput;

    // Displacing the top of the grass. 
    // Can only affect the top vertex since vertex.position.y is 0 for all others
    // TODO find a better random function
    let strength = abs(wind.x) + abs(wind.y);
    position.x += sin(position.z * position.z - vertex.position.y  * globals.time * strength) / 10. + vertex.position.y * wind.x / 4.;
    position.z += sin(position.x * position.z + vertex.position.y  * globals.time * strength ) / 10. + vertex.position.y * wind.y / 4.;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    // The grass should be darker at the buttom
    out.color = color * (vertex.position.y + 0.1) * 0.3;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}