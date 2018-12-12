fn main(){
	let mut vect = Vec::new();
	vect.push(40);
	vect.push(20);
	vect.push(10);
	vect[1] = 15;//replaces index 1 with value 15
	for index in &vect {
		println!("{}", index);
	}
	vect.pop();//removes top value in the vector
	
	let mut vec_cap = Vec::with_capacity(5);//can create vector with specificed size
	vec_cap.resize(10,0); //can resize this vector	
}