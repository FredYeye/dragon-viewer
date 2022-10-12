use self::{player::Player, weapon::Weapon, enemy::Enemy};

pub mod player;
pub mod enemy;
pub mod weapon;

pub struct DragonView {
    pub player: Player,
    pub current_enemy: Enemy,
    pub is_boss: bool,
}

impl Default for DragonView {
    fn default() -> Self {
        Self {
            player: Player::default(),
            current_enemy: Enemy::BeetleBlue,
            is_boss: false,
        }
    }
}

impl DragonView {
    pub fn damage_dealt(&self) -> Vec<(String, Vec<(u8, bool)>)> {
        let mut damage_dealt = Vec::new();
        let enemy = self.current_enemy.get_enemy();

        for weapon in Weapon::list() {
            let mut damage_vec = Vec::new();

            match weapon {
                Weapon::FireRing | Weapon::IceRing | Weapon::LightningRing => {
                    let idx = match weapon { //bootleg but OK
                        Weapon::FireRing => 0,
                        Weapon::IceRing => 1,
                        Weapon::LightningRing => 2,
                        _ => unreachable!(),
                    };

                    for ring_multiplier in weapon.damage() {
                        damage_vec.push((ring_multiplier * enemy.ring_damage[idx], false));
                    }
                }

                _ => {
                    for weapon_damage in weapon.damage() {
                        let mut attack_power = weapon_damage + self.player.level * 4;

                        attack_power = attack_power.saturating_sub(enemy.defense);

                        let mut zero_damage = false;

                        if attack_power == 0 {
                            attack_power = 1;
                            zero_damage = true;
                        }

                        if let Weapon::SwordTech | Weapon::HauzaTechSpin | Weapon::HauzaTech = weapon {
                            attack_power = attack_power.saturating_add(10);
                        }

                        damage_vec.push((attack_power, zero_damage));
                    }
                }
            }

            damage_dealt.push((weapon.name(), damage_vec));
        }

        damage_dealt
    }
}
