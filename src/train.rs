use rand::prelude::*;
use rand::Rng; // 0.8.5
pub use crate::synapse_network::*;
pub static simplicity_weight = 0;
pub static accuracy_weight = 0;
enum Enum{
	StartTheSearchFromWhere([usize; usize]),
	NoSearch(usize)
}
struct DataPoint{
	input_vector: Vec<Vec<f64>>,
	input_number: Vec<f64>,
	output: Vec<f64>
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
	synapse_network: &mut LinkedList<Synapse>
}
fn check_if_not(synapse_network: &mut LinkedList<Synapse>, synapse_index: usize, output: usize) -> bool{
	match synapse_network[synapse_index]{
		Synapse::OtherSynapsesStruct(other_synapses_struct) => {
			other_synapses_struct.output != output
		}, Synapse::LoopSynapse => {
			true
		}
	}
}
/*
			let mut vector_of_indexes_of_vector_output: Vec<usize> = Vec::New();
			let mut vector_of_indexes_of_number_output: Vec<usize> = Vec::New();

*/
fn function(synapse_network: &mut LinkedList<Synapse>, start_from_where: Enum, vector_of_indexes_of_number_output: &mut Vec<usize>, vector_of_indexes_of_vector_output: &mut Vector<usize>){
	if Random<bool> {
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
		boolean_vector.starts = synapse_network_index;
	}else{
		boolean_vector.which.push(boolean_vector.starts - synapse_network_index);
	}
	if synapse_network[synapse_network_index] == {	
		vector_of_indexes_of_number_output.push(synapse_network_index);
	}else{
		vector_of_indexes_of_vector_output.push(synapse_network_index);
	}
	how_many_to_go--;
	match synapse_network[synapse_network_index] {
		Synapse::OtherSynapsesStruct(other_synapses_struct) => {match other_synapses_struct.kind {
			OtherSynapses::TwoInputSynapse(two_input_synapse) => {
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; two_input_synapse.input_1]), vector_of_indexes_of_number_output, vector_of_indexes_of_vector_output);
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; two_input_synapse.input_2]), vector_of_indexes_of_number_output, vector_of_indexes_of_vector_output);
			},
			OtherSynapses::OneInputSynapse(one_input_synapse) => {
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; one_input_synapse.input]), vector_of_indexes_of_number_output, vector_of_indexes_of_vector_output);				
			},
			OtherSynapses::RandomSynapse(_) => {
			},
			OtherSynapses::ConstantSynapse(_) => {				
			},
			OtherSynapses::ArraySynapse(array_synapse) => {
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; array_synapse.index_neuron]), vector_of_indexes_of_number_output, vector_of_indexes_of_vector_output);
				function(&synapse_network, Enum::StartTheSearchFromWhere([start_from_where; array_synapse.vector_neuron]), vector_of_indexes_of_number_output, vector_of_indexes_of_vector_output);
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
fn add_synapse(synapse_network: &mut LinkedList<Synapse>, number_of_vector_arguments: usize, number_of_number_arguments: usize, number_of_dependent_number_variables: &mut Vec<LinkedList<[usize; usize]>>, number_of_dependent_vector_variables: &mut Vec<LinkedList<[usize; usize]>>, should_it_be_vector: bool, list_of_constants_in_the_network: &mut Vector<usize>, may_it_contain_loop_synapses: bool) -> usize{
	let random = random -> x => 0 , x < 6
	if may_it_contain_loop_synapses{
		random -> x => 0 , x < 5 
	}
	if random<bool>{ 
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
							if value[0] != value[1]{
								if boolean{
									let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
									linked_list.push(value);
							 		number_of_dependent_vector_variables.push(linked_list);
								}else{
							 		number_of_dependent_vector_variables[usize_variable].push(value);
													
								}
							}
						}else{
							if value[0] != value[1]{
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
					}
					return_value = add_synapse(synapse_network, number_of_vector_arguments, number_of_number_arguments, number_of_dependent_number_variables, number_of_dependent_vector_variables, should_it_be_vector, list_of_constants_in_the_network, true);
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
					if value[0] != value[1]{
						if boolean{
							let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
							linked_list.push(value);
					 		number_of_dependent_vector_variables.push(linked_list);
						}else{
					 		number_of_dependent_vector_variables[usize_variable].push(value);						
						}
					}
				}else{
					let value = [synapse_network.len(); initial_length];
					if value[0] != value[1]{
						if boolean{
							let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
							linked_list.push(value);
					 		number_of_dependent_number_variables.push(linked_list);
						}else{
					 		number_of_dependent_number_variables[usize_variable].push(value);
						}						
					}
				}
				let return_value = add_synapse(synapse_network, number_of_vector_arguments, number_of_number_arguments, number_of_dependent_number_variables, number_of_dependent_vector_variables, should_it_be_vector, list_of_constants_in_the_network, true);
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
				synapse_network.push_to_the_start(Synapse::OtherSynapsesStruct{kind: Synapse::ArraySynapse{index_neuron: 0 as usize, vector_neuron: 0 as usize}, output: output_neuron});
				for i in 0..2{
					if i == 1{
						if should_it_be_vector{
							let value = [synapse_network.len(); initial_length];
							if value[0] != value[1]{ 
								if boolean{
									let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
									linked_list.push(value);
							 		number_of_dependent_vector_variables.push(linked_list);
								}else{
							 		number_of_dependent_vector_variables[usize_variable].push(value);
													
								}
							}
						}else{
							let value = [synapse_network.len(); initial_length];
							if value[0] != value[1]{
								if boolean{
									let mut linked_list: LinkedList<[usize; usize]> = LinkedList::New();
									linked_list.push(value);
							 		number_of_dependent_number_variables.push(linked_list);
								}else{
							 		number_of_dependent_number_variables[usize_variable].push(value);
													
								}
							}	
						}
				 	}
					let return_value = add_synapse(synapse_network, number_of_vector_arguments, number_of_number_arguments, number_of_dependent_number_variables, number_of_dependent_vector_variables, should_it_be_vector, list_of_constants_in_the_network, true);
					match synapse_network[0] {
						OtherSynapsesStruct(other_synapses_struct) => {
							match other_synapses_struct.kind {
								ArraySynapse(two_input_synapse) => {
									if i == 0{
										two_input_synapse.vector_neuron = return_value; 
									}else{
										two_input_synapse.index_neuron = return_value;
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
				list_of_constants_in_the_network.push(synapse_network.len());
				synapse_network.push_to_the_start(Synapse::OtherSynapsesStruct{kind: Synapse::ConstantSynapse{}, output: output_neuron});			
			}, _ => { //LoopSynapse
//fn add_synapse(synapse_network: &mut LinkedList<Synapse>, number_of_vector_arguments: usize, number_of_number_arguments: usize, number_of_dependent_number_variables: &mut Vec<LinkedList<[usize; usize]>>, number_of_dependent_vector_variables: &mut Vec<LinkedList<[usize; usize]>>, should_it_be_vector: bool, list_of_constants_in_the_network: Option<&mut Vector<usize>>, may_it_contain_loop_synapses: bool) -> usize{
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
fn add_synapse(synapse_network: &mut LinkedList<Synapse>, number_of_vector_arguments: usize, number_of_number_arguments: usize, number_of_dependent_number_variables: &mut Vec<LinkedList<[usize; usize]>>, number_of_dependent_vector_variables: &mut Vec<LinkedList<[usize; usize]>>, should_it_be_vector: bool, list_of_constants_in_the_network: Option<&mut Vector<usize>>, may_it_contain_loop_synapses: bool) -> usize{

*/

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
				let mut synapse_network_1 = Vec::New();
				let mut synapse_network_2 = Vec::New();
				let mut synapse_network_3 = Vec::New();
				add_synapse(synapse_network_1, 1, 0, &mut Vec::New(), &mut Vec::New(), true, &mut Vec::New(), false);
				add_synapse(synapse_network_2, 1, 0, &mut Vec::New(), &mut Vec::New(), false, &mut Vec::New(), false);
				add_synapse(synapse_network_3, number_of_vector_arguments, number_of_number_arguments, number_of_dependent_number_variables, number_of_dependent_vector_variables, should_it_be_vector, list_of_constants_in_the_network, false);
				synapse_network_3.push(Synapse::OtherSynapsesStruct{kind: OtherSynapses::Synapse{input_1: output_neuron, input_2: synapse_network_3.last().output, operation: type_of_two_input_synapse}, output: output_neuron});
				let synapse: Synapse = Synapse::LoopSynapse{inner_loop: synapse_network_3, loop_variable_starts_at: vec![0 as f64], at_the_end_of_the_loop: synapse_network_1, check_if_it_should_exit_the_loop: synapse_network_2};
				
			}
		}
	}else{
		if should_it_be_vector{
			RANDOM( 0, number_of_vector_arguments ) * 2
		}else{
			(RANDOM( 0, number_of_number_arguments ) * 2) + 1
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
fn calculate_the_score(synapse_network: &mut LinkedList<Synapse>, dataset: Vec<DataPoint>, how_many_randomly_selected_ones: usize, not_always_same_number_of_number_arguments: bool, not_always_same_number_of_vector_arguments: bool) -> f64{
	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	list_of_vector_memories.push(Vec::New());
	let mut number_memory: [f64] = Vec::New();
	let mut ranking = Vec::New();
	let mut vector_of_ranking = Vec::New();
	let mut difference = 0;
	for i in 0..how_many_randomly_selected_ones{
		let random_index = /*TODO: Implement random number between 0 and the length of dataset*/;
		let mut number_memory_index = 0;
		if not_always_same_number_of_number_arguments  {
			number_memory[0] = dataset[random_index].input_number.len();
			number_memory_index += 1;	
		}
		if not_always_same_number_of_vector_arguments  {
			number_memory[number_memory_index] = dataset[random_index].input_vector.len();
			number_memory_index += 1;	
		}
		for j in 0..dataset[random_index].input_number.len() {
			number_memory[number_memory_index] = dataset[random_index].input_number[j];
			number_memory_index += 1;
		}
		for j in 0..dataset[random_index].input_vector.len() {
			vector_memory[j] = dataset[random_index].input_vector[j];
		}
		let output = run_synapse_network(synapse_network, list_of_vector_memories.len() - 1, &mut number_memory, ranking, vector_of_ranking);
		assert!(dataset[random_index].output.len() == output.len()); //TODO: look up the difference between assert_eq and assert
		for j in 0..output.len(){
			difference += abs( log2(output[j] / dataset[random_index].output[j]) ) ; //TODO:
		} 
	}
	difference / how_many_randomly_selected_ones
}
fn train(minimum_score: u32, synapse_network: &mut LinkedList<Synapse>, dataset: Vec<DataPoint>, how_many_vector_arguments: usize, how_many_number_arguments: usize, how_many_randomly_selected_ones: usize){
	let mut variables = Vec<bool>;
	while calculate_score() < minimum_score{
		let mut modifications: Vec<LinkedList<Synapse>> = Vec::New();
		let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
		list_of_vector_memories.push(Vec::New());
		mem::drop(list_of_vector_memories);
		let mut ranking = Vec::New();
		let mut vector_of_rankings = Vec::New();
		run_synapse_network(synapse_network, list_of_vector_memories, number_memory: &mut [f64], Some<&mut vector1>, Some<&mut vector2>)				 
		loop{
			let constant = 4;
			let mut most_influential_synapses = [[usize; usize]; constant];
			for i in 0..ranking.len(){
				for j in 0..constant{
					if ranking[i] > most_influential_synapses[j]{
						for k in 1..(j + 1){
							most_influential_synapses[k-1] = ranking[k];
						}
						ranking[i] = most_influential_synapses[j];
						break;
					}
				}
			}
			for i1 in 0..vector_of_ranking.len(){
				for i2 in 0..vector_of_ranking[i1].ranking.len(){
					for j in 0..constant{
						if vector_of_ranking[i1].ranking[i2] > most_influential_synapses[j]{
							for k in 1..(j + 1){
								most_influential_synapses = vector_of_ranking[i1].ranking[k];
							}
							vector_of_ranking[i1].ranking[i2] = most_influential_synapses[j];
							break;
						}
					}
				}
			}
			let mut which_among_the_most_influential = 0 as usize;
			for count in 1..constant{
				if random<bool>{
					break;
				}else{
					which_among_the_most_influential += 1;
				}
			}
			modifications.push(LinkedList::New());
			let mut vector_of_indexes_of_vector_output: Vec<usize> = Vec::New();
			let mut vector_of_indexes_of_number_output: Vec<usize> = Vec::New();
			let mut list_of_constants_in_the_network = Vec::New();
			function(synapse_network, NoSearch(), &mut vector_of_usize);
//fn add_synapse(synapse_network: &mut LinkedList<Synapse>, number_of_vector_arguments: usize, number_of_number_arguments: usize, number_of_dependent_number_variables: &mut Vec<LinkedList<[usize; usize]>>, number_of_dependent_vector_variables: &mut Vec<LinkedList<[usize; usize]>>, should_it_be_vector: bool, list_of_constants_in_the_network: &mut Vector<usize>, may_it_contain_loop_synapses: bool) -> usize{
			add_synapse(LinkedList.last(), how_many_vector_arguments, how_many_number_arguments, Vec::New(), Vec::New(), true, list_of_constants_in_the_network, true);
			for constant_index in list_of_constants_in_the_network{
				LinkedList.last()[constant_index].constant = 1;
				let score_1 = calculate_the_score(synapse_network: &mut LinkedList<Synapse>, dataset: Vec<DataPoint>, how_many_randomly_selected_ones: usize, not_always_same_number_of_number_arguments: bool, not_always_same_number_of_vector_arguments: bool);				
				LinkedList.last()[constant_index].constant = 0.1;
				let score_2 = calculate_the_score(synapse_network: &mut LinkedList<Synapse>, dataset: Vec<DataPoint>, how_many_randomly_selected_ones: usize, not_always_same_number_of_number_arguments: bool, not_always_same_number_of_vector_arguments: bool)				
				if difference between score_1 - score_2 <= certain_value { //TODO
					
				}else if score_1 > score_2 {
					let mut what_is_part_of_the = 0;
					loop{
						if implement random between 1 and N{ //TODO 
							LinkedLast.last()[constant_index].constant *= 2;
							let new_score = calculate_the_score(synapse_network: &mut LinkedList<Synapse>, dataset: Vec<DataPoint>, how_many_randomly_selected_ones: usize, not_always_same_number_of_number_arguments: bool, not_always_same_number_of_vector_arguments: bool);				
						}else{
							break
						}
					}
					
				}else{
					let mut what_is_part_of_the = 0;
					loop{
						if implement random between 1 and N { //TODO 
							LinkedLast.last()[constant_index].constant /= 2;
							let new_score = calculate_the_score(synapse_network: &mut LinkedList<Synapse>, dataset: Vec<DataPoint>, how_many_randomly_selected_ones: usize, not_always_same_number_of_number_arguments: bool, not_always_same_number_of_vector_arguments: bool);				
						}else{
							break
						}
					}
					
				}
			}				
			if Random<Bool>{
				break;
			}	
		}		
	}
}
