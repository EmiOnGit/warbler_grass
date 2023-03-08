#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct ShaderRegionConfiguration {
    main_color: vec4<f32>,
    bottom_color: vec4<f32>,
    wind: vec2<f32>,
    _wasm_padding: vec2<f32>,
};
struct Vertex {
    @location(0) vertex_position: vec3<f32>,
    @location(3) xz_position: vec2<f32>,
}
@group(1) @binding(0)
var<uniform> mesh: Mesh;

@group(2) @binding(0)
var<uniform> config: ShaderRegionConfiguration;

@group(2) @binding(1)
var noise_texture: texture_2d<f32>;

#ifdef HEIGHT_MAP
    @group(3) @binding(0)
    var height_map: texture_2d<f32>;

    @group(3) @binding(1)
    var<uniform> aabb: vec3<f32>;
#else
    @group(3) @binding(0)
    var y_positions: texture_2d<f32>;
#endif

// @group(4) @binding(0)
// var xz_positions: texture_2d<f32>;

@group(4) @binding(0)
var heights: texture_2d<f32>;

#import bevy_pbr::mesh_functions

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
const BIG_PRIME: f32 = 7759.;

fn density_map_offset(vertex_position: vec2<f32>) -> vec2<f32> {
    var texture_position = vec2<f32>(vertex_position.x ,vertex_position.y) * BIG_PRIME ;
    
    // dimensions of noise texture in vec2<u32>
    let dim = textureDimensions(noise_texture, 0);

    // read just position in case of a over/under flow of tex. coords
    texture_position = abs(texture_position % vec2<f32>(dim));
    var texture_pixel = textureLoad(noise_texture, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0);
    return texture_pixel.xz - vec2<f32>(0.5,0.5) ;
}
#ifdef HEIGHT_MAP
    fn height_map_offset(vertex_position: vec2<f32>) -> f32 {
        let dim = textureDimensions(height_map, 0);
        let texture_position = abs((vertex_position.xy / aabb.xz ) * vec2<f32>(dim)) ;
        var texture_r = textureLoad(height_map, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0).r;
        return texture_r * aabb.y;
    }
#endif

// 2d textures are used to store vertex information.
// normally this would be done using storage buffers.
// Storage buffer as of now are not supported by wgsl, therefore this hack is used
fn storage_pixel_from_texture(index: u32, texture: texture_2d<f32>) -> vec4<f32> {
    let dim = vec2<u32>(textureDimensions(texture, 0));
    let coord = vec2<u32>(index % dim.x, index / dim.x);
    let pixel = textureLoad(texture,coord,0);
    return(pixel);
}

@vertex
fn vertex(vertex: Vertex, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // load explicit xz positions
    // let xz_pixel = storage_pixel_from_texture(instance_index, xz_positions);
    // var position_field_offset = vec3<f32>(xz_pixel.r, 0.,xz_pixel.g);
    var position_field_offset = vec3<f32>(vertex.xz_position.x, 0.,vertex.xz_position.y);

    let density_offset = density_map_offset(position_field_offset.xz) / 1.;
    position_field_offset += vec3<f32>(density_offset.x, 0.,density_offset.y);
    // position of the vertex in the y_texture
    // ---Y_POSITIONS---
    #ifdef HEIGHT_MAP
        // from height map
        position_field_offset.y = height_map_offset(position_field_offset.xz);
    #else
        // from explicit y positions
        position_field_offset.y = storage_pixel_from_texture(instance_index, y_positions).r;
    #endif
    // ---HEIGHT---
    let height = storage_pixel_from_texture(instance_index, heights).r;
    var position = vertex.vertex_position * vec3<f32>(1.,height, 1.) + position_field_offset;

    // ---WIND---
    // only applies wind if the vertex is not on the bottom of the grass (or very small)
    let offset = wind_offset(position_field_offset.xz);
    let strength = max(0.,log(vertex.vertex_position.y + 1.));
    position.x += offset.x * strength;
    position.z += offset.y * strength;
    
    // ---CLIP_POSITION---
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    // ---COLOR---
    let lambda = clamp(vertex.vertex_position.y, 0.,1.);
    out.color = mix(config.bottom_color, config.main_color, lambda);
    return out;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}