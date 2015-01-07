struct Monster {
    health: i32,
    strength: i32,
}

trait Attack {
    fn attack(&self, target: Monster) -> Monster;
}

impl Attack for Monster {
    fn attack(&self, target: Monster) -> Monster {
        Monster {
            health: target.health - self.strength,
            strength: target.strength
        }
    }
}

fn main() {
    let hero = Monster {
        health: 5,
        strength: 10
    };

    let enemy = Monster {
        health: 25,
        strength: 1
    };

    println!("the enemy's health is {}", enemy.health);
    let damaged_enemy = hero.attack(enemy);
    println!("the enemy's health is {}", damaged_enemy.health);
}
