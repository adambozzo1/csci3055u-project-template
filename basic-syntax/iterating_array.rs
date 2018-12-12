fn main(){
	let array = [2,3,4,5,10];
	
	//iterating array with for loop
	for element in array.iter(){
	   println! ("value of array is: {}", element);
	}
	
	//iterating array with while loop
	let mut array_index = 0;
	while array_index < array.len() {
	    println!("Array value is: {}", array[array_index]);
		array_index += 1;
	}
}