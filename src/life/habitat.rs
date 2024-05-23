pub(crate) enum LandHabitat {
    Plains,
    Desert,
    IslandAndBeach,
    Woodlands,
    Swampland,
    Mountain,
    Arctic,
    Jungle
}

pub(crate) enum WaterHabitat {
    Banks,
    OpenOcean,
    FreshWaterLakes,
    RiverOrStream,
    TropicalLagoon,
    DeepOceanVents,
    SaltWaterSea,
    Reef
}

pub(crate) enum Habitat {
    Land(LandHabitat),
    Water(WaterHabitat),
    Space,
    Exotica
}

pub(crate) enum GenericHabitat {
    Land,
    Water,
    Space,
    Exotica
}

impl GenericHabitat {
    pub fn random(planetside: bool) -> GenericHabitat {

    }
}
