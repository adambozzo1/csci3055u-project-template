fn main(){
	use std::collections::VecDeque;
	let mut queue = VecDeque::new();
	queue.push_back(3);
	queue.push_back(4);
	queue.push_back(5);
	queue.push_back(10);//adds to back of queue
	queue.pop_front();//pops and returns front of queue
	queue.push_front(40);//adds to the front of the queue
	while queue.len() > 0{
		println!("{:?}", queue.pop_front());
	}
}