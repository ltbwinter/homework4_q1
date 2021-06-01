enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            Self::Red => 1,
            Self::Yellow => 2,
            Self::Green => 3,
        }
    }
}

fn main() {
    println!("RED: {}", TrafficLight::Red.time());
    println!("YELLOW: {}", TrafficLight::Yellow.time());
    println!("GREEN: {}", TrafficLight::Green.time());
}
