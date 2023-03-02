#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct ShaderRegionConfiguration {
    main_color: vec4<f32>,
    bottom_color: vec4<f32>,
    wind: vec2<f32>,
    _wasm_padding: vec2<f32>,
};
@group(1) @binding(0)
var<uniform> mesh: Mesh;

@group(2) @binding(0)
var<uniform> config: ShaderRegionConfiguration;

@group(2) @binding(1)
var noise_texture: texture_2d<f32>;

@group(3) @binding(0)
var height_map: texture_2d<f32>;

@group(3) @binding(1)
var<uniform> aabb: vec3<f32>;
@group(4) @binding(0)
var y_positions: texture_1d<f32>;

#import bevy_pbr::mesh_functions

struct Vertex {
    // position of the local vertex in the blade
    @location(0) position: vec3<f32>,
    // position of the blade as an instance
    @location(1) position_field_offset: vec2<f32>,
    // height of the blade
    @location(2) height: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

const NOISE_TEXTURE_SPEED: f32 = 30.;
const NOISE_TEXTURE_ZOOM: f32 = 5.;

fn wind_offset(vertex_position: vec2<f32>) -> vec2<f32> {
    var texture_offset = config.wind.xy * globals.time * NOISE_TEXTURE_SPEED;
    var texture_position = vec2<f32>(vertex_position.x ,vertex_position.y) * NOISE_TEXTURE_ZOOM + texture_offset;
    
    // dimensions of noise texture in vec2<u32>
    let dim = textureDimensions(noise_texture, 0);

    // read just position in case of a over/under flow of tex. coords
    texture_position = abs(texture_position % vec2<f32>(dim));
    var texture_pixel = textureLoad(noise_texture, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0);
    return texture_pixel.xy * config.wind;
}
fn height_map_offset(vertex_position: vec2<f32>) -> f32 {

    let dim = textureDimensions(height_map, 0);
    let texture_position = abs((vertex_position.xy / aabb.xz ) * vec2<f32>(dim)) ;
    var texture_r = textureLoad(height_map, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0).r;
    return texture_r * aabb.y;
}
@vertex
fn vertex(vertex: Vertex, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let field_y_offset = textureLoad(y_positions, u32(instance_index), 0).r;
    let position_field_offset = vec3<f32>(vertex.position_field_offset.x, field_y_offset, vertex.position_field_offset.y);
    var out: VertexOutput;
    var position = vertex.position.xyz * vec3<f32>(1.,vertex.height, 1.) + position_field_offset;
    let local_field_position = vec2<f32>(vertex.position_field_offset.x, position_field_offset.z);
    #ifdef HEIGHT_MAP
        position.y += height_map_offset(position_field_offset.xz );
    #endif
    // only applies wind if the vertex is not on the bottom of the grass (or very small)
    let offset = wind_offset(local_field_position);
    let strength = max(0.,log(vertex.position.y + 1.));
    position.x += offset.x * strength;
    position.z += offset.y * strength;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    let lambda = clamp(vertex.position.y, 0.,1.);
    out.color = mix(config.bottom_color, config.main_color, lambda);
    return out;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}