#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;
@group(2) @binding(1)
var<uniform> colorr: vec4<f32>;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec4<f32>,
    @location(1) i_pos_scale: vec3<f32>,
    @location(2) heigth: f32,
    // @location(3) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var position = vertex.position.xyz + vertex.i_pos_scale;
    var out: VertexOutput;
    // Displacing the top of the grass. 
    // Can only affect the top vertex since vertex.position.y is 0 for all others
    position.x += sin(vertex.position.y * position.z * globals.time / 10.) / 10.;
    position.z += sin(vertex.position.y * position.x * globals.time / 10.) / 10.3;
    
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));
    // The grass should be darker at the buttom
    out.color = colorr * (vertex.position.y + 0.1) * 0.3;
    // out.color = colorr;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}