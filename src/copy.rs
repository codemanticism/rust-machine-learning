use std::process::Command;
use num::complex::Complex;
use rand_distr::{Distribution, Normal};
#[derive(Clone, Copy)]
struct DataPoint{
	input: f64,
	output: f64
}
struct ReturnValue{
	multiplier: f64,
	best_loss: f64,
	stopped_at_epoch: usize
}
fn calculate_loss(operation: u8, dataset: &Vec<DataPoint>, multiplier: f64) -> f64{
	let mut loss = 0 as f64;
	for element in dataset{
		loss += calculate_difference( do_operation(operation, element.input, multiplier), element.output ); 
	}
	loss / (dataset.len() as f64)
}
fn calculate_difference(x: f64, y: f64) -> f64{
	let division = x / y;
	if y == (0 as f64){
		println!("x= {} y= {}", x, y);
	}
    let z: num::complex::Complex<f64> = Complex::new(division, 0 as f64).ln() / Complex::new(2 as f64, 0 as f64).ln(); // Must use floats
	z.norm()	
}
fn fn_operation(operation: u8, a: f64, b: f64) -> f64{
	match operation {
		0 => {
			if a == (0 as f64){
				println!("a = {}, b = {}", a, b);
			}
			b / a
		},
		1 => {
			b - a
		}, _ => {
			panic!("Something went wrong");
		}
	}
}
fn do_operation(operation: u8, a: f64, b: f64) -> f64{
	match operation {
		0 => {
			a * b
		},
		1 => {
			a + b
		}, _ => {
			panic!("Something went wrong");
		}
	}
	
}
fn calibration(dataset: &Vec<DataPoint>, operation: u8, multiplier: f64, loss: Option<f64>, maximum_epochs: Option<usize>, halt_if_stagnant: bool, how_much_to_consider_stagnant: usize, variation_in_change: f64) -> ReturnValue{
	println!("Multiplier => {}", multiplier);
	let mut change = 0.5;
	let mut original_change = 0.0;
	let mut best_loss = 0.0;
	let mut best_value_for_multiplier = multiplier;
	let mut for_how_many_times_in_a_row = 0;  
	match loss{
		Some(x) => {
			best_loss = x;
		}, None => {
			best_loss = calculate_loss(operation, dataset, multiplier);
		}
	}
	let mut is_there_a_maximum_epochs = true; 
	let mut original_maximum = 0;
	match maximum_epochs {
		None => {
			is_there_a_maximum_epochs = false;
		}, Some(x) => {
			original_maximum = x;	
		}
	}
	let mut i = original_maximum;
	loop{
		println!("*");
		let mut was_lost_changed = false;
		{
			let new_multiplier = multiplier + (multiplier * change);
			let new_lost = calculate_loss(operation, &dataset, new_multiplier);
			if new_lost < best_loss {
				best_loss = new_lost;
				best_value_for_multiplier = new_multiplier;
				was_lost_changed = true;
			}
		}
		{
			let new_multiplier = multiplier - (multiplier * change);
			let new_lost = calculate_loss(operation, &dataset, new_multiplier);
			if new_lost < best_loss {
				best_loss = new_lost;
				best_value_for_multiplier = new_multiplier;
				was_lost_changed = true;
			}
		}
		{
			let new_multiplier = multiplier + (multiplier / change);
			let new_lost = calculate_loss(operation, &dataset, new_multiplier);
			if new_lost < best_loss {
				best_loss = new_lost;
				best_value_for_multiplier = new_multiplier;
				was_lost_changed = true;
			}
		}
		{
			let new_multiplier = multiplier - (multiplier / change);
			let new_lost = calculate_loss(operation, &dataset, new_multiplier);
			if new_lost < best_loss {
				best_loss = new_lost;
				best_value_for_multiplier = new_multiplier;
				was_lost_changed = true;
			}
		}
		if was_lost_changed == false{
			change = change * 2.0;
			for_how_many_times_in_a_row += 1;
			if for_how_many_times_in_a_row == how_much_to_consider_stagnant{
				if halt_if_stagnant{
					return ReturnValue{stopped_at_epoch: i, best_loss, multiplier: best_value_for_multiplier};
				}else{
					let mut rng = rand::rng();
					let normal = Normal::new(1.0, variation_in_change).unwrap();
					for j in 0..how_much_to_consider_stagnant{
						println!("j = {}", j);
						let mut return_value = ReturnValue{best_loss: 0.0, stopped_at_epoch: 0 as usize, multiplier: best_value_for_multiplier};
						let normal_sample = normal.sample(&mut rng);
						let result = do_operation(operation, best_value_for_multiplier, normal_sample);
						if is_there_a_maximum_epochs{
							return_value = calibration(dataset, operation, result, Some(calculate_loss(operation, &dataset, result)), Some(i), true);
						}else{
							return_value = calibration(dataset, operation, result, Some(calculate_loss(operation, &dataset, result)), None, true);
						}
						i = return_value.stopped_at_epoch + 1;
						if return_value.best_loss < best_loss{
							 best_value_for_multiplier = return_value.multiplier;
							 best_loss = return_value.best_loss;
						}
						if i == 0{
							return ReturnValue{stopped_at_epoch: 0, best_loss, multiplier: best_value_for_multiplier};			
						}
					}
				}
				for_how_many_times_in_a_row = 1;
			} 
		}else{
			for_how_many_times_in_a_row = 1;
		}
		println!("BEST_LOSS: {}", best_loss);
		println!("MULTIPLIER: {}", multiplier);
		println!("CHANGE: {}", change);
		let mut child = Command::new("sleep").arg("0.25").spawn().unwrap();
		let _result = child.wait().unwrap();
		if is_there_a_maximum_epochs == true{
			i -= 1;
		}
		if i == 0{
			break;
		}
	}
	ReturnValue{stopped_at_epoch: 0, best_loss, multiplier: best_value_for_multiplier}
}
fn initial_estimate(dataset: &Vec<DataPoint>, operation: u8) -> f64{
	let mut sum = 0 as f64;
	let mut vector: Vec<f64> = Vec::new();
	let mut ratio: Vec<f64> = Vec::new();
	for element in dataset{
		let division = fn_operation( operation, element.input, element.output );
		vector.push( division );
		sum += division;
	}
	let mut sum_of_losses = 0 as f64; 
	for i in 0..vector.len(){
		let division = (sum - vector[i]) / ((vector.len() - 1) as f64);
		let loss = calculate_difference( do_operation(operation, dataset[i].input, division), dataset[i].output );
		if division.is_nan() {
			println!("division == NaN");
		}
		ratio.push(loss);
		sum_of_losses += loss;
	}
	let mut result = 0 as f64;
	if sum_of_losses == (0 as f64){
		println!("sum_of_losses == 0");
		return ratio[0];
	}
	for i in 0..vector.len(){
		result += (ratio[i] / sum_of_losses) * vector[i]; 
	}	
	result
}
//pub fn run_synapse_network(synapse_network: &LinkedList<Synapse>, vector_memory: usize, number_memory: &mut [f64], ranking: Option<&mut Vec<f64>>, vector_of_ranking: Option<&mut Vec<LoopRanking>>) -> KindsOfNeurons{
fn train(synapse_network: LinkedList<Synapse>, sample: Vec<f64>){
	for synapse in synapse_network{
		let boolean = if_constant_synapse(synapse);
		match boolean {
			None => {
				
			}, Some(number) => {
				let next_synapse = synapse.next;
				let two_input_synapse = if_two_input_synapse(next_synapse);
				match boolean 
			}
		}
		match element in sample {
						
		}
	}
}
fn main() {
	let dataset: Vec<DataPoint> = vec![
		DataPoint{input: 1 as f64, output: 3 as f64},
		DataPoint{input: 2 as f64, output: 6 as f64},
		DataPoint{input: 3 as f64, output: 9 as f64},
		DataPoint{input: 4 as f64, output: 11 as f64}
	];
	let multiplier = initial_estimate(&dataset, 0);
/*
	calculate_loss(operation: u8, dataset: &Vec<DataPoint>, multiplier: f64)
*/
	let total_difference = calculate_loss(0, &dataset, multiplier);
	//let total_difference_two = calculate_loss(1, &new_dataset, new_multiplier);
	println!("{}", total_difference);
	let calibrated_difference: f64 = calibration(&dataset, 0, multiplier, Some(total_difference), Some(40), false).multiplier;
	for element in dataset{
		println!("INPUT = {}", element.input);
		println!("PREDICTION = {}", element.input * calibrated_difference);
		println!("OUTPUT = {}", element.output); 
	}
	println!("{}", multiplier);
}
