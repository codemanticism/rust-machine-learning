extern crate rand;
use rand::prelude::*;
use rand::Rng; // 0.8.5
pub use crate::synapse_network::*;
pub static simplicity_weight = 0;
pub static accuracy_weight = 0;
pub enum Enum{
	StartTheSearchFromWhere([usize; usize]),
	NoSearch(usize)
}
pub struct UsedMemoryCells{
	pub exceptions_vector_memory: Vec<usize>,
	pub lowest_unused_vector_memory: usize,
	pub exceptions_number_memory: Vec<usize>,
	pub lowest_unused_number_memory: usize,
	pub exceptions_cardinal_memory: Vec<usize>,
	pub lowest_unused_cardinal_memory: usize
}
pub struct SynapseNetwork{
	pub synapse_network: NodeList<Synapse>,
	pub length: usize,
	pub how_many_vector_arguments: Option<usize>,
	pub how_many_number_arguments: Option<usize>,
}
pub struct DataPoint{
	pub input_vector: Vec<Vec<f64>>,
	pub input_number: Vec<f64>,
	pub output: Vec<f64>
}
pub struct ArrayNode{
}
pub enum Node{
	TwoInputSynapseTypes(TwoInputSynapseTypes),
	OneInputSynapseTypes(OneInputSynapseTypes),
	RandomSynapse(RandomSynapse),
	ArrayNode(ArrayNode),
	ConstantNode(usize)
}
pub struct BooleanStruct{
	pub which: &mut Vec<usize>,
	pub starts: usize 
}
pub struct SynapseID{
	pub synapse_index: usize,
	pub synapse_network: &mut NodeList<Synapse>
}
pub fn check_if_not(synapse_network: &mut NodeList<Synapse>, synapse_index: usize, output: usize) -> bool{
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
pub fn error(error_code: u32){
	panic!("Error code {}", error_code);
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
pub fn maybe_add_synapse(used_memory_cells: &mut UsedMemoryCells, synapse_network: &mut NodeList<Synapse>, output_should_be: usize, in_loop: bool, cannot_be_constant: bool, arguments: &mut Vec<usize>, called_from: Option<TwoInputSynapseTypes>, it_doesnt_need_that: bool) -> Option<usize>{
	let mut rng = rand::rng();
	let random_bool = rng.random::<bool>();
	if random_bool && it_doesnt_need_that{
		None
	}
	Some(add_synapse(used_memory_cells, synapse_network, output_should_be, in_loop, cannot_be_constant, arguments, called_from))
}
pub fn add_synapse(used_memory_cells: &mut UsedMemoryCells, synapse_network: &mut NodeList<Synapse>, output_should_be: usize, in_loop: bool, cannot_be_constant: bool, arguments: &mut Vec<usize>, called_from: Option<TwoInputSynapseTypes>) -> usize{
	let mut rng = rand::thread_rng();
	let mut random_number: Option<f64> = None;
	if in_loop{
		if cannot_be_constant {	
			random_number = rng.gen_range(0..4);
			if random_number == 3 {
				random_number += 1;
			}
		}else{
			random_number = rng.gen_range(0..5);
		}
	}else{
		if cannot_be_constant {
			random_number = rng.gen_range(0..3);
		}else{
			random_number = rng.gen_range(0..4);
		}
	}
	let mut synapse: Option<Synapse> = None; 
	match random_number{
	/*
		TwoInputSynapse
		OneInputSynapse
		- ArraySynapse
		- WriteSynapse
		RandomSynapse
		C ConstantSynapse

		ForwardSynapse
		BackwardsSynapse
	*/
		0 => { //TwoInputSynapse
			let input_1 = 0;
			let input_2 = 0;
			let mut rng_2 = rand::rng();
			let random_operation = rng.random::<TwoInputSynapseTypes>();
			let it_doesnt_need_to = arguments.length != 0;
			match maybe_add_synapse(used_memory_cells, synapse_network, output_should_be, in_loop, cannot_be_constant, arguments, Some(random_operation), arguments.len() > 0) {
				None => {
					input_1 = arguments.last();
					arguments.pop();			
				}, Some(x) => {
					input_1 = x;
				}	
			}
			let it_doesnt_need_to_2 = arguments.length != 0;
			match maybe_add_synapse(used_memory_cells, synapse_network, output_should_be, in_loop, cannot_be_constant, arguments, Some(random_operation), arguments.len() > 0) {
				None => {
					input_1 = arguments.last();
					arguments.pop();			
				}, Some(x) => {
					input_1 = x;
				}	
			}
			synapse = Some(  Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: OtherSynapses::TwoInputSynapse( TwoInputSynapse{operation: random_operation, input_1, input_2} ), output: output_should_be ) );
		}, 1 => { //OneInputSynapse
			let input = 0;
			let random_operation = rng.random::<OneInputSynapseTypes>();
			let it_doesnt_need_to = arguments.length != 0;
			match maybe_add_synapse(synapse_network: &mut NodeList<Synapse>, output_should_be: usize, in_loop: bool, true, arguments, None, it_doesnt_need_to) {
				None => {
					input = arguments.last();
					arguments.pop();
				}, Some(x) => {
					input = x;
				}	
			}	
			synapse = Some(  Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: OtherSynapses::OneInputSynapse( OneInputSynapse{operation: random_operation, input} ), output: output_should_be ) );
		}, 2 => { //RandomSynapse
			synapse = Some( Synapse::OtherSynapsesStruct{
				OtherSynapsesStruct{
					kind: RandomSynapse{},
					output: output_should_be
				}
			} );
		}, 3 => { //ConstantSynapse
			match called_from{
				Some(x) => {
					match x{
						Add => {
							synapse = Some( Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: ConstantSynapse{constant: 0}, output: output_should_be ) );
						},
						Multiply => {
							synapse = Some( Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: ConstantSynapse{constant: 1}, output: output_should_be ) );
						},
						Divide => {
							synapse = Some( Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: ConstantSynapse{constant: 1}, output: output_should_be ) );
						},
						EqualTo => {
							synapse = Some( Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: ConstantSynapse{constant: 1}, output: output_should_be ) );
						}						 
					}				
				}, None => {
					synapse = Some( Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: ConstantSynapse{constant: 1}, output: output_should_be ) );
				}
			}
		}, 4 => { //ForwardSynapse
			let mut cardinal_neuron: Option<usize> = None; 
			if used_memory_cells.cardinal_memory_exceptions.len() > 0{
				cardinal_neuron = Some(used_memory_cells.cardinal_memory_exceptions.last());
				used_memory_cells.cardinal_memory_exceptions.pop();	
			}else{
				cardinal_neuron = Some(used_memory_cells.lowest_unused_cardinal_memory);
				used_memory_cells.lowest_unused_cardinal_memory++;
			}
			variable.unwrap();
			let synapse_2 = Synapse::PurelyCardinalSynapse( PurelyCardinalSynapse{kind: ForwardSynapse, cardinal_neuron: cardinal_neuron} );
			synapse_network.len++;
			(*synapse_network.start).prev = Some(Box<synapse_2>);
			*synapse_network.start = (*synapse_network.start).prev;
			let mut rng = rand::rng();
			let random_bool = rng.random::<bool>();
			if (output_should_be % 2) == 1{
				let mut vector_neuron: Option<usize> = None;
				if random_bool && (arguments.len() != 0){
					vector_neuron = Some(arguments.last());
					arguments.pop();
				}else{
					if used_memory_cells.vector_memory_excetpions.len() > 0{
						vector_neuron = Some(used_memory_cells.vector_memory_exceptions.last());
						used_memory_cells.vector_memory_exceptions.pop();	
					}else{
						vector_neuron = Some(used_memory_cells.lowest_unused_vector_memory);
						used_memory_cells.lowest_unused_number_memory++;
					}
					add_synapse(used_memory_cells, vector_of_references, synapse_network, vector_neuron, in_loop, cannot_be_constant, arguments, called_from);						
 				}			
				synapse = Some(Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: OtherSynapses::ArraySynapse( ArraySynapse{vector_neuron: vector_neuron, cardinal_neuron: cardinal_neuron} ), output: output_should_be} );			
			}else{
				let mut number_neuron: Option<usize> = None;
				if random_bool && (arguments.len() != 0){
					number_neuron = Some(arguments.last());
					arguments.pop();
				}else{
					if used_memory_cells.number_memory_excetpions.len() > 0{
						number_neuron = Some(used_memory_cells.number_memory_exceptions.last());
						used_memory_cells.number_memory_exceptions.pop();	
					}else{
						number_neuron = Some(used_memory_cells.lowest_unused_number_memory);
						used_memory_cells.lowest_unused_number_memory++;
					}
					add_synapse(used_memory_cells, vector_of_references, synapse_network, number_neuron, in_loop, cannot_be_constant, arguments, called_from);
				}
				synapse = Some(Synapse::OtherSynapsesStruct( OtherSynapsesStruct{ kind: OtherSynapses::WriteSynapse( WriteSynapse{number_neuron: number_neuron, cardinal_neuron: cardinal_neuron} ), output: output_should_be} );
			}
		}, _ => {
					
		}
/*
	pub kind: PurelyCardinalSynapseKind,
	pub cardinal_neuron: usize

*/
	}
	synapse_network.len++;
	(*synapse_network.start).prev = Some(Box<synapse.unwrap()>);
	*synapse_network.start = (*synapse_network.start).prev; 
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
pub fn calculate_the_score(synapse_network: &mut NodeList<Synapse>, dataset: Vec<DataPoint>, how_many_randomly_selected_ones: usize, not_always_same_number_of_number_arguments: bool, not_always_same_number_of_vector_arguments: bool, patches: Vec<Patch>) -> f64{
	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	list_of_vector_memories.push(Vec::new());
	let mut number_memory: [f64] = Vec::new();
	let mut ranking = Vec::new();
	let mut vector_of_ranking = Vec::new();
	let mut difference = 0;
	for i in 0..how_many_randomly_selected_ones{
		let mut rng = rand::thread_rng();
		let random_index = rng.gen_range(0..dataset.len()) as usize;
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
		assert_eq!(dataset[random_index].output.len(), output.len());
		for j in 0..output.len(){
			difference += abs( log2(output[j] / dataset[random_index].output[j]) ) ; 
		} 
	}
	difference / how_many_randomly_selected_ones
}
pub fn find_last_synapse_that_has_output_x_before_y(x: u32, y: Node<Synapse>) -> Option<Node<Synapse>>{
	let mut position = y;
	loop{	
		position = position.prev;
		if if_not_loop_synapse( position ) == x{
			return Some(Node);
		}else if position.prev == None{
			break;
		}
	}
	return None;
}
pub fn find_last_cardinal_synapse_that_has_input_x_before_y(x: u32, y: Node<Synapse>) -> Option<Node<Synapse>>{
	let mut position = y;
	loop{	
		position = position.prev;
		if if_not_loop_synapse( position ) == x{
			return Some(Node);
		}else if position.prev == None{
			break;
		}
	}
	return None;
}
pub fn find_last_synapse_that_has_output_x_before_y(x: u32, y: Node<Synapse>, z: Node<Bool>) -> Option<[Node<Synapse>; Node<Bool>]>{
	let mut position = y;
	let mut bool_node = z;
	loop{	
		position = position.prev;
		z = z.prev;
		if if_not_loop_synapse( position ) == x{
			return Some(Node);
		}else if position.prev == None{
			break;
		}
	}
	return None;
}
pub fn find_last_cardinal_synapse_that_has_input_x_before_y(x: u32, y: Node<Synapse>, z: Node<Bool>) -> Option<[Node<Synapse>; Node<Bool>]>{
	let mut position = y;
	let mut bool_node = z;
	loop{	
		position = position.prev;
		z = z.prev;
		if if_not_loop_synapse( position ) == x{
			return Some(Node);
		}else if position.prev == None{
			break;
		}
	}
	return None;
}
pub fn search(node: Option<Box<Node<Synapse>>>, maximum: usize, counter: &mut usize, list_of_arguments: &mut NodeList<Option<Box<Node<Synapse>>>>){
	let mut rng = rand::rng();
	let random_bool = rng.random::<bool>();
	if random_bool{
		*(list_of_arguments.end).next = Some(Box<Node<Synapse{value: node, prev: list_of_arguments.end, next: None);
		list_of_arguments.end = *(list_of_arguments.end).next;
		break;
	}
	counter += 1;
	match node.value{
		OtherSynapsesStruct(other_synapses_struct) => {
			match other_synapses_struct{
				TwoInputSynapse(two_input_synapse) => {
					let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input_1, node);
					let find2 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input_2, node);
					let mut boolean = true;
					if (find1 != None) && (find2 != None) {
						if (maximum - counter) < 2 {
							boolean = false;
						}
					}else if ((find1 != None) && (find2 == None)) || ((find1 == None) && (find2 != None)) {
						if (maximum - counter) < 1 {
							boolean = false;
						}						
					}
					if boolean{
						match find1{
							None => {
						
							}, Some(x) => {
								search(x, maximxum, counter, list_of_arguments);
							}						
						}
						match find2{
							None => {
								
							}, Some(x) => {
								search(x, maximxum, counter, list_of_arguments);							
							}
						}
					}
				}, OneInputSynapse(one_input_synapse) => {
					let find = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input, node);
					match find{
						None => {
							
						}, Some(x) => {
							if (maximum - counter) >= 1{
								search(x, maximxum, counter, list_of_arguments);
							}
						}						
					}				
				}, WriteSynapse(write_synapse) => {
					let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.output_, node);
					let find2 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input_2, node);
					let mut boolean = true;
					if (find1 != None) && (find2 != None) {
						if (maximum - counter) < 2 {
							boolean = false;
						}
					}else if ((find1 != None) && (find2 == None)) || ((find1 == None) && (find2 != None)) {
						if (maximum - counter) < 1 {
							boolean = false;
						}						
					}
					if boolean{
						match find1{
							None => {
						
							}, Some(x) => {
								search(x, maximxum, counter, list_of_arguments);
							}						
						}
						match find2{
							None => {
								
							}, Some(x) => {
								search(x, maximxum, counter, list_of_arguments);							
							}
						}
					}
				}, ArraySynapse(array_synapse) => {
					let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.vector_neuron, node);
					if (maximum - counter) >= 1{
						match find1{
							None => {
								
							}, Some(x) => {
								search(x, maximxum, counter, list_of_arguments);
							}						
						}
					}
				}
			}
		}, _ => {
			return;
		}
	}
}
pub fn delete_all_its_children_synapses(parent_node: Box<Node<Synapse>>, except_if_it_is: Box<Node<Synapse>>){
	let mut node = *parent_node;
	let output = if_not_loop_synapse(*parent_node);
	if output == None{
		return;
	}else if parent_node == except_if_it_is {
		return;
	}
	loop{
		if node.next != None{
			node = node.next.unwrap();
		}else{
			break;
		}
		match node{
			OtherSynapsesStruct(other_synapses_struct) => {
				match other_synapses_struct.kind{
					TwoInputSynapse(two_input_synpase) => {
						if Some(two_input_synpase.input_1) == output{
							delete_all_its_children_synapses(node, except_if_it_is);
							return;								
						} 
						if Some(two_input_synpase.input_2) == output{
							delete_all_its_children_synapses(node, except_if_it_is);
							return;								
						} 
					}, OneInputSynapse(one_input_synpase) => {
						if Some(one_input_synpase.input) == output{
							delete_all_its_children_synapses(node, except_if_it_is);
							return;								
						} 						
					}, ArraySynapse(array_synapse) => {
						if Some(array_synapse.vector_neuron) == output{
							delete_all_its_children_synapses(node, except_if_it_is);
							return;								
						}						
					}, WriteSynapse(write_synapse) => {
						if Some(array_synapse.number_neuron) == output{
							delete_all_its_children_synapses(node, except_if_it_is);
							return;								
						}						
					}
				}
			}, _ => {
				
			}
		}

	}
	link( *parent_node.prev , *parent_node.next );
} 
pub fn patch(synapse_network: &mut SynapseNetwork, patches: &mut Vec<Patch>, argument: &mut Vec<Vec<usize>>){
	let mut i = 0;
	for patch in patches{
		let mut j = 0;
		let mut vector_of_synapses: Vec<Synapse> = Vec::new();
		'outer_loop: for input in patch.input{
			let mut synapse = input;
			loop{
				synapse = Box<*synapse.prev>;
				match *synapse{
					OtherSynapsesStruct(other_synapses_struct) => {match other_synapses_struct{
						TwoInputSynapse(two_input_synapse) => {
							if (*input.output == two_input_synapse.input_1) || (*input.output == two_input_synapse.input_2){
								delete_all_its_children_synapses(synapse, patch.output);	
							}
						}, OneInputSynapse(one_input_synapse) => {
							if *input.output == one_input_synapse.input {
								delete_all_its_children_synapses(synapse, patch.output);	
							}							
						}, ArraySynapse(array_synapse) => {
							if (*input.output == other_synapses_struct.output) || (*input.output == two_input_synapse.vector_neuron) {
								delete_all_its_children_synapses(synapse, patch.output);
							}							
						}, WriteSynapse(write_synapse) => {
							if (*input.output == other_synapses_struct.output) || (*input.output == two_input_synapse.number_neuron) {
								delete_all_its_children_synapses(synapse, patch.output);	
							}
						}
					}}, _ => {
						
					}
				}
			}
			match *input{
				OtherSynapsesStruct(other_synapses_struct) => {
					other_synapses_struct.output = argument[i][j];		
				}, _ => {
					
				}
			}
			j++;
		}
		let pointer = *patch.insert_after_this.next;
		*(patch.insert_after_this).next = patch.patch.start;
		*pointer.prev = patch.patch.end;
		synapse_network.length += patch.length; 
		i++;
	} 
}
pub fn train(minimum_score: u32, synapse_network: &mut SynapseNetwork, dataset: Vec<DataPoint>, how_many_vector_arguments: usize, how_many_number_arguments: usize, how_many_randomly_selected_ones: usize, natural_n: u32, maximum_of_synapses: usize, used_memory_cells: &mut UsedMemoryCells, sample_size: usize, maximum_calibration_epochs: usize, how_much_to_consider_stagnant: usize){
	let mut variables = Vec<bool>;
	let mut vector_of_bools: NodeList<Bool> = synapse_network.len;
	let mut current_best_results = calculate_the_score(synapse_network.synapse_network, dataset, sample_size, synapse_network.how_many_number_arguments == None, synapse_network.how_many_vector_arguments == None, Vec::new());
	while calculate_score() < minimum_score{
		let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
		let mut patches: Vec<Patch> = Vec::new();
		list_of_vector_memories.push(Vec::new())
		mem::drop(list_of_vector_memories);
		run_synapse_network(synapse_network.synapse_network, list_of_vector_memories, number_memory: &mut [f64], Some<&mut vector1>, Some<&mut vector2>);				 
		loop{
			let mut current_node: Node<Synapse> = Some(synapse_network.end);
			let mut current_bool_node: Node<Bool> = Some(vector_of_bools.end);			
			loop{
				match node.value{
					OtherSynapsesStruct(other_synapses_struct) => {
						match other_synapses_struct{
							TwoInputSynapse(two_input_synapse) => {
								let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input_1, current_node, current_bool_node);
								let find2 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input_2, current_node, current_bool_node);
								let mut rng = rand::rng();
								let random_bool = rng.random::<bool>();
								if random_bool{
									match find1{
										None => {
											break;
										}, Some(x) => {
											current_node = x[0];
										}						
									}
								}else{
									match find2{
										None => {
											break;
										}, Some(x) => {
											current_node = x[0];							
										}
									}
								}
							}, OneInputSynapse(one_input_synapse) => {
								let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.input, current_node, current_bool_node);
								match find1{
									None => {
										break;
									}, Some(x) => {
										current_node = x[0];
									}						
								}				
							}, WriteSynapse(write_synapse) => {
								let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.number_neuron, current_node, current_bool_node);
								let find2 = find_last_cardinal_synapse_that_has_output_x_before_y(two_input_synapse.cardinal_neuron, current_node, current_bool_node);
								let mut rng = rand::rng();
								let random_bool = rng.random::<bool>();
								if random_bool{
									match find1{
										None => {
											break;
										}, Some(x) => {
											current_node = x[0];
										}						
									}
								}else{
									match find2{
										None => {
											break;
										}, Some(x) => {
											current_node = x[0];
										}						
									}
								}
							}, ArraySynapse(array_synapse) => {
								let find1 = find_last_synapse_that_has_output_x_before_y(two_input_synapse.vector_neuron, current_node, current_bool_node);
								let find2 = find_last_cardinal_synapse_that_has_output_x_before_y(two_input_synapse.cardinal_neuron, current_node, current_bool_node);
								let mut rng = rand::rng();
								let random_bool = rng.random::<bool>();
								if random_bool{
									match find1{
										None => {
											break;
										}, Some(x) => {
											current_node = x[0];
										}						
									}
								}else{
									match find2{
										None => {
											break;
										}, Some(x) => {
											current_node = x[0];
										}						
									}
								}
							}
						}
					}, PurelyCardinalSynapse(purely_cardinal_synapse) => {
						match find_last_cardinal_synapse_that_has_output_x_before_y(purely_cardinal_synapse.cardinal_neuron, current_node, current_bool_node){
							None => {
								break;
							}, Some(x) => {
								if (maximum - counter) >= 1{
									current_node = x[0];
								} 
							}
						} 			
					}, LoopSynapse => {
					break;
					}
				}
				let mut rng = rand::rng();
				if current_bool_node.value && rng.random::<bool>(){
					break;
				}
			}
			let mut counter = 0;
			let mut nodelist: NodeList<Box<Node<Synapse>>> = NodeList:new();
			search(current_node, maximum_of_synapses - synapse_network.length, &mut counter, &mut nodelist);			
			patches.push(NodeList::new());
			let mut vector_of_indexes_of_vector_output: Vec<usize> = Vec::new();
			let mut vector_of_indexes_of_number_output: Vec<usize> = Vec::new();
			let mut list_of_constants_in_the_network = Vec::new();
			let mut how_many_vector_arguments = 0;
			let mut how_many_number_arguments = 0;
			for node in nodelist {
				match if_not_loop_synapses(node){
					None => {
						error(0);
					}, Some(x) => {
						match x.output {
							0 => {
								how_many_number_arguments += 1;		
							}, 1 => {
								how_many_vector_arguments += 1;
							}
						}
					}
				}				
			}
//fn add_synapse(synapse_network: &mut LinkedList<Synapse>, number_of_vector_arguments: usize, number_of_number_arguments: usize, number_of_dependent_number_variables: &mut Vec<LinkedList<[usize; usize]>>, number_of_dependent_vector_variables: &mut Vec<LinkedList<[usize; usize]>>, should_it_be_vector: bool, list_of_constants_in_the_network: &mut Vector<usize>, may_it_contain_loop_synapses: bool) -> usize
			add_synapse(LinkedList.last(), how_many_vector_arguments, how_many_number_arguments, Vec::new(), Vec::new(), true, list_of_constants_in_the_network, true);
			for constant in list_of_constants_in_the_network{
				let mut variable = constant.value;
				for index in 0..(natural_n){
					variable = calibration(dataset, nodelist, constant, variable, None, maximum_calibration_epochs, true, how_much_to_consider_stagnant, index);																						
				}
			} 
			if (random_bool) || (synapse_network.length == maximum_of_synapses){
				break;
			}			
		}
		let mut results = 0.0;
		let newer_results = calculate_the_score(synapse_network.synapse_network, dataset, sample_size, false, synapse_network.how_many_vector_arguments == None, patches); 
		if newer_results >= current_best_results {
			current_best_results = newer_results;
			patch(synapse_network, patches, argument);
		}
	}
}
