fn find_index(arr: &[i32], target: i32) -> Option<usize>{
	for (index, &num) in arr.iter().enumerate(){
		if num == target{
			return Some(index);
		}
	}
	None
}

fn main(){
	let numbers = [3, 7, 2, 9];
	match find_index(&numbers, 2){
		Some(i) => println!("Found at index {}", i),
		None => println!("Not found"),
	}
}


