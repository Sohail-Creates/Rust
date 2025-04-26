fn check_number(num: i32){
	if num > 0{
		println!("Positive");
	} else if num < 0{
		println!("Negative");
	} else{
		println!("Zero");
	}
}

fn main (){
	check_number(-5);
}


