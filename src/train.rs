use rand::prelude::*;
use rand::Rng; // 0.8.5
pub use crate::synapse_network::*;
pub static simplicity_weight = 0;
pub static accuracy_weight = 0;
enum Enum{
	StartTheSearchFromWhere([usize; usize]),
	NoSearch(usize)
}
struct ArrayNode{
}
enum Node{
	TwoInputSynapseTypes(TwoInputSynapseTypes),
	OneInputSynapseTypes(OneInputSynapseTypes),
	RandomSynapse(RandomSynapse),
	ArrayNode(ArrayNode),
	ConstantNode(usize)
}
struct BooleanStruct{
	which: &mut Vec<usize>,
	starts: usize 
}
struct SynapseID{
	synapse_index: usize,
	synapse_network: &mut [Synapse]
}
fn check_if_not(synapse_network: &mut [Synapse], synapse_index: usize, output: usize) -> bool{
	match synapse_network[synapse_index]{
		Synapse::OtherSynapsesStruct(other_synapses_struct) => {
			other_synapses_struct.output != output
		}, Synapse::LoopSynapse => {
			true
		}
	}
}
fn 
fn function(synapse_network: &mut [Synapse], start_from_where: Enum, how_many_to_go: mut u32, boolean_struct: &mut BooleanStruct){
	if(how_many_to_go == 0){
		return;
	}
	let mut synapse_network_index: usize = start_from_where;
	match start_from_where {
		Enum::StartTheSearchFromWhere(start_from_where) => {
			let output = start_from_where[1];
			let start_from_where_usize = start_from_where[0];
			while check_if_not(synapse_network, synapse_network_index, output){
				if(synapse_network_index == 0){
					return;				
				}
				synapse_network_index--;
			}
		}, Enum::NoSearch(start_from_where_usize) => {
			synapse_network_index = start_from_where_usize; 
		} 
	}
	if boolean_vector.which.len() == 0 {
		boolean_struct.starts = synapse_network_index;
	}else{
		boolean_struct.which.push(boolean_struct.starts - synapse_network_index);
	}
	vector_of_bools.push(synapse_network_index);
	how_many_to_go--;
	match synapse_network[synapse_network_index] {
		Synapse::OtherSynapsesStruct(other_synapses_struct) => {match other_synapses_struct.kind {
			OtherSynapses::TwoInputSynapse(two_input_synapse) => {
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; two_input_synapse.input_1]), how_many_to_go, vector_of_bools);
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; two_input_synapse.input_2]), how_many_to_go, vector_of_bools);
			},
			OtherSynapses::OneInputSynapse(one_input_synapse) => {
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; one_input_synapse.input]), how_many_to_go, vector_of_bools);				
			},
			OtherSynapses::RandomSynapse(_) => {
			},
			OtherSynapses::ConstantSynapse(_) => {				
			},
			OtherSynapses::ArraySynapse(array_synapse) => {
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; array_synapse.index_neuron]), how_many_to_go, vector_of_bools);
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; array_synapse.vector_neuron]), how_many_to_go, vector_of_bools);
			},			
		}}, Synapse::LoopSynapse(_) => {
		}
	}
}
fn calculate_score(simplicity: u32, acurracy: u32){
	let simplicity_part: f64 = (simplicity_weight * simplicity) / u32::MAX;
	let accuracy_part: f64 = (accuracy_weight * accuracy) / u32::MAX;
}
fn generate_random_number() -> u32{
	let mut rng = rand::rng();
	let mut random = rng.random::<u32>();
	let mut i = 1;
	while (random % 2) == 1{
		i += 1;
		random = random / 2;
	}
	i
}
/*
pub struct LoopSynapse<'a, 'b>{
	pub loop_variable_starts_at: Vec<f64>,  
	pub at_the_end_of_the_loop: FunctionCall<'a>,
	pub check_if_it_should_exit_the_loop: &'b [Synapse],
	pub end_of_the_loop: usize,    
}
pub struct FunctionCall<'a>{
	pub synapse_network: &'a [Synapse],
	pub arguments: Vec<u32>,	
}

*/
fn add_synapse(synapse_network: &mut LinkedList<Synapse>, number_of_vector_arguments: usize, number_of_number_arguments: usize, number_of_dependent_number_variables: &mut Vec<LinkedList<[usize; usize]>>, number_of_dependent_vector_variables: &mut Vec<LinkedList<[usize; usize]>>, should_it_be_vector: bool, where_constants_: Option<&mut Vector<usize>>) -> usize{
	random -> x => 0 , x < 5 
	match random{
		0 => { //TwoInputSynapseTypes
			let type_of_two_input_synapse: TwoInputSynapseTypes = random_one;
			let mut output_neuron: usize = 0 as usize;
			let mut usize_variable = 0;
			let mut boolean: bool = false;
			match should_it_be_vector{
				false => { outer_loop: for i in 0..(number_of_dependent_number_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_number_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_number_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_number_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = (number_of_number_arguments + i) * 2;
							usize_variable = i;	 		
							break outer_loop;
						}  
					}
				}
				output_neuron = (number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2;
				usize_variable = number_of_dependent_number_variables.len();
				boolean = true;
				},
				true => { outer_loop: for i in 0..(number_of_dependent_vector_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_vector_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_vector_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_vector_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = ((number_of_number_arguments + i) * 2) + 1;	 		
							usize_variable = i;
							break outer_loop;
						}  
					}  
				}
				output_neuron = ((number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2) + 1;
				usize_variable = number_of_dependent_vector_variables.len();
				boolean = true;
				}			
			}
			let initial_length = synapse_network.len();
			synapse_network.push_to_the_start(Synapse::OtherSynapsesStruct{kind: Synapse::TwoInputSynapse{input_1: 0 as usize, input_2: 0 as usize}, output: output_neuron});
			for i in 0..2{
				if i == 1{
					if should_it_be_vector{
						let value = [synapse_network.len(); initial_length];
						if boolean{
							let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
							linked_list.push(value);
					 		number_of_dependent_vector_variables.push(linked_list);
						}else{
					 		number_of_dependent_vector_variables[usize_variable].push(value);
											
						}
					}else{
						let value = [synapse_network.len(); initial_length];
						if boolean{
							let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
							linked_list.push(value);
					 		number_of_dependent_number_variables.push(linked_list);
						}else{
					 		number_of_dependent_number_variables[usize_variable].push(value);
											
						}						
					}
			 	}
				let return_value = add_synapse(synapse_network, number_of_vector_arguments, number_of_number_arguments, number_of_dependent_number_variables, number_of_dependent_vector_variables, should_it_be_vector);
				match synapse_network[0] {
					OtherSynapsesStruct(other_synapses_struct) => {
						match other_synapses_struct.kind {
							TwoInputSynapse(two_input_synapse) => {
								if i == 0{
									two_input_synapse.input_1 = return_value; 
								}else{
									two_input_synapse.input_2 = return_value;
								}
								;
							},
							_ => {
								error!("something went wrong");
							} 
						}
					}
					, _ => {
						error!("something went wrong");
					}
				}
			}
		},
		1 => { //OneInputSynapseTypes
			let type_of_two_input_synapse: OneInputSynapseTypes = random_one;
			let mut output_neuron: usize = 0 as usize;
			let mut usize_variable = 0;
			let mut boolean: bool = false;
			match should_it_be_vector{
				false => { outer_loop: for i in 0..(number_of_dependent_number_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_number_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_number_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_number_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = (number_of_number_arguments + i) * 2;
							usize_variable = i;	 		
							break outer_loop;
						}  
					}
				}
				output_neuron = (number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2;
				usize_variable = number_of_dependent_number_variables.len();
				boolean = true;
				},
				true => { outer_loop: for i in 0..(number_of_dependent_vector_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_vector_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_vector_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_vector_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = ((number_of_number_arguments + i) * 2) + 1;	 		
							usize_variable = i;
							break outer_loop;
						}  
					}  
				}
				output_neuron = ((number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2) + 1;
				usize_variable = number_of_dependent_vector_variables.len();
				boolean = true;
				}			
			}
			let initial_length = synapse_network.len();
			synapse_network.push_to_the_start(Synapse::OtherSynapsesStruct{kind: Synapse::OneInputSynapse{input: 0 as usize}, output: output_neuron});
			if should_it_be_vector{
				let value = [synapse_network.len(); initial_length];
				if boolean{
					let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
					linked_list.push(value);
			 		number_of_dependent_vector_variables.push(linked_list);
				}else{
			 		number_of_dependent_vector_variables[usize_variable].push(value);
									
				}
			}else{
				let value = [synapse_network.len(); initial_length];
				if boolean{
					let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
					linked_list.push(value);
			 		number_of_dependent_number_variables.push(linked_list);
				}else{
			 		number_of_dependent_number_variables[usize_variable].push(value);
									
				}						
			}
			let return_value = add_synapse(synapse_network, number_of_vector_arguments, number_of_number_arguments, number_of_dependent_number_variables, number_of_dependent_vector_variables, should_it_be_vector);
			match synapse_network[0] {
				OtherSynapsesStruct(other_synapses_struct) => {
					match other_synapses_struct.kind {
						OneInputSynapse(two_input_synapse) => {
							two_input_synapse.input = return_value; 
						},
						_ => {
							error!("something went wrong");
						} 
					}
				}
				, _ => {
					error!("something went wrong");
				}
			}
			}
		}, 
		2 => { //RandomSypnase
			let mut output_neuron: usize = 0 as usize;
			let mut usize_variable = 0;
			let mut boolean: bool = false;
			match should_it_be_vector{
				false => { outer_loop: for i in 0..(number_of_dependent_number_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_number_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_number_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_number_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = (number_of_number_arguments + i) * 2;
							usize_variable = i;	 		
							break outer_loop;
						}  
					}
				}
				output_neuron = (number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2;
				usize_variable = number_of_dependent_number_variables.len();
				boolean = true;
				},
				true => { outer_loop: for i in 0..(number_of_dependent_vector_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_vector_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_vector_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_vector_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = ((number_of_number_arguments + i) * 2) + 1;	 		
							usize_variable = i;
							break outer_loop;
						}  
					}  
				}
				output_neuron = ((number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2) + 1;
				usize_variable = number_of_dependent_vector_variables.len();
				boolean = true;
				}			
			}
			synapse_network.push_to_the_start(Synapse::OtherSynapsesStruct{kind: Synapse::RandomSynapse{}, output: output_neuron});
						
		},
		3 => { //ArraySynapse
			
		},
		4 => { //ConstantSynapse
			let mut output_neuron: usize = 0 as usize;
			let mut usize_variable = 0;
			let mut boolean: bool = false;
			match should_it_be_vector{
				false => { outer_loop: for i in 0..(number_of_dependent_number_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_number_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_number_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_number_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = (number_of_number_arguments + i) * 2;
							usize_variable = i;	 		
							break outer_loop;
						}  
					}
				}
				output_neuron = (number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2;
				usize_variable = number_of_dependent_number_variables.len();
				boolean = true;
				},
				true => { outer_loop: for i in 0..(number_of_dependent_vector_variables.len()){
					let mut condition = true;
					for j in 0..(number_of_dependent_vector_variables[i].len()){
						if ( (synapse_network.len() - 1)) < number_of_dependent_vector_variables[i][j][0] {
							if ( (synapse_network.len() - 1)) > number_of_dependent_vector_variables[i][j][1] {
								condition = false;			
							}  							
						}
						if condition{
							output_neuron = ((number_of_number_arguments + i) * 2) + 1;	 		
							usize_variable = i;
							break outer_loop;
						}  
					}  
				}
				output_neuron = ((number_of_number_arguments +  number_of_dependent_vector_variables.len()) * 2) + 1;
				usize_variable = number_of_dependent_vector_variables.len();
				boolean = true;
				}			
			}
			synapse_network.push_to_the_start(Synapse::OtherSynapsesStruct{kind: Synapse::ConstantSynapse{}, output: output_neuron});			
		}
	}
}
/*
	+ log 2(x)
	- log 2(x)
	
	TwoInputSynapse
	OneInputSynapse
	ArraySynapse
	ConstantSynapse
	RandomSynapse
*/
fn train(minimum_score: u32, synapse_network: &mut [Synapse]){
	let mut variables = Vec<bool>;
	while calculate_score() < minimum_score{
		for i in 0..generate_random_number{
			let mut synapse_tree: Vec<Node> = Vec::New();
			add_node_to_operation_tree(&mut synapse_tree, 0);
			try();
		}		
		if score_better{
			apply_difference();
		}						

	}
}
