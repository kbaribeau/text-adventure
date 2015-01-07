struct Monster {
    health: i32,
    strength: i32,
}

trait Attack {
    fn attack(&self, target: &mut Monster);
}

impl Attack for Monster {
    fn attack(&self, target: &mut Monster) {
        target.health -= self.strength;
    }
}

fn main() {
    let hero = Monster {
        health: 5,
        strength: 10
    };

    let enemy = &mut Monster {
        health: 25,
        strength: 1
    };

    println!("the enemy's health is {}", enemy.health);
    hero.attack(enemy);
    println!("the enemy's health is {}", enemy.health);
}
