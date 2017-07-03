

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// This struct represents a command to control a GameObject.
struct MovementCommand {
    direction: Direction 
}

/// This struct represent a Room in the game.
#[derive(Debug)]
struct Room {
    neighbors: &[&Room],
}

#[derive(Debug, Display)]
struct Player {
    location: Room,
    name: &str,
}

trait Movable {
    fn should_move() -> bool;
    fn move(&self);
}


trait GameObject {
    fn update(&self, delta: f64);
    //fn render(&self);
}


impl GameObject for Player {
    fn update(&self, delta: f64) {
        if (self.should_move()) {
            self.move();
        } 
    }
}

impl Movable for Player {
     fn move(&self) {
         if self.should_move() {
             let cmd = self.get_next_command();
         }
     }
}

struct World {
    players: []Player
}