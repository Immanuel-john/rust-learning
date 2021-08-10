enum Action{
    Drive,
    Turn(Direction),
    Pickup,
    Stop
}

enum Direction{
    Left,
    Right
}

fn print_action(a: Action){
    match a{
        Action::Drive => {
            println!("Drive forward");
        }
        Action::Turn(direction) => {
            match direction {
                Direction::Left => {
                    println!("Turn left");
                }
                Direction::Right => {
                    println!("Turn right");
                }
            }
        }
        Action::Pickup => {
            println!("Pickup ");
        }
        Action::Stop => {
            println!("Stop");
        }
    }
}

fn main() {
    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Turn(Direction::Right),
        Action::Pickup,
        Action::Stop
    ];

    for action in program{
        print_action(action);
    }
}
