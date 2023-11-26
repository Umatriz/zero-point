use std::marker::PhantomData;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        reflect::ReflectResource,
        system::{Commands, Res, ResMut, Resource},
    },
    math::{Vec2, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    reflect::{std_traits::ReflectDefault, Reflect},
    render::{
        color::Color,
        mesh::{shape::Plane, Mesh, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    transform::components::Transform,
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};
use noise::{NoiseFn, Perlin};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::utils::inv_lerp;

pub mod map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<NoiseMeshHandle>()
            .add_systems(Startup, spawn_noise)
            .add_systems(Update, update_noise)
            .init_resource::<NoiseConfig>()
            .register_type::<NoiseConfig>()
            .register_type::<TerrainType>()
            .add_plugins(ResourceInspectorPlugin::<NoiseConfig>::default());
    }
}

#[derive(Resource, Default)]
struct NoiseMeshHandle(Handle<Mesh>);

#[derive(Component)]
struct NoiseMarker;

/*
    TODO: rewrite this code
*/

fn spawn_noise(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut noise_mesh_handle: ResMut<NoiseMeshHandle>,
    config: Res<NoiseConfig>,
) {
    let subdivisions = config.subdivisions;
    let noise = Noise::from(config).generate_map();

    let mut plane = Mesh::new(PrimitiveTopology::TriangleList).with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        {
            let mut vertices = vec![Vec3::ZERO; noise.width() * noise.height()];
            for y in 0..noise.height() {
                for x in 0..noise.width() {
                    todo!()
                }
            }
            vertices
        },
    );
    colorize(&mut plane, &noise);
    let handle = meshes.add(plane);
    noise_mesh_handle.0 = handle.clone();
    commands
        .spawn(PbrBundle {
            mesh: handle,
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..Default::default()
        })
        .insert(NoiseMarker);
}

fn update_noise(
    noise_mesh_handle: ResMut<NoiseMeshHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<NoiseConfig>,
) {
    let noise = Noise::from(config).generate_map();

    if let Some(mesh) = meshes.get_mut(noise_mesh_handle.0.clone()) {
        colorize(mesh, &noise)
    }
}

fn colorize(plane: &mut Mesh, noise: &Noise<Generated>) {
    if let Some(VertexAttributeValues::Float32x3(_positions)) =
        plane.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors = noise.colorize_map();
        plane.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct NoiseConfig {
    width: usize,
    height: usize,
    #[inspector(min = 0.0)]
    scale: f64,
    seed: u32,
    subdivisions: u32,
    octaves: usize,
    #[inspector(min = 0.0, max = 1.0)]
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
    draw_mode: DrawMode,
    regions: Vec<TerrainType>,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            scale: 22.0,
            seed: Default::default(),
            subdivisions: 98,
            octaves: 4,
            persistance: 0.5,
            lacunarity: 2.0,
            offset: Default::default(),
            regions: vec![
                TerrainType {
                    height: 0.4,
                    color: Color::BLUE,
                    name: "Water".to_string(),
                },
                TerrainType {
                    height: 1.0,
                    color: Color::GREEN,
                    name: "Land".to_string(),
                },
            ],
            draw_mode: DrawMode::ColorMap,
        }
    }
}

enum Generated {}
enum Undefined {}

#[derive(typed_builder::TypedBuilder)]
pub struct Noise<Map> {
    #[builder(default = noise::Perlin::new(seed.0))]
    perlin: Perlin,
    seed: u32,
    width: usize,
    height: usize,
    scale: f64,
    octaves: usize,
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
    regions: Vec<TerrainType>,
    draw_mode: DrawMode,
    #[builder(default = vec![])]
    noise_map: Vec<f64>,
    #[builder(default = std::marker::PhantomData)]
    _marker: PhantomData<Map>,
}

#[derive(Reflect, InspectorOptions, Default, Clone)]
#[reflect(InspectorOptions, Default)]
pub struct TerrainType {
    pub height: f64,
    pub color: Color,
    pub name: String,
}

#[derive(Reflect, InspectorOptions, Default, Clone)]
#[reflect(InspectorOptions, Default)]
pub enum DrawMode {
    NoiseMap,
    #[default]
    ColorMap,
    Mesh,
}

// region: From impls

impl From<NoiseConfig> for Noise<Undefined> {
    fn from(value: NoiseConfig) -> Self {
        Noise::builder()
            .width(value.width)
            .height(value.height)
            .draw_mode(value.draw_mode.clone())
            .lacunarity(value.lacunarity)
            .octaves(value.octaves)
            .offset(value.offset)
            .regions(value.regions.clone())
            .persistance(value.persistance)
            .scale(value.scale)
            .seed(value.seed)
            .build()
    }
}

impl From<Res<'_, NoiseConfig>> for Noise<Undefined> {
    fn from(value: Res<'_, NoiseConfig>) -> Self {
        Noise::builder()
            .width(value.width)
            .height(value.height)
            .draw_mode(value.draw_mode.clone())
            .lacunarity(value.lacunarity)
            .octaves(value.octaves)
            .offset(value.offset)
            .regions(value.regions.clone())
            .persistance(value.persistance)
            .scale(value.scale)
            .seed(value.seed)
            .build()
    }
}

// endregion: From impls

impl<Map> Noise<Map> {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl Noise<Undefined> {
    pub fn generate_map(self) -> Noise<Generated> {
        let mut noise_map = vec![0.0; self.width * self.height];

        let mut prng = StdRng::seed_from_u64(self.seed.into());
        let mut octave_offsets = vec![Vec2::ZERO; self.octaves];

        for i in octave_offsets.iter_mut() {
            let offset_x = prng.gen_range(-100_000.0..100_000.0) + self.offset.x;
            let offset_y = prng.gen_range(-100_000.0..100_000.0) + self.offset.y;
            *i = Vec2::new(offset_x, offset_y);
        }

        let mut max_noise_height = f64::MIN;
        let mut min_noise_height = f64::MAX;

        let half_width = (self.width / 2) as f64;
        let half_height = (self.height / 2) as f64;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut noise_height = 0.0;

                for i in &octave_offsets {
                    let sample_x = (x as f64 - half_width) / self.scale * frequency + (i.x as f64);
                    let sample_y = (y as f64 - half_height) / self.scale * frequency + (i.y as f64);

                    let perlin_value = self.perlin.get([sample_x, sample_y]) * 2.0 - 1.0;
                    noise_height += perlin_value * amplitude;

                    amplitude *= self.persistance;
                    frequency *= self.lacunarity;
                }

                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }
                noise_map[x + y * self.width] = noise_height;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                noise_map[x + y * self.width] = inv_lerp(
                    min_noise_height,
                    max_noise_height,
                    noise_map[x + y * self.width],
                )
            }
        }

        Noise {
            noise_map,
            perlin: self.perlin,
            seed: self.seed,
            width: self.width,
            height: self.width,
            scale: self.scale,
            octaves: self.octaves,
            persistance: self.persistance,
            lacunarity: self.lacunarity,
            offset: self.offset,
            regions: self.regions,
            draw_mode: self.draw_mode,
            _marker: PhantomData,
        }
    }
}

impl Noise<Generated> {
    pub fn colorize_map(&self) -> Vec<[f32; 4]> {
        match self.draw_mode {
            DrawMode::NoiseMap => {
                let mut colors: Vec<[f32; 4]> =
                    vec![[0.0, 0.0, 0.0, 0.0]; self.height * self.width];
                for y in 0..self.height {
                    for x in 0..self.width {
                        let val = self.noise_map[x + y * self.width];
                        let cl = crate::utils::color_lerp(Color::BLACK, Color::WHITE, val as f32);
                        colors[x + y * self.width] = cl.into();
                    }
                }
                colors
            }
            DrawMode::ColorMap | DrawMode::Mesh => {
                let mut colors = vec![[0.0, 0.0, 0.0, 0.0]; self.width * self.height];
                for y in 0..self.height {
                    for x in 0..self.width {
                        let current_height = self.noise_map[x + y * self.width];
                        for region in self.regions.iter() {
                            if current_height <= region.height {
                                colors[x + y * self.width] = region.color.into();
                                break;
                            }
                        }
                    }
                }
                colors
            }
        }
    }

    pub fn generate_mesh(&self) -> MeshData {
        let top_left_x = (self.width - 1) as f32 / -2.0;
        let top_left_z = (self.height - 1) as f32 / 2.0;

        let mut mesh_data = MeshData::new(self.width, self.height);
        let mut vertex_index: usize = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                mesh_data.vertices[vertex_index] = Vec3::new(
                    top_left_x + x as f32,
                    self.noise_map[x + y * self.width] as f32,
                    top_left_z - y as f32,
                );
                mesh_data.uvs[vertex_index] =
                    Vec2::new(x as f32 / self.width as f32, y as f32 / self.height as f32);

                if x < self.width - 1 && y < self.height - 1 {
                    mesh_data.add_triangle(
                        vertex_index,
                        vertex_index + self.width + 1,
                        vertex_index + self.width,
                    );
                    mesh_data.add_triangle(
                        vertex_index + self.width + 1,
                        vertex_index,
                        vertex_index + 1,
                    );
                }

                vertex_index += 1;
            }
        }
        mesh_data
    }
}

pub struct MeshData {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<usize>,
    pub uvs: Vec<Vec2>,
    triangle_index: usize,
}

impl MeshData {
    pub fn new(mesh_width: usize, mesh_height: usize) -> Self {
        Self {
            vertices: vec![Vec3::ZERO; mesh_width * mesh_height],
            triangles: vec![0; (mesh_width - 1) * (mesh_height - 1) * 6],
            uvs: vec![Vec2::ZERO; mesh_width * mesh_height],
            triangle_index: 0,
        }
    }

    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) {
        self.triangles[self.triangle_index] = a;
        self.triangles[self.triangle_index + 1] = b;
        self.triangles[self.triangle_index + 2] = c;
        self.triangle_index += 3;
    }

    pub fn create_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        mesh.compute_flat_normals();
        mesh
    }
}
