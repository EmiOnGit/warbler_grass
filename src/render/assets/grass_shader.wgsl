#import bevy_pbr::mesh_functions mesh_position_local_to_clip
#import bevy_pbr::mesh_types Mesh
#import bevy_pbr::mesh_view_bindings globals

struct ShaderRegionConfiguration {
    wind: vec2<f32>,
    _wasm_padding: vec2<f32>,
};
struct Vertex {
    @location(0) vertex_position: vec3<f32>,
    @location(3) xz_position: vec2<f32>,
}
struct Color {
    main_color: vec4<f32>,
    bottom_color: vec4<f32>,
}
@group(1) @binding(0)
var<uniform> mesh: Mesh;

@group(2) @binding(0)
var<uniform> config: ShaderRegionConfiguration;

@group(2) @binding(1)
var noise_texture: texture_2d<f32>;

@group(3) @binding(0)
var<uniform> color: Color;

@group(4) @binding(0)
var y_texture: texture_2d<f32>;

struct ShaderAabb {
    vect: vec3<f32>,
    _wasm_padding: f32,
}

@group(4) @binding(1)
var<uniform> aabb: ShaderAabb;

#ifdef HEIGHT_TEXTURE
    @group(5) @binding(0)
    var height_texture: texture_2d<f32>;
#else
    struct ShaderHeightUniform {
        height: f32,
        _wasm_padding: vec2<f32>,
    }
    @group(5) @binding(0)
    var<uniform> height_uniform: ShaderHeightUniform;
#endif

@group(6) @binding(0)
var t_normal: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

const NOISE_TEXTURE_SPEED: f32 = 50.;
const NOISE_TEXTURE_ZOOM: f32 = 35.;
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
const BIG_PRIME: f32 = 1302151.;

fn density_map_offset(vertex_position: vec2<f32>) -> vec2<f32> {
    var texture_position = vec2<f32>(vertex_position.x ,vertex_position.y) * BIG_PRIME ;
    
    // dimensions of noise texture in vec2<u32>
    let dim = textureDimensions(noise_texture, 0);

    // read just position in case of a over/under flow of tex. coords
    texture_position = abs(texture_position % vec2<f32>(dim));
    var texture_pixel = textureLoad(noise_texture, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0);
    return texture_pixel.xz - vec2<f32>(0.5,0.5) ;
}
fn texture2d_offset(texture: texture_2d<f32>, vertex_position: vec2<f32>) -> vec3<f32> {
    let dim = textureDimensions(texture, 0);
    let texture_position = abs((vertex_position.xy / aabb.vect.xz ) * vec2<f32>(dim)) ;
    var texture_rgb = textureLoad(texture, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0).rgb;
    return texture_rgb * aabb.vect.y;
}
// Source: https://gist.github.com/kevinmoran/b45980723e53edeb8a5a43c49f134724
fn rotate_align(v1: vec3<f32>, v2: vec3<f32>) -> mat3x3<f32> {
    let axis = cross(v1, v2);

    let cos_a = dot(v1, v2);
    let k = 1.0 / (1.0 + cos_a);

    let result = mat3x3<f32>( 
        (axis.x * axis.x * k) + cos_a,  (axis.y * axis.x * k) - axis.z, (axis.z * axis.x * k) + axis.y,
        (axis.x * axis.y * k) + axis.z, (axis.y * axis.y * k) + cos_a,  (axis.z * axis.y * k) - axis.x,
        (axis.x * axis.z * k) - axis.y, (axis.y * axis.z * k) + axis.x, (axis.z * axis.z * k) + cos_a 
    );

    return result;
}
@vertex
fn vertex(vertex: Vertex, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var out: VertexOutput;

    var position_field_offset = vec3<f32>(vertex.xz_position.x, 0.,vertex.xz_position.y);

    let density_offset = density_map_offset(position_field_offset.xz) / 1.;
    position_field_offset += vec3<f32>(density_offset.x, 0.,density_offset.y);

    // ---Y_POSITIONS---
    position_field_offset.y = texture2d_offset(y_texture, position_field_offset.xz).r;
    
    // ---NORMAL---
    var normal = texture2d_offset(t_normal, vertex.xz_position.xy).xyz; // Get normal scaled over grass field
    normal = normalize(normal);
    let rotation_matrix = rotate_align(vertex.vertex_position, normal); // Calculate rotation matrix to align grass with normal
    
    // ---HEIGHT---
    var height = 0.;
    #ifdef HEIGHT_TEXTURE
        height = (texture2d_offset(height_texture, position_field_offset.xz).r + 4.) / 3.;
    #else
        height = height_uniform.height;
    #endif
    var position = rotation_matrix * vertex.vertex_position;
    position = position * vec3<f32>(1., height, 1.) + position_field_offset;

    // ---WIND---
    // only applies wind if the vertex is not on the bottom of the grass (or very small)
    let offset = wind_offset(position_field_offset.xz);
    let strength = max(0.,log(vertex.vertex_position.y + 1.));
    position.x += offset.x * strength;
    position.z += offset.y * strength;
    
    // ---CLIP_POSITION---
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    // ---COLOR---
    let lambda = clamp(vertex.vertex_position.y, 0., 1.);
    out.color = mix(color.bottom_color, color.main_color, lambda);
    return out;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
