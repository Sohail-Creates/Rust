enum TrafficLight{
	Red,
	Yellow,
	Green,
}

fn signal_duration(light: TrafficLight) -> u32{
	match light{
		TrafficLight::Red => 30,
		TrafficLight::Yellow => 5,
		TrafficLight::Green => 45,
	}
}

fn main(){
	let light = TrafficLight::Yellow;
	println!("Duration: {} seconds", signal_duration(light));
}


