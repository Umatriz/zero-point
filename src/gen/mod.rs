use bevy::{
    app::{Plugin, Startup, Update},
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        reflect::ReflectResource,
        system::{Commands, Res, ResMut, Resource},
    },
    math::Vec2,
    pbr::{PbrBundle, StandardMaterial},
    reflect::Reflect,
    render::{
        color::Color,
        mesh::{shape::Plane, Mesh, VertexAttributeValues},
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
    let noise = Noise::new(
        config.seed,
        config.width,
        config.height,
        config.scale,
        config.octaves,
        config.persistance,
        config.lacunarity,
        config.offset,
    )
    .generate_map();
    let map = &noise.noise_map;

    let mut plane = Mesh::from(Plane {
        size: 15.,
        // vertices.length=(subdivision+2)^2
        subdivisions: config.subdivisions,
    });
    colorize(&mut plane, &noise, map);
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
    let noise = Noise::new(
        config.seed,
        config.width,
        config.height,
        config.scale,
        config.octaves,
        config.persistance,
        config.lacunarity,
        config.offset,
    )
    .generate_map();
    let map = &noise.noise_map;

    if let Some(mesh) = meshes.get_mut(noise_mesh_handle.0.clone()) {
        colorize(mesh, &noise, map)
    }
}

fn colorize(plane: &mut Mesh, noise: &Noise, map: &[f64]) {
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        plane.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        let mut colors: Vec<[f32; 4]> = vec![[0.0, 0.0, 0.0, 0.0]; noise.height() * noise.width()];
        for y in 0..noise.height() {
            for x in 0..noise.width() {
                let val = map[x + y * noise.width()];
                // let num = ((val * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
                // let cl = Color::rgb_u8(num, num, num).as_linear_rgba_f32();
                let cl = crate::utils::color_lerp(Color::BLACK, Color::WHITE, val as f32);
                // FIXME
                colors[x + y * noise.width()] = cl.into();
            }
        }
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
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
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
        }
    }
}

pub struct Noise {
    perlin: Perlin,
    seed: u32,
    width: usize,
    height: usize,
    scale: f64,
    octaves: usize,
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
    noise_map: Vec<f64>,
}

impl Noise {
    pub fn new(
        seed: u32,
        width: usize,
        height: usize,
        scale: f64,
        octaves: usize,
        persistance: f64,
        lacunarity: f64,
        offset: Vec2,
    ) -> Self {
        let perlin = Perlin::new(seed);
        let mut scale = scale;

        if scale <= 0.0 {
            scale = 0.0001
        }

        Self {
            perlin,
            width,
            height,
            scale,
            noise_map: vec![],
            octaves,
            persistance,
            lacunarity,
            seed,
            offset,
        }
    }

    fn generate_map(self) -> Self {
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

        Self { noise_map, ..self }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
