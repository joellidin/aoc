use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct LastingEffect {
    name: &'static str,
    field: Field,
}

#[derive(Debug, Clone, Copy)]
enum Field {
    Damage(u16, u16),
    Armor(u16, u16),
    Mana(u16, u16),
}

#[derive(Debug, Clone, Copy)]
struct ImidiateEffect {
    damage: u16,
    healing: u16,
}

#[derive(Debug, Clone, Copy)]
enum Effect {
    Lasting(LastingEffect),
    Imidiate(ImidiateEffect),
}

#[derive(Debug, Clone, Copy)]
struct Spell {
    name: &'static str,
    cost: u16,
    effect: Effect,
}

#[derive(Debug, Clone)]
struct SpellBook {
    spells: Vec<Spell>,
}

impl SpellBook {
    fn new() -> Self {
        SpellBook { spells: Vec::new() }
    }

    fn default() -> Self {
        use Field::*;

        let mut book = SpellBook::new();
        book.add_spell(
            "Magic Missile",
            53,
            Effect::Imidiate(ImidiateEffect {
                damage: 4,
                healing: 0,
            }),
        );
        book.add_spell(
            "Drain",
            73,
            Effect::Imidiate(ImidiateEffect {
                damage: 2,
                healing: 2,
            }),
        );
        book.add_spell(
            "Shield",
            113,
            Effect::Lasting(LastingEffect {
                name: "Shield",
                field: Armor(7, 5),
            }),
        );
        book.add_spell(
            "Poison",
            173,
            Effect::Lasting(LastingEffect {
                name: "Poison",
                field: Damage(3, 5),
            }),
        );
        book.add_spell(
            "Recharge",
            229,
            Effect::Lasting(LastingEffect {
                name: "Recharge",
                field: Mana(101, 4),
            }),
        );
        book
    }

    fn add_spell(&mut self, name: &'static str, cost: u16, effect: Effect) {
        self.spells.push(Spell { name, cost, effect });
    }
}

impl LastingEffect {
    fn new(name: &'static str, field: Field) -> Self {
        Self { name, field }
    }
}

#[derive(Debug, Clone)]
struct Player {
    hp: u16,
    armor: u16,
    mana: u16,
    active_effects: Vec<LastingEffect>,
}

#[derive(Debug, Clone)]
struct Boss {
    hp: u16,
    damage: u16,
}

impl FromStr for Boss {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.lines();
        let hp = v
            .next()
            .unwrap()
            .strip_prefix("Hit Points: ")
            .unwrap()
            .parse()
            .unwrap();
        let damage = v
            .next()
            .unwrap()
            .strip_prefix("Damage: ")
            .unwrap()
            .parse()
            .unwrap();
        Ok(Boss {
            hp,
            damage,
        })
    }
}

impl PartialOrd for Boss {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Boss {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hp.cmp(&other.hp)
    }
}

impl PartialEq for Boss {
    fn eq(&self, other: &Self) -> bool {
        self.hp == other.hp
    }
}

impl Eq for Boss {}

impl Player {
    fn new(hp: u16, mana: u16) -> Self {
        Player {
            hp,
            mana,
            armor: 0,
            active_effects: Vec::new(),
        }
    }

    fn default() -> Self {
        Player::new(50, 500)
    }

    fn cast_spell(&mut self, spell: &Spell, boss: &mut Boss) -> bool {
        let mut new_mana = 0u16;
        for effect in self.active_effects.iter() {
            if let Field::Mana(mana, _) = effect.field {
                new_mana = new_mana.saturating_add(mana)
            }
        }
        let active_spell_names = self
            .active_effects
            .iter()
            .filter(|eff| match eff.field {
                Field::Damage(_, t) => t > 0,
                Field::Armor(_, t) => t > 0,
                Field::Mana(_, t) => t > 0,
            })
            .map(|eff| eff.name)
            .collect::<Vec<_>>();

        if self.mana + new_mana < spell.cost || active_spell_names.contains(&spell.name) {
            return false;
        }
        self.mana = self.mana - spell.cost + new_mana;
        for effect in self.active_effects.iter() {
            if let Field::Damage(damage, _) = effect.field {
                boss.hp = boss.hp.saturating_sub(damage)
            }
        }
        self.decrease_effect_duration();
        match spell.effect {
            Effect::Lasting(eff) => self.active_effects.push(eff),
            Effect::Imidiate(eff) => {
                self.hp = self.hp.saturating_add(eff.healing);
                boss.hp = boss.hp.saturating_sub(eff.damage);
            }
        }
        true
    }

    fn decrease_effect_duration(&mut self) {
        use Field::*;

        let mut active_effects = Vec::new();
        for effect in self.active_effects.iter() {
            match effect.field {
                Damage(damage, time) if time > 0 => {
                    active_effects.push(LastingEffect::new(effect.name, Damage(damage, time - 1)));
                }
                Armor(armor, time) if time > 0 => {
                    active_effects.push(LastingEffect::new(effect.name, Armor(armor, time - 1)));
                }
                Mana(mana, time) if time > 0 => {
                    active_effects.push(LastingEffect::new(effect.name, Mana(mana, time - 1)));
                }
                _ => (),
            }
        }
        self.active_effects = active_effects;
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.mana.cmp(&other.mana))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mana.cmp(&other.mana)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.mana == other.mana
    }
}

impl Eq for Player {}

fn dijkstras(boss: &Boss, player: &Player, spell_book: &SpellBook, hard_mode: bool) -> Option<u16> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), player.clone(), boss.clone()));

    while let Some((Reverse(cost), mut player, boss)) = heap.pop() {
        if hard_mode && player.hp <= 1 {
            continue;
        } else if hard_mode {
            player.hp -= 1;
        }

        for spell in spell_book.spells.iter() {
            let mut new_player = player.clone();
            let mut new_boss = boss.clone();
            if new_player.cast_spell(spell, &mut new_boss) {
                player.armor = 0;
                for effect in new_player.active_effects.iter() {
                    match effect.field {
                        Field::Damage(damage, _) => {
                            new_boss.hp = new_boss.hp.saturating_sub(damage)
                        }
                        Field::Armor(armor, _) => player.armor = armor,
                        Field::Mana(mana, _) => {
                            new_player.mana = new_player.mana.saturating_add(mana)
                        }
                    }
                }
                new_player.decrease_effect_duration();

                if new_boss.hp == 0 {
                    return Some(cost + spell.cost);
                }
                new_player.hp = new_player
                    .hp
                    .saturating_sub(new_boss.damage.saturating_sub(player.armor).max(1));

                if new_player.hp == 0 {
                    continue;
                }

                heap.push((Reverse(cost + spell.cost), new_player, new_boss));
            }
        }
    }
    None
}

pub fn solution() {
    let boss = std::fs::read_to_string("data/day22.txt")
        .unwrap()
        .parse::<Boss>()
        .unwrap();
    let player = Player::default();
    let spells = SpellBook::default();
    println!("I need to spend at least {} mana to win", dijkstras(&boss, &player, &spells, false).unwrap());
    println!("I need to spend at least {} mana in hard mode to win", dijkstras(&boss, &player, &spells, true).unwrap());
}
