use PlanetaryMonster::MarsMonster;

enum Compass {
    North, South, East, West
}

type species = &'static str;

enum PlanetaryMonster {
    VenusMonster(species, i32),
    MarsMonster(species, i32)
}

fn main() {
    let direction = Compass::West;
    let martian = PlanetaryMonster::MarsMonster("Chela", 42);
    let martian = MarsMonster("Chela", 42);
}
