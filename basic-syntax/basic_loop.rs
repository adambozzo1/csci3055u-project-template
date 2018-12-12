fn main(){
	let number = 6;
	let mut i = 0;
	loop{
		if i == number{
			println!("Complete loop!");
			break;
		}else{
			println!("Loop still run!");
			i += 1;
		}
	}
}