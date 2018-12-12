//program that converts string to morse code
fn main(){
	use std::collections::HashMap;
	let mut mor_keys = HashMap::new();

	mor_keys.insert("A".to_string(),".-".to_string(),);
	mor_keys.insert("B".to_string(),"-...".to_string(),);
	mor_keys.insert("C".to_string(),"-.-.".to_string(),);
	mor_keys.insert("D".to_string(),"-..".to_string(),);
	mor_keys.insert("E".to_string(),".".to_string(),);
	mor_keys.insert("F".to_string(),"..-.".to_string(),);
	mor_keys.insert("G".to_string(),"--.".to_string(),);
	mor_keys.insert("H".to_string(),"....".to_string(),);
	mor_keys.insert("I".to_string(),"..".to_string(),);
	mor_keys.insert("J".to_string(),".---".to_string(),);
	mor_keys.insert("K".to_string(),"-.-".to_string(),);
	mor_keys.insert("L".to_string(),".-..".to_string(),);
	mor_keys.insert("M".to_string(),"--".to_string(),);
	mor_keys.insert("N".to_string(),"-.".to_string(),);
	mor_keys.insert("O".to_string(),"---".to_string(),);
	mor_keys.insert("P".to_string(),".--.".to_string(),);
	mor_keys.insert("Q".to_string(),"--.-".to_string(),);
	mor_keys.insert("R".to_string(),".-.".to_string(),);
	mor_keys.insert("S".to_string(),"...".to_string(),);
	mor_keys.insert("T".to_string(),"-".to_string(),);
	mor_keys.insert("U".to_string(),"..-".to_string(),);
	mor_keys.insert("V".to_string(),"...-".to_string(),);
	mor_keys.insert("X".to_string(),"-..-".to_string(),);
	mor_keys.insert("W".to_string(),"-.--".to_string(),);
	mor_keys.insert("Y".to_string(),"-.--".to_string(),);
	mor_keys.insert("Z".to_string(),"--..".to_string(),);
	
	let text = "HEYTHERE".to_string();
	let convertedText = "".to_string();
	
	for i in text.chars(){
		
	}

}