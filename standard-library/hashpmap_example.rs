
fn main(){
	use std::collections::HashMap;
	let mut names_grades = HashMap::new();

	names_grades.insert(
		"John".to_string(),
		54,
	);
	names_grades.insert(
		"Adam".to_string(),
		74,
	);
	names_grades.insert(
		"Jessica".to_string(),
		92,
	);
	names_grades.insert(
		"Robert".to_string(),
		41,
	);
	
	//check to see if robert exists in map
	if names_grades.contains_key("Robert"){
		println!("Robert is a student!");
	}else{
		println!("Robert is not a student!");
	}
	
	//print the map
	for (names, grades) in &names_grades {
		println!("{}: \"{}\"", names, grades);
	}
	//remove students from map
	names_grades.remove("Robert");
	names_grades.remove("Adam");
	
	//print the map
	for (names, grades) in &names_grades {
		println!("{}: \"{}\"", names, grades);
	}
}