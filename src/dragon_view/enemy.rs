#[derive(PartialEq, Clone, Copy)]
pub enum Enemy {
    LizardKnightGreen,
    MothPink,
    BeetleBlue,
    ArmorPurple,
    MothGreen,
    ScorpionOrange,
    MonkeyPirateBrown,
    ElementalFire,
    GolemIceStone,
    //9 four armed swordsman, snow region?

    GargoyleYellow,
    WizardBlue,
    Sandworm,
    GoldenSandworm,
    Mummy,

    SlimeBlue,
    HugeBugGreen,
    WaterClone,
    ReaperLightGreen,
    GiantGreenPurple,
    LizardKnightBlack,
    GoblinKnightSilver,
    GargoyleTeal,
    BeetleBrown,
    ArmorOrange,
    LizardKnightBlue,
    ScorpionPurple,
    WizardPurple,
    MonkeyPirateBlue,
    ElementalWater,
    FourArmedSwordsmanBlue,

    MothBrown,

    
    GolemStone,
    GiantBlue,
    GargoyleOrange, //comes in more colors (yellow)
    GoblinKnightGold,
    //27 mage in casdra / lake?
    //28 monkey pirate?
    FourArmedSwordsmanGreen,
    ReaperDarkGreen,
    SlimeGray,
    SlimeFire,
    //2D mage?

    // bosses
    JadeDemon,
    Piercia,
    Efreet,
    FrozenHorror,
    Jester,
    WaterDragon,
    Giza,
    Efreet2,
    Piercia2,

    //?
    HugeBugPurple,
}

impl Enemy {
    pub fn list() -> Vec<Self> {
        vec![
            Self::ArmorPurple,
            Self::ArmorOrange,

            Self::BeetleBlue,
            Self::BeetleBrown,

            Self::ElementalFire,
            Self::ElementalWater,

            Self::FourArmedSwordsmanBlue,
            Self::FourArmedSwordsmanGreen,

            Self::GargoyleYellow,
            Self::GargoyleOrange,
            Self::GargoyleTeal,

            Self::GiantGreenPurple,
            Self::GiantBlue,

            Self::GoblinKnightSilver,
            Self::GoblinKnightGold,

            Self::GolemIceStone,
            Self::GolemStone,

            Self::HugeBugGreen,
            Self::HugeBugPurple,

            Self::LizardKnightGreen,
            Self::LizardKnightBlue,
            Self::LizardKnightBlack,

            Self::MonkeyPirateBrown,
            Self::MonkeyPirateBlue,

            Self::MothGreen,
            Self::MothPink,
            Self::MothBrown,

            Self::Mummy,

            Self::ReaperLightGreen,
            Self::ReaperDarkGreen,

            Self::Sandworm,
            Self::GoldenSandworm,

            Self::ScorpionOrange,
            Self::ScorpionPurple,

            Self::SlimeBlue,
            Self::SlimeGray,
            Self::SlimeFire,

            Self::WaterClone,

            Self::WizardBlue,
            Self::WizardPurple,
        ]
    }

    pub fn list_boss() -> Vec<Self> {
        vec![
            Self::Piercia,
            Self::Efreet,
            Self::FrozenHorror,
            Self::Jester,
            Self::JadeDemon,
            Self::WaterDragon,
            Self::Piercia2,
            Self::Efreet2,
            Self::Giza,
        ]
    }

    pub fn get_enemy(&self) -> EnemyData {
        match self {
            Self::LizardKnightGreen => EnemyData {
                name: String::from("Lizard knight (green)"), id: 0x00,
                defense: 3, ring_damage: [10, 10, 10],
                xp: 6,
            },

            Self::MothPink => EnemyData {
                name: String::from("Moth (pink)"), id: 0x01,
                defense: 15, ring_damage: [20, 10, 10],
                xp: 23,
            },

            Self::BeetleBlue => EnemyData {
                name: String::from("Beetle (blue)"), id: 0x02,
                defense: 0, ring_damage: [10, 10, 10],
                xp: 2,
            },

            Self::ArmorPurple => EnemyData {
                name: String::from("Armor (purple)"), id: 0x03,
                defense: 7, ring_damage: [10, 8, 15],
                xp: 7,
            },

            Self::MothGreen => EnemyData {
                name: String::from("Moth (green)"), id: 0x04,
                defense: 3, ring_damage: [20, 10, 10],
                xp: 3,
            },

            Self::ScorpionOrange => EnemyData {
                name: String::from("Scorpion (orange)"), id: 0x05,
                defense: 7, ring_damage: [10, 10, 10],
                xp: 5,
            },

            Self::MonkeyPirateBrown => EnemyData {
                name: String::from("Monkey pirate (brown)"), id: 0x06,
                defense: 35, ring_damage: [5, 15, 10],
                xp: 89,
            },

            Self::ElementalFire => EnemyData {
                name: String::from("Fire elemental"), id: 0x07,
                defense: 35, ring_damage: [1, 20, 10],
                xp: 92,
            },

            Self::GolemIceStone => EnemyData {
                name: String::from("Ice / stone golem"), id: 0x08,
                defense: 73, ring_damage: [20, 1, 10],
                xp: 296,
            },

            Self::GargoyleYellow => EnemyData {
                name: String::from("Gargoyle (yellow)"), id: 0x0B,
                defense: 53, ring_damage: [15, 10, 14],
                xp: 213,
            },

            Self::WizardBlue => EnemyData {
                name: String::from("Wizard (blue)"), id: 0x0C,
                defense: 101, ring_damage: [1, 1, 1],
                xp: 467,
            },

            Self::Sandworm => EnemyData {
                name: String::from("Sandworm"), id: 0x0D,
                defense: 99, ring_damage: [5, 17, 10],
                xp: 735,
            },

            Self::GoldenSandworm => EnemyData {
                name: String::from("Sandworm (Golden)"), id: 0x0E,
                defense: 99, ring_damage: [1, 1, 1],
                xp: 1405,
            },

            Self::Mummy => EnemyData {
                name: String::from("Mummy"), id: 0x0F,
                defense: 113, ring_damage: [10, 20, 10],
                xp: 389,
            },

            Self::SlimeBlue => EnemyData {
                name: String::from("Slime (blue)"), id: 0x10,
                defense: 125, ring_damage: [12, 10, 10],
                xp: 675,
            },

            Self::HugeBugGreen => EnemyData {
                name: String::from("Huge bug (green)"), id: 0x11,
                defense: 149, ring_damage: [5, 5, 5],
                xp: 999,
            },

            Self::WaterClone => EnemyData {
                name: String::from("Water clone"), id: 0x12,
                defense: 133, ring_damage: [12, 10, 10],
                xp: 638,
            },

            Self::ReaperLightGreen => EnemyData {
                name: String::from("Reaper (light green)"), id: 0x13,
                defense: 161, ring_damage: [3, 3, 12],
                xp: 1066,
            },

            Self::GiantGreenPurple => EnemyData {
                name: String::from("Giant (green / purple)"), id: 0x14,
                defense: 157, ring_damage: [15, 12, 15],
                xp: 893,
            },

            Self::LizardKnightBlack => EnemyData {
                name: String::from("Lizard knight (black)"), id: 0x15,
                defense: 157, ring_damage: [5, 5, 15],
                xp: 665,
            },

            Self::GoblinKnightSilver => EnemyData {
                name: String::from("Goblin knight (silver)"), id: 0x16,
                defense: 175, ring_damage: [7, 7, 10],
                xp: 1970,
            },

            Self::GargoyleTeal => EnemyData {
                name: String::from("Gargoyle (teal)"), id: 0x17,
                defense: 175, ring_damage: [10, 6, 8],
                xp: 1741,
            },

            Self::BeetleBrown => EnemyData {
                name: String::from("Beetle (brown)"), id: 0x18,
                defense: 23, ring_damage: [2, 20, 10],
                xp: 73,
            },

            Self::ArmorOrange => EnemyData {
                name: String::from("Armor (orange)"), id: 0x19,
                defense: 57, ring_damage: [15, 5, 10],
                xp: 190,
            },

            Self::LizardKnightBlue => EnemyData {
                name: String::from("Lizard knight (blue)"), id: 0x1A,
                defense: 91, ring_damage: [10, 10, 10],
                xp: 380,
            },

            Self::ScorpionPurple => EnemyData {
                name: String::from("Scorpion (purple)"), id: 0x1B,
                defense: 91, ring_damage: [7, 15, 10],
                xp: 515,
            },

            Self::WizardPurple => EnemyData {
                name: String::from("Wizard (purple)"), id: 0x1C,
                defense: 165, ring_damage: [1, 1, 1],
                xp: 515,
            },

            Self::MonkeyPirateBlue => EnemyData {
                name: String::from("Monkey pirate (blue)"), id: 0x1D,
                defense: 87, ring_damage: [10, 10, 10],
                xp: 442,
            },

            Self::ElementalWater => EnemyData {
                name: String::from("Elemental (water)"), id: 0x1E,
                defense: 141, ring_damage: [15, 8, 10],
                xp: 714,
            },

            Self::FourArmedSwordsmanBlue => EnemyData {
                name: String::from("Four-armed swordsman (blue)"), id: 0x1F,
                defense: 169, ring_damage: [10, 8, 10],
                xp: 1030,
            },

            Self::MothBrown => EnemyData {
                name: String::from("Moth (brown)"), id: 0x20,
                defense: 125, ring_damage: [20, 10, 10],
                xp: 531,
            },

            Self::GolemStone => EnemyData {
                name: String::from("Golem (stone 2)"), id: 0x23,
                defense: 95, ring_damage: [5, 5, 5],
                xp: 594,
            },

            Self::GiantBlue => EnemyData {
                name: String::from("Giant (blue)"), id: 0x24,
                defense: 181, ring_damage: [10, 8, 10],
                xp: 1432,
            },

            Self::GargoyleOrange => EnemyData {
                name: String::from("Gargoyle (orange / yellow)"), id: 0x25,
                defense: 97, ring_damage: [12, 10, 11],
                xp: 493,
            },

            Self::GoblinKnightGold => EnemyData {
                name: String::from("Goblin knight (gold)"), id: 0x26,
                defense: 187, ring_damage: [5, 5, 8],
                xp: 2310,
            },

            Self::FourArmedSwordsmanGreen => EnemyData {
                name: String::from("Four-armed swordsman (blue)"), id: 0x29,
                defense: 187, ring_damage: [8, 6, 8],
                xp: 1283,
            },

            Self::ReaperDarkGreen => EnemyData {
                name: String::from("Reaper (dark green)"), id: 0x2A,
                defense: 183, ring_damage: [1, 1, 10],
                xp: 2089,
            },

            Self::SlimeGray => EnemyData {
                name: String::from("Slime (gray)"), id: 0x2B,
                defense: 99, ring_damage: [2, 2, 10],
                xp: 784,
            },

            Self::SlimeFire => EnemyData {
                name: String::from("Slime (fire)"), id: 0x2C,
                defense: 161, ring_damage: [1, 20, 2],
                xp: 784,
            },

            //-----

            Self::JadeDemon => EnemyData {
                name: String::from("Jade Demon"), id: 0x2F,
                defense: 137, ring_damage: [0, 10, 5],
                xp: 2797,
            },

            Self::Piercia => EnemyData {
                name: String::from("Piercia"), id: 0x30,
                defense: 11, ring_damage: [10, 10, 10],
                xp: 87,
            },

            Self::Efreet => EnemyData {
                name: String::from("Efreet"), id: 0x31,
                defense: 47, ring_damage: [0, 10, 5],
                xp: 483,
            },

            Self::FrozenHorror => EnemyData {
                name: String::from("Frozen Horror"), id: 0x33,
                defense: 81, ring_damage: [10, 0, 5],
                xp: 875,
            },

            Self::Jester => EnemyData {
                name: String::from("Jester"), id: 0x34,
                defense: 131, ring_damage: [2, 2, 2],
                xp: 1798,
            },

            Self::WaterDragon => EnemyData {
                name: String::from("Water dragon"), id: 0x35,
                defense: 161, ring_damage: [8, 0, 5],
                xp: 3681,
            },

            Self::Giza => EnemyData {
                name: String::from("Giza"), id: 0x38,
                defense: 207, ring_damage: [0, 0, 0],
                xp: 0,
            },

            Self::Efreet2 => EnemyData {
                name: String::from("Efreet 2"), id: 0x50,
                defense: 165, ring_damage: [0, 10, 5],
                xp: 3466,
            },

            Self::Piercia2 => EnemyData {
                name: String::from("Piercia 2"), id: 0x52,
                defense: 137, ring_damage: [10, 5, 5],
                xp: 1348,
            },

            //-----

            Self::HugeBugPurple => EnemyData { //starts with 39 hp
                name: String::from("Huge bug (purple)"), id: 0x99,
                defense: 149, ring_damage: [10, 10, 10],
                xp: 0,
            },
        }
    }
}

pub struct EnemyData {
    pub name: String,
    id: u8,
    // attack: Vec<(String, u8)>,
    pub defense: u8,
    pub ring_damage: [u8; 3], //fire, ice, lightning
    pub xp: u16,
}

//todo: some enemies seem to bypass ring damage and instead take some "default" damage instead.
//not sure why or if it's even intended.
//enemies known to exhibit this behavior:
// moth (brown)
// huge bug (green / purple)
// water elemental
