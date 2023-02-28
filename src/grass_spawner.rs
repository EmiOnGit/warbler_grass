use bevy::{math::Vec3Swizzles, prelude::*};

use crate::height_map::HeightMap;
#[derive(Default, Component)]
pub struct GrassSpawner {
    positions_xz: Vec<Vec2>,
    positions_y: Vec<f32>,
    heights: HeightRepresentation,
    height_map: Option<HeightMap>,
    _density_map: Option<Handle<Image>>,
    flags: u32,
}

impl GrassSpawner {
    pub fn new() -> GrassSpawner {
        Self::default()
    }
    pub fn with_positions(mut self, positions: Vec<Vec3>) -> GrassSpawner {
        assert!(!positions.is_empty());

        let mut flags = GrassSpawnerFlags::from_bits(self.flags).unwrap();
        if flags.contains(GrassSpawnerFlags::XZ_DEFINED) {
            panic!("Can not insert positions to `GrassSpawner` since the xz positions are already defined");
        }
        if flags.contains(GrassSpawnerFlags::Y_DEFINED) {
            panic!("Can not insert positions to `GrassSpawner` since the y positions are already defined");
        }
        flags.insert(GrassSpawnerFlags::Y_DEFINED);
        flags.insert(GrassSpawnerFlags::XZ_DEFINED);

        let (positions_xz, positions_y) = positions
            .into_iter()
            .map(|position| (position.xz(), position.y))
            .unzip();

        self.flags = flags.bits();
        self.positions_xz = positions_xz;
        self.positions_y = positions_y;

        self.validate();
        self
    }
    pub fn with_positions_xz(mut self, positions_xz: Vec<Vec2>) -> GrassSpawner {
        assert!(!positions_xz.is_empty());

        let mut flags = GrassSpawnerFlags::from_bits(self.flags).unwrap();
        if flags.contains(GrassSpawnerFlags::XZ_DEFINED) {
            panic!("Can not insert positions_xz to `GrassSpawner` since the xz positions are already defined");
        }

        flags.insert(GrassSpawnerFlags::XZ_DEFINED);

        self.flags = flags.bits();
        self.positions_xz = positions_xz;

        self.validate();
        self
    }
    pub fn with_positions_y(mut self, positions_y: Vec<f32>) -> GrassSpawner {
        assert!(!positions_y.is_empty());

        let mut flags = GrassSpawnerFlags::from_bits(self.flags).unwrap();
        if flags.contains(GrassSpawnerFlags::Y_DEFINED) {
            panic!("Can not insert positions_y to `GrassSpawner` since the y positions are already defined");
        }

        flags.insert(GrassSpawnerFlags::Y_DEFINED);

        self.flags = flags.bits();
        self.positions_y = positions_y;

        self.validate();
        self
    }
    pub fn with_heights(mut self, heights: Vec<f32>) -> GrassSpawner {
        assert!(!heights.is_empty());

        let mut flags = GrassSpawnerFlags::from_bits(self.flags).unwrap();
        if flags.contains(GrassSpawnerFlags::HEIGHT_DEFINED) {
            panic!("Can not insert heights to `GrassSpawner` since the heights are already defined");
        }

        flags.insert(GrassSpawnerFlags::HEIGHT_DEFINED);

        self.flags = flags.bits();
        self.heights = HeightRepresentation::PerBlade(heights);

        self.validate();
        self
    }
    pub fn with_height_uniform(mut self, uniform_height: f32) -> GrassSpawner {

        let mut flags = GrassSpawnerFlags::from_bits(self.flags).unwrap();
        if flags.contains(GrassSpawnerFlags::HEIGHT_DEFINED) {
            panic!("Can not insert heights to `GrassSpawner` since the heights are already defined");
        }

        flags.insert(GrassSpawnerFlags::HEIGHT_DEFINED);

        self.flags = flags.bits();
        self.heights = HeightRepresentation::Uniform(uniform_height);

        self
    }
    pub fn with_height_map(mut self, height_map: HeightMap) -> GrassSpawner {
        let mut flags = GrassSpawnerFlags::from_bits(self.flags).unwrap();
        if flags.contains(GrassSpawnerFlags::Y_DEFINED) {
            panic!("Can not insert height map to `GrassSpawner` since the y positions are already defined");
        }

        flags.insert(GrassSpawnerFlags::Y_DEFINED);
        flags.insert(GrassSpawnerFlags::HEIGHT_MAP);

        self.flags = flags.bits();
        self.height_map = Some(height_map);
        self
    }
    fn validate(&self) {
        if !self.positions_xz.is_empty() && !self.positions_y.is_empty() {
            assert_eq!(self.positions_xz.len(), self.positions_y.len());
        }
        if let HeightRepresentation::PerBlade(heights) = &self.heights {
            if !self.positions_y.is_empty() {
                assert_eq!(heights.len(), self.positions_y.len());
            }
            if !self.positions_xz.is_empty() {
                assert_eq!(heights.len(), self.positions_xz.len());
            }
        }
    }
}
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct GrassSpawnerFlags: u32 {
        const Y_DEFINED      = (1 << 0);
        const XZ_DEFINED     = (1 << 1);
        const HEIGHT_DEFINED = (1 << 2);
        const HEIGHT_MAP     = (1 << 3);
        const DENSITY_MAP    = (1 << 4);
        const NONE           = 0;
        const UNINITIALIZED  = 0xFFFF;
    }
}
pub enum HeightRepresentation {
    PerBlade(Vec<f32>),
    Uniform(f32)
}
impl Default for HeightRepresentation {
    fn default() -> Self {
        HeightRepresentation::Uniform(1.)
    }
}