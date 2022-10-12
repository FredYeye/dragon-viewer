pub enum Weapon {
    Sword,
    Hauza,

    SwordTech,
    HauzaTechSpin,
    HauzaTechNoBonus,
    HauzaTech,

    SerpentScale,
    Bomb,
    BowArrow,
    FireRing,
    IceRing,
    LightningRing,
}

impl Weapon {
    pub fn list() -> Vec<Self> {
        vec![
            Self::Sword, Self::Hauza,
            Self::SwordTech, Self::HauzaTechSpin, Self::HauzaTechNoBonus, Self::HauzaTech,
            Self::SerpentScale, Self::Bomb, Self::BowArrow,
            Self::FireRing, Self::IceRing, Self::LightningRing,
        ]
    }

    pub fn name(&self) -> String {
        String::from( match self {
            Self::Sword => "Sword",
            Self::Hauza => "Hauza",

            Self::SwordTech => "Sword Tech",
            Self::HauzaTechSpin => "Hauza Tech (spin)",
            Self::HauzaTechNoBonus => "Hauza Tech (no tech bonus)",
            Self::HauzaTech => "Hauza Tech",

            Self::SerpentScale => "Serpent Scales",
            Self::Bomb => "Bomb",
            Self::BowArrow => "Bow & arrows",
            Self::FireRing => "Fire Ring",
            Self::IceRing => "Ice Ring",
            Self::LightningRing => "Lightning Ring",
        })
    }

    pub fn damage(&self) -> Vec<u8> {
        match self {
            Self::Sword | Self::SwordTech | Self::HauzaTechSpin => vec![15, 25, 35, 45, 55],
            Self::Hauza => vec![20, 30, 40, 50, 60],
            Self::HauzaTechNoBonus | Self::HauzaTech => vec![70],

            Self::SerpentScale => vec![10],
            Self::Bomb => vec![50],
            Self::BowArrow => vec![30],

            Self::FireRing => vec![2, 3, 4],
            Self::IceRing | Self::LightningRing => vec![1, 2, 4],
        }
    }
}
