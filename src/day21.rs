use std::str::FromStr;

#[derive(Debug)]
struct ShopItem {
    cost: u16,
    damage: u16,
    armor: u16,
}

impl ShopItem {
    fn new(cost: u16, damage: u16, armor: u16) -> Self {
        ShopItem {
            cost,
            damage,
            armor,
        }
    }
}

#[derive(Debug)]
struct Player {
    hit_points: u16,
    damage: u16,
    armor: u16,
}

impl FromStr for Player {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.lines();
        let hit_points = v
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
        let armor = v
            .next()
            .unwrap()
            .strip_prefix("Armor: ")
            .unwrap()
            .parse()
            .unwrap();
        Ok(Player {
            hit_points,
            armor,
            damage,
        })
    }
}

impl Player {
    fn new(hit_points: u16, damage: u16, armor: u16) -> Self {
        Player {
            hit_points,
            damage,
            armor,
        }
    }
}

fn simulate_fight(player: &mut Player, boss: &mut Player) -> bool {
    loop {
        let player_damage = (player.damage.saturating_sub(boss.armor)).max(1);
        boss.hit_points = boss.hit_points.saturating_sub(player_damage);
        if boss.hit_points == 0 {
            return true;
        }

        let boss_damage = (boss.damage.saturating_sub(player.armor)).max(1);
        player.hit_points = player.hit_points.saturating_sub(boss_damage);
        if player.hit_points == 0 {
            return false;
        }
    }
}
pub fn solution() {
    let weapons = vec![
        ShopItem::new(8, 4, 0),
        ShopItem::new(10, 5, 0),
        ShopItem::new(25, 6, 0),
        ShopItem::new(40, 7, 0),
        ShopItem::new(74, 8, 0),
    ];
    let armors = vec![
        ShopItem::new(13, 0, 1),
        ShopItem::new(31, 0, 2),
        ShopItem::new(53, 0, 3),
        ShopItem::new(75, 0, 4),
        ShopItem::new(102, 0, 5),
        ShopItem::new(0, 0, 0),
    ];
    let rings = vec![
        ShopItem::new(25, 1, 0),
        ShopItem::new(50, 2, 0),
        ShopItem::new(100, 3, 0),
        ShopItem::new(20, 0, 1),
        ShopItem::new(40, 0, 2),
        ShopItem::new(80, 0, 3),
        ShopItem::new(0, 0, 0),
        ShopItem::new(0, 0, 0),
    ];

    let mut possible_purchase = Vec::new();
    weapons.iter().for_each(|weapon| {
        armors.iter().for_each(|armor| {
            (0..rings.len()).for_each(|i| {
                (i + 1..rings.len()).for_each(|j| {
                    possible_purchase.push(ShopItem::new(
                        weapon.cost + armor.cost + rings[i].cost + rings[j].cost,
                        weapon.damage + rings[i].damage + rings[j].damage,
                        armor.armor + rings[i].armor + rings[j].armor,
                    ))
                });
            });
        });
    });
    possible_purchase.sort_by(|a, b| a.cost.cmp(&b.cost));

    let mut player_stats = possible_purchase.iter();
    let mut player = Player::new(100, 0, 0);
    let mut boss = std::fs::read_to_string("data/day21.txt")
        .unwrap()
        .parse::<Player>()
        .unwrap();
    let boss_hit_points = boss.hit_points;
    let cost = loop {
        let ShopItem {
            cost,
            damage,
            armor,
        } = player_stats.next().unwrap();
        player.hit_points = 100;
        boss.hit_points = boss_hit_points;
        player.damage = *damage;
        player.armor = *armor;
        let won = simulate_fight(&mut player, &mut boss);
        if won {
            break cost;
        }
    };
    println!("I need to spend at least {cost} gold to win the fight");

    let mut player_stats = possible_purchase.iter().rev();
    let cost = loop {
        let ShopItem {
            cost,
            damage,
            armor,
        } = player_stats.next().unwrap();
        player.hit_points = 100;
        boss.hit_points = boss_hit_points;
        player.damage = *damage;
        player.armor = *armor;
        let won = simulate_fight(&mut player, &mut boss);
        if !won {
            break cost;
        }
    };
    println!("I could spend as much as {cost} gold and still lose the fight");
}
