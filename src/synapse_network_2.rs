use std::collections::LinkedList;
use std::mem;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PatchVector{
	pub vector: LinkedList<usize>,
	pub what_patch: usize,
	pub vector_arguments: Vec<Vec<f64>>,
	pub number_arguments: Vec<f64>,
	pub cardinal_arguments: Vec<usize>
}
pub struct NodeList{
	pub length: usize,
	pub start: Option<Node>,
	pub end: Option<Node> 
}
pub struct Node<T>{
	pub next: Option<Box<Node>>,
	pub prev: Option<Box<Node>>,
	pub value: T
}
pub struct Patch{
	pub input: Vec<Box<Node<Synapse>>>,
	pub output: Box<Node<Synapse>>,
	pub patch: LinkedList<Synapse>,
	pub insert_after_this: Box<Node<Synapse>> 	
}
pub struct LoopRanking{
	pub ranking: Vec<f64>,
	pub index_of_the_loop_synapse: usize,
}
pub enum KindsOfNeurons{
	Vector(usize),
	Scalar(f64),
}
pub struct FunctionCall<'a>{
	pub synapse_network: &'a LinkedList<Synapse>,
	pub arguments: Vec<u32>,	
}
pub struct LoopSynapse<'a, 'b>{
	pub loop_variable_starts_at: Vec<f64>,  
	pub at_the_end_of_the_loop: FunctionCall<'a>,
	pub check_if_it_should_exit_the_loop: &'b LinkedList<Synapse>,
	pub synapse_network_to_repeat: LinkedList<Synapse>,    
}
pub struct TwoInputSynapse{
	pub input_1: u32,
	pub input_2: u32,
	pub operation: TwoInputSynapseTypes,
}
pub struct OneInputSynapse{
	pub input: u32,
	pub operation: OneInputSynapseTypes,
}
pub struct ConstantSynapse{
	pub constant: f64,
}
pub struct RandomSynapse{
}
pub struct ArraySynapse{
	pub cardinal_neuron: usize,
	pub vector_neuron: u32,
}
pub struct WriteSynapse{
	pub cardinal_neuron: usize,
	pub number_neuron: u32,
}
pub struct PurelyCardinalSynapse{
	pub kind: PurelyCardinalSynapseKind,
	pub cardinal_neuron: usize
}
pub enum PurelyCardinalSynapseKind{
	ForwardSynapse,
	BackwardsSynapse
}
pub enum OtherSynapses{
	TwoInputSynapse(TwoInputSynapse),
	OneInputSynapse(OneInputSynapse),
	ConstantSynapse(ConstantSynapse),
	RandomSynapse(RandomSynapse),
	ArraySynapse(ArraySynapse),
	WriteSynapse(WriteSynapse),
}
pub struct OtherSynapsesStruct{
	pub output: u32,
	pub kind: OtherSynapses,	
}
pub enum Synapse{
	OtherSynapsesStruct(OtherSynapsesStruct),
	LoopSynapse(LoopSynapse<'static, 'static>),
	PurelyCardinalSynapse(PurelyCardinalSynapse)
}
#[derive(Debug, Copy, Clone)]
pub enum TwoInputSynapseTypes{
	Add,
	Multiply,
	EqualTo,
}
#[derive(Debug, Copy, Clone)]
pub enum OneInputSynapseTypes{
	IsNegative,
	Floor,
	InverseOf
}
pub static LIST_OF_VECTOR_MEMORIES: Mutex<Vec<Vec<usize>>>> = Mutex::new(Vec::new());
pub static LIST_OF_VECTORS: Mutex<Vec<f64>>> = Mutex::new(Vec::new());
pub fn execute_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &Synapse, ranking: Option<&mut Vector<usize>>){
	match synapse {
		Synapse::OtherSynapsesStruct(other_synapse) => {
			execute_other_synapse(vector_memory, number_memory, cardinal_memory, &other_synapse);
		},
		Synapse::LoopSynapse(_) => {
			execute_loop_synapse(vector_memory, number_memory, cardinal_memory, synapse, ranking);
		},
		Synapse::PurelyCardinalSynapse(purely_cardinal_synapse) => {
			if purely_cardinal_synapse.kind == ForwardSynapse{
				cardinal_memory[purely_cardinal_synapse.cardinal_neuron]++;
			}else{ //BackwardsSynapse
				cardinal_memory[purely_cardinal_synapse.cardinal_neuron]--;
			}
		}
	}
}
#[macro_export]
macro_rules! read_the_value_at {
    ($($x:expr, $vector_memory:expr, $number_memory:expr),*) => {{
        (
            $(
                if $x % 2 == 1 {
                    KindsOfNeurons::Vector(($x / 2) as usize)
                } else {
                    KindsOfNeurons::Scalar($number_memory[($x / 2) as usize])
                }
            ),*
        )
    }};
}
pub fn as_other_synapse(synapse: Synapse) -> OtherSynapsesStruct{
	match synapse{
		Synapse::OtherSynapsesStruct(other_synapse) => {
			other_synapse
		}, _ => {
			panic!("as_other_synapse ERROR");
		}
	}
}
pub fn execute_other_synapse<'a>(vector_memory: usize, number_memory: &'a mut  [f64], cardinal_memory: &'a mut [usize], Vesynapse: &OtherSynapsesStruct){
	match synapse.kind {
		OtherSynapses::TwoInputSynapse(_) => execute_two_input_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::OneInputSynapse(_) => execute_one_input_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::ConstantSynapse(_) => execute_constant_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::RandomSynapse(_) => execute_random_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::ArraySynapse(_) => execute_array_synapse(vector_memory, number_memory, cardinal_memory, &synapse),
		OtherSynapses::WriteSynapse(_) => execute_write_synapse(vector_memory, number_memory, cardinal_memory, &synapse),
		OtherSynapses::BackwardsSynapse(_) => execute_backwards_synapse(cardinal_memory, &synapse), 
		OtherSynapses::ForwardSynapse(_) => execute_forward_synapse(cardinal_memory, &synapse)
	}
}
pub fn execute_loop_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &Synapse, ranking: Option<&mut Vector<usize>>){
	match synapse {
		 Synapse::LoopSynapse(loop_synapse) => {
			let mut loop_variable = loop_synapse.loop_variable_starts_at.clone();
			loop{
				run_synapse_network(&loop_synapse.synapse_network_to_repeat, vector_memory, number_memory, cardinal_memory, ranking);
				let mut new_number_memory: Vec<f64> = Vec::new();
				let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
				let mut len = list_of_vector_memories.len();
				let index = list_of_vector_memories[vector_memory].len();
				list_of_vector_memories.push( vec![] );
				len += 1;
				list_of_vector_memories[len - 1][index - 1] = loop_variable.clone();
				for i in 0.. {
					let constant = loop_synapse.at_the_end_of_the_loop.arguments[i];
					if (constant % 2) == 0 {
						let variable = read_the_value_at!(constant, &vector_memory, &number_memory);
						match variable {
							KindsOfNeurons::Scalar(scl) => {
								new_number_memory[i] = scl;
							}, _ => {
								panic!("");
							}
						}
					}else{
						let variable = read_the_value_at!(constant, &vector_memory, &number_memory);
						match variable {
							KindsOfNeurons::Vector(vect) => {
								list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
								for j in 0..(list_of_vector_memories[vector_memory][vect].len()){
									list_of_vectors[list_of_vector_memories[len - 1][i]][j] = list_of_vectors[list_of_vector_memories[vector_memory][vect]][j];
								}
								drop(list_of_vectors);
							}, _ => {
								panic!("");
							}
						}
					}
				}
				match run_synapse_network(loop_synapse.at_the_end_of_the_loop.synapse_network, len - 1, &mut new_number_memory, Vec::New()){
					KindsOfNeurons::Scalar(_) => {
						panic!("wrong last synapse");
					}, KindsOfNeurons::Vector(vector) => {
						loop_variable = list_of_vector_memories[len - 1][vector].clone();
					} 
				}
				list_of_vector_memories.pop();
				len -= 1;
				let mut second_number_memory: Vec<f64> = Vec::new(); //Does something to that loop variable
				list_of_vector_memories.push( Vec::new() );
				len += 1;
				let index_2 = list_of_vector_memories[vector_memory].len();
				list_of_vector_memories[len - 1][index_2 - 1] = loop_variable.clone();
				//TODO: This is something that I need to fix...
				match run_synapse_network(loop_synapse.check_if_it_should_exit_the_loop, len - 1, &mut second_number_memory, Vec::New()){
					KindsOfNeurons::Scalar(scalar) => {
						if scalar < 1_f64 {
							break;		
						}
					}, 
					KindsOfNeurons::Vector(_) => {
						panic!("invalid");
					}
				}
				list_of_vector_memories.pop();
				drop(list_of_vector_memories);
			}
		}, Synapse::OtherSynapsesStruct(_) => {
			panic!("execute_loop_synapse: Called non-loop");			
		}
		 
	}
}
pub fn execute_array_synapse(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	match synapse.kind {
		OtherSynapses::ArraySynapse(array_synapse) => {
			let neuron = read_the_value_at!( array_synapse.vector_neuron, vector_memory, number_memory );
			match neuron {
				KindsOfNeurons::Scalar(_) => {
					panic!("Crashed");	
				}, KindsOfNeurons::Vector(vctr) => {
					 number_memory[synapse.output / 2] = vctr[cardinal_memory[array_synapse.cardinal_neuron]];		
				}
			}
		}
	}
		
}
pub fn execute_write_synapse(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	match synapse.kind {
		OtherSynapses::WriteSynapse(write_synapse) => {
			let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
			let number = read_the_value_at!( array_synapse.number_neuron, vector_memory, number_memory );
			match number{
				KindsOfNeurons::Scalar(scalar) => {
					list_of_vectors[list_of_vector_memories[vector_memory][synapse.output / 2]][cardinal_memory[synapse.cardinal_neuron]] = *number;
					drop(list_of_vector_memories);					
				}, KindsOfNeurons::Vector(_) => {
					panic!("Crashed");
				}
			}
		}
	}
		
}
pub fn execute_backwards_synapse(cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	cardinal_memory[synapse.output as usize] -= 1;		
}
pub fn execute_forward_synapse(cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	cardinal_memory[synapse.output as usize] += 1;		
} 
pub fn get_two_input_synapse_kind_property(synapse: &OtherSynapsesStruct) -> &TwoInputSynapse{
	match synapse.kind{
		OtherSynapses::TwoInputSynapse(ref arr) => {
			arr
		}, _ => {
			panic!("Get kind property error");
		}
	}
}
/*fn execute_two_input_synapse<'a>(vector_memory: &'a mut [Vec<f64>], number_memory: &'a mut  [f64], synapse: &OtherSynapsesStruct){
	let input_1 = read_the_value_at!(get_two_input_synapse_kind_property(synapse).input_1, vector_memory, number_memory).clone();
	let input_2 = read_the_value_at!(get_two_input_synapse_kind_property(synapse).input_2, vector_memory, number_memory).clone();
	match input_1{
		KindsOfNeurons::Vector(index_to_vector_1) => {
			let vector_1 = &mut vector_memory[*index_to_vector_1];
			match input_2 {
				KindsOfNeurons::Vector(index_to_vector_2) => {
					let vector_2 = &mut vector_memory[*index_to_vector_2];
					let mut bigger = vector_2.len();
					if (vector_1.len()) > (vector_2.len()) {
						bigger = vector_1.len();
					}
					let difference = get_length_of_output_neuron(vector_memory, synapse) - bigger;
					reduce_size(vector_memory, (synapse.output / 2) as usize, difference);
					let has_not_passed_smaller = true;
					let has_not_passed_smaller_2 = true;
					for i in 0..get_length_of_output_neuron(vector_memory, synapse){
						set_nth_value_of_output_neuron_to(i, vector_memory, synapse, execution_binary_operation(get_element_number_n_or_zero(vector_1,i,has_not_passed_smaller), get_element_number_n_or_zero(vector_2,i,has_not_passed_smaller_2), get_two_input_synapse_kind_property(synapse).operation));
					}
					for i in get_length_of_output_neuron(vector_memory, synapse)..bigger{
						adding_to_output_neuron(vector_memory, synapse, execution_binary_operation(get_element_number_n_or_zero(vector_1,i,has_not_passed_smaller), get_element_number_n_or_zero(vector_2,i,has_not_passed_smaller_2), get_two_input_synapse_kind_property(synapse).operation));
					}
				}, KindsOfNeurons::Scalar(scalar_2) => {
					let bigger = vector_1.len();
					let difference = get_length_of_output_neuron(vector_memory, synapse) - vector_1.len();
					let has_not_passed_smaller = true;
					let _has_not_passed_smaller_2 = true;
					reduce_size(vector_memory, (synapse.output / 2) as usize, difference);
					for i in 0..get_length_of_output_neuron(vector_memory, synapse){
						set_nth_value_of_output_neuron_to(i, vector_memory, synapse, execution_binary_operation(get_element_number_n_or_zero(vector_1,i,has_not_passed_smaller), *scalar_2, get_two_input_synapse_kind_property(synapse).operation));
					}
					for i in get_length_of_output_neuron(vector_memory, synapse)..bigger{
						adding_to_output_neuron(vector_memory, synapse, execution_binary_operation(get_element_number_n_or_zero(vector_1,i,has_not_passed_smaller), *scalar_2, get_two_input_synapse_kind_property(synapse).operation));
					}															
				}	
			}
		},
		KindsOfNeurons::Scalar(scalar_1) => {
			match input_2 {
				KindsOfNeurons::Vector(index_to_vector_2) => {
					let vector_2 = &mut vector_memory[*index_to_vector_2];
					let mut new_value: f64 = 0 as f64;
					for i in 0..vector_2.len(){
						new_value += execution_binary_operation(*scalar_1, vector_2[i], get_two_input_synapse_kind_property(synapse).operation);
					}			
					set_output_neuron_to(vector_memory, number_memory, synapse, KindsOfNeurons::Scalar(new_value));		
				}, KindsOfNeurons::Scalar(scalar_2) => {//
					set_output_neuron_to(vector_memory, number_memory, synapse, KindsOfNeurons::Scalar(execution_binary_operation(*scalar_1, *scalar_2, get_two_input_synapse_kind_property(synapse).operation)));
				}			
			}
		}
	}
}*/
pub fn execute_two_input_synapse<'a>(
    vector_memory: usize,
    number_memory: &'a mut [f64], 
    synapse: &OtherSynapsesStruct,
) {
    let input_1 = read_the_value_at!(
        get_two_input_synapse_kind_property(synapse).input_1,
        vector_memory,
        number_memory
    );

    let input_2 = read_the_value_at!(
        get_two_input_synapse_kind_property(synapse).input_2,
        vector_memory,
        number_memory
    );

    let operation = get_two_input_synapse_kind_property(synapse).operation;
	let mut list_of_vector_memories_first = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	let _output_len = get_length_of_output_neuron(&mut (list_of_vector_memories_first[vector_memory]), synapse);
    let _output_index = (synapse.output / 2) as usize;
	mem::drop(list_of_vector_memories_first);
    match input_1 {
        KindsOfNeurons::Vector(index_to_vector_1) => {
            //let vector_1 = &mut (vector_memory[*index_to_vector_1]);

            match input_2 {
                KindsOfNeurons::Vector(index_to_vector_2) => {
                    //let vector_2 = &mut (vector_memory[*index_to_vector_2]);
                    /*let bigger = vector_memory[*index_to_vector_1].len().max(vector_memory[*index_to_vector_2].len());
                    let difference = output_len.saturating_sub(bigger);
					{
                    	reduce_size(vector_memory, output_index, difference);
					}
                    for i in 0..output_len {
                        let val = execution_binary_operation(
                            get_element_number_n_or_zero(&vector_memory[*index_to_vector_1], i, true),
                            get_element_number_n_or_zero(&vector_memory[*index_to_vector_2], i, true),
                            operation,
                        );
                        set_nth_value_of_output_neuron_to(i, vector_memory, synapse, val);
                    }

                    for i in output_len..bigger {
                        let val = execution_binary_operation(
                            get_element_number_n_or_zero(&vector_memory[*index_to_vector_1], i, true),
                            get_element_number_n_or_zero(&vector_memory[*index_to_vector_2], i, true),
                            operation,
                        );
                        adding_to_output_neuron(vector_memory, synapse, val);
                    }*/
					let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
					let index_to_smaller = index_to_vector_2;
					let index_to_bigger = index_to_vector_1;
                    if list_of_vector_memories[vector_memory][index_to_vector_1] > list_of_vector_memories[vector_memory][index_to_vector_2] {
						let _index_to_smaller = index_to_vector_1;
						let _index_to_bigger = index_to_vector_2; 
                    }
                    let length1 = list_of_vector_memories[vector_memory][index_to_bigger].len();
                    let length2 = list_of_vector_memories[vector_memory][index_to_smaller].len();
					{
						while get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse) > (length2){
							pop_output_neuron(&mut list_of_vector_memories[vector_memory], synapse);						
						}
					}
					//TODO: Something to be fixed
					let already_exceeded_original_length = false;
					mem::drop(list_of_vector_memories);
                   	for i in 0..(length2){
                   		if already_exceeded_original_length {
							let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
							let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
                   			let result = execution_binary_operation( list_of_vectors[ list_of_vector_memories_new[vector_memory][index_to_smaller]][i], list_of_vectors[list_of_vector_memories_new[vector_memory][index_to_bigger]][i], operation);
							mem::drop(list_of_vectors);
							mem::drop(list_of_vector_memories_new);
							let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
							adding_to_output_neuron(&mut list_of_vector_memories_2[vector_memory], synapse, result);
							mem::drop(list_of_vector_memories_2);	
                   		}else{
							let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
							let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
                   			let result = execution_binary_operation( list_of_vectors[ list_of_vector_memories_new[vector_memory][index_to_smaller]][i], list_of_vectors[list_of_vector_memories_new[vector_memory][index_to_bigger]][i], operation);
							mem::drop(list_of_vectors);
							mem::drop(list_of_vector_memories_new);
							let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
                   			set_nth_value_of_output_neuron_to(i, &mut list_of_vector_memories_2[vector_memory], synapse, result);
							mem::drop(list_of_vector_memories_2);
                   		}	
                   	}
					for i in (length2)..(length1) {
                   		if already_exceeded_original_length {
							let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
							let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
                   			let result = list_of_vectors[list_of_vector_memories_new[vector_memory][index_to_bigger]][i];
							mem::drop(list_of_vectors);
							mem::drop(list_of_vector_memories_new);
							let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
                   			adding_to_output_neuron(&mut list_of_vector_memories_2[vector_memory], synapse, result);	
							mem::drop(list_of_vector_memories_2);
                   		}else{
							let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
							let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
                   			let result = list_of_vectors[list_of_vector_memories_new[vector_memory][index_to_bigger]][i];
                   			mem::drop(list_of_vectors);
							mem::drop(list_of_vector_memories_new);
							let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
                   			set_nth_value_of_output_neuron_to(i, &mut list_of_vector_memories_2[vector_memory], synapse, result);
                   			mem::drop(list_of_vector_memories_2);
                   		}
	                }
                }
                KindsOfNeurons::Scalar(scalar_2) => {
                	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
					while get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse) > (list_of_vector_memories[vector_memory][index_to_vector_1].len()){
						pop_output_neuron(&mut list_of_vector_memories[vector_memory], synapse);						
					}
					let already_exceeded_original_length = false;
                   	for i in 0..(list_of_vector_memories[vector_memory][index_to_vector_1].len()){
                   		if already_exceeded_original_length {
							let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();                   		
							let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
                   			let result = execution_binary_operation( list_of_vectors[list_of_vector_memories_new[vector_memory][index_to_vector_1]][i], scalar_2, operation);
							mem::drop(list_of_vectors);
							mem::drop(list_of_vector_memories_new);
							let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap(); 
                   			adding_to_output_neuron(&mut list_of_vector_memories_2[vector_memory], synapse, result);	
                   			mem::drop(list_of_vector_memories_2);
                   		}else{
							let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();                   		
							let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
                   			let result = execution_binary_operation( list_of_vectors[list_of_vector_memories_new[vector_memory][index_to_vector_1]][i], scalar_2, operation);
							mem::drop(list_of_vectors);
							mem::drop(list_of_vector_memories_new);
							let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
                   			set_nth_value_of_output_neuron_to(i, &mut list_of_vector_memories_2[vector_memory], synapse, result);
                   			mem::drop(list_of_vector_memories_2);
                   		}	
                   	}
                }
            }
        }
        KindsOfNeurons::Scalar(scalar_1) => {
            match input_2 {
                KindsOfNeurons::Vector(_) => {
                    /*let vector_2 = &vector_memory[*index_to_vector_2];
                    let new_value = vector_2.iter().fold(0.0, |acc, &val| {
                        acc + execution_binary_operation(*scalar_1, val, operation)
                    });
                    set_output_neuron_to(
                        vector_memory,
                        number_memory,
                        synapse,
                        KindsOfNeurons::Scalar(new_value),
                    );*/
                    panic!("-");
                }
                KindsOfNeurons::Scalar(scalar_2) => {
                	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
                    let new_value = execution_binary_operation(scalar_1, scalar_2, operation);
					set_output_neuron_to(
	                        &mut list_of_vector_memories[vector_memory],
	                        number_memory,
	                        synapse,
	                        KindsOfNeurons::Scalar(new_value),
	                );
	                drop(list_of_vector_memories);
                }
            }
        }
    }
}
pub fn get_one_input_synapse_kind_property(synapse: &OtherSynapsesStruct) -> &OneInputSynapse{
 	match synapse.kind{
		OtherSynapses::OneInputSynapse(ref arr) => {
			arr
		}, _ => {
			panic!("Get kind property error");
		}
	}
}
pub fn executing_one_synapse_operation(input: f64, operation: OneInputSynapseTypes) -> f64{
	match operation{
		OneInputSynapseTypes::IsNegative => {
			if input < (0 as f64) {
				1_f64
			}else{
				0_f64
			}
		}, OneInputSynapseTypes::Floor => {
			input.floor()
		}, OneInputSynapseTypes::InverseOf => {
			1 / input
		}
	}
}
pub fn execute_one_input_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], synapse: &OtherSynapsesStruct){
	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	let input = read_the_value_at!(get_one_input_synapse_kind_property(synapse).input, vector_memory, number_memory);
	let typ = &get_one_input_synapse_kind_property(synapse).operation;
	match input{
		KindsOfNeurons::Scalar(vctr) => {
			let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
			set_output_neuron_to(&mut list_of_vector_memories[vector_memory], number_memory, synapse, KindsOfNeurons::Scalar(executing_one_synapse_operation(vctr, *typ)));
			mem::drop(list_of_vector_memories);
		}, 
		KindsOfNeurons::Vector(int) => {
			if list_of_vector_memories[vector_memory][int].len() < get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse) {
				for _i in 0..(get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse) - list_of_vector_memories[vector_memory][int].len()){
					pop_output_neuron(&mut list_of_vector_memories[vector_memory], synapse);
				}
			}//
			let len = get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse);
			mem::drop(list_of_vector_memories);
			for i in 0..len{
//vector_memory: &mut Vec<Vec<f64>>, synapse: &OtherSynapsesStruct, value: f64
				let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
				let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
				let result = executing_one_synapse_operation(list_of_vectors[list_of_vector_memories_new[vector_memory][int]][i], *typ);
				mem::drop(list_of_vectors);
				mem::drop(list_of_vector_memories_new);
				let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
				set_nth_value_of_output_neuron_to(i, &mut list_of_vector_memories_2[vector_memory], synapse, result);
				mem::drop(list_of_vector_memories_2);
			}
			let mut list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
			let length = list_of_vector_memories_new[vector_memory][int].len();
			if length > get_length_of_output_neuron(&mut list_of_vector_memories_new[vector_memory], synapse) {
				let another_length = get_length_of_output_neuron(&mut list_of_vector_memories_new[vector_memory], synapse);
				mem::drop(list_of_vector_memories_new);
				for i in another_length..length{
					let list_of_vector_memories_new = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
					let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
					let result = executing_one_synapse_operation(list_of_vectors[list_of_vector_memories_new[vector_memory][int]][i], *typ);
					mem::drop(list_of_vectors);
					mem::drop(list_of_vector_memories_new);
					let mut list_of_vector_memories_2 = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
					adding_to_output_neuron(&mut list_of_vector_memories_2[vector_memory], synapse, result);
					mem::drop(list_of_vector_memories_2); 
				}
			}
		}
	}
}
pub fn get_constant_synapse_kind_property(synapse: &OtherSynapsesStruct) -> &ConstantSynapse{
	match synapse.kind {
		OtherSynapses::ConstantSynapse(ref arr) => {
			arr
		}, _ => {
			panic!("Get kind property error");
		}
	}
}
pub fn get_the_constant_of_a_constant_synapse(synapse: &OtherSynapsesStruct) -> f64{
	match synapse.kind{
		OtherSynapses::ConstantSynapse(ref const_) => {
			const_.constant
		}, _ => {
			panic!("exiting");
		}
	}
}
pub fn execute_constant_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], synapse: &OtherSynapsesStruct){
	println!("-");
	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	println!("=");
	match synapse.kind {
		OtherSynapses::ConstantSynapse(_) => {if (synapse.output % 2) == 1 {
			println!("?");
			for i in 0..(get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse)){
				set_nth_value_of_output_neuron_to(i, &mut list_of_vector_memories[vector_memory], synapse, get_the_constant_of_a_constant_synapse(synapse));
			}
		}else{
			println!("!");
			set_output_neuron_to(&mut list_of_vector_memories[vector_memory], number_memory, synapse, KindsOfNeurons::Scalar(get_the_constant_of_a_constant_synapse(synapse)));			
		}}, _ => {
			panic!("execute_constant_synapse");
		}
	}
	println!("@");
	mem::drop(list_of_vector_memories);
}
pub fn execute_random_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], synapse: &OtherSynapsesStruct){
	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	if (synapse.output % 2) == 1 {
		for i in 0..(get_length_of_output_neuron(&mut list_of_vector_memories[vector_memory], synapse)){
			set_nth_value_of_output_neuron_to(i, &mut list_of_vector_memories[vector_memory], synapse, generate_random_number());
		}
	}else{
		set_output_neuron_to(&mut list_of_vector_memories[vector_memory], number_memory, synapse, KindsOfNeurons::Scalar(generate_random_number()));			
	}
	mem::drop(list_of_vector_memories);
}

/*fn get_element_number_n_or_zero(vector: &Vec<f64>, index: usize, boolean: bool) -> f64{
	if boolean {
		vector[index]
	}else{
		0_f64
	}
}*/
pub fn execution_binary_operation(input_1: f64, input_2: f64, operation: TwoInputSynapseTypes) -> f64{
	match operation{
		TwoInputSynapseTypes::Add => {
			input_1 + input_2
		},
		TwoInputSynapseTypes::Multiply =>{
			input_1 * input_2	
		},
		TwoInputSynapseTypes::EqualTo => {
			if input_1 == input_2{
				1_f64
			}else{
				0_f64
			}
		},
	}						
}
pub fn get_length_of_output_neuron<'a>(vector_memory: &'a mut  Vec<usize>, synapse: &OtherSynapsesStruct) -> usize{
	let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
	list_of_vectors[vector_memory[(synapse.output / 2) as usize]].len()
	drop(list_of_vectors);
}
pub fn pop_output_neuron<'a>(vector_memory: &'a mut  Vec<usize>, synapse: &OtherSynapsesStruct){
	let mut list_of_vectors  = LIST_OF_VECTORS.lock().unwrap();
	list_of_vectors[vector_memory[(synapse.output / 2) as usize]].pop(); 	
	drop(list_of_vectors);
}
pub fn set_nth_value_of_output_neuron_to<'a>(n: usize, vector_memory: &'a mut Vec<Vec<f64>>, synapse: &OtherSynapsesStruct, value: f64){
	let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
	list_of_vectors[vector_memory[(synapse.output / 2) as usize]][n] = value;
	drop(list_of_vectors);
}
pub fn set_output_neuron_to<'a>(vector_memory: &'a mut  Vec<Vec<f64>>, number_memory: &'a mut  [f64], synapse: &OtherSynapsesStruct, value: KindsOfNeurons){
	if (synapse.output % 2) == 0{
		match value {
			KindsOfNeurons::Scalar(scalar) => {number_memory[(synapse.output / 2) as usize] = scalar},
			KindsOfNeurons::Vector(_) => {panic!("mismatch set_output_neuron_to function")}  
		}
	}else{
		match value {
			KindsOfNeurons::Scalar(_) => {panic!("mismatch set_output_neuron_to function")},
			KindsOfNeurons::Vector(vector) => {
				let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
				list_of_vectors[vector_memory[(synapse.output / 2) as usize]] = list_of_vectors[vector_memory[vector]].clone();
				drop(list_of_vectors);
			}  
		}
	}
}
pub fn clearing_output_neuron<'a>(vector_memory: &'a mut Vec<Vec<f64>>, synapse: &OtherSynapsesStruct){
	if(synapse.output % 2) == 1{
		let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
		list_of_vectors[vector_memory[(synapse.output / 2) as usize]].clear();
		drop(list_of_vectors);
	}	
}
pub fn adding_to_output_neuron<'a>(vector_memory: &'a mut Vec<Vec<f64>>, synapse: &OtherSynapsesStruct, value: f64){
	if(synapse.output % 2) == 1{
		let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
		list_of_vectors[vector_memory[(synapse.output / 2) as usize]].push(value);
		drop(list_of_vectors);
	}		
} 
pub fn get_neuron_scalar(neuron: KindsOfNeurons) -> f64{
	match neuron {
		KindsOfNeurons::Scalar(sclr) => {
			sclr
		}, KindsOfNeurons::Vector(_) => {
			panic!("#");
		}
	}
}
pub fn generate_random_number() -> f64{
	32_f64
	//rand::random<f64>()
	//TODO: Implementing pseudo-random number generation

}
pub fn convert_synapse_to_other_synapses_struct(synapse: &Synapse) -> &OtherSynapsesStruct{
	match &synapse{
		Synapse::OtherSynapsesStruct(other_synapses_struct) => {
			other_synapses_struct
		}, _ => {
			panic!("convert_synapse_to_other_synapses_struct")
		}
	}
}
pub fn run_synapse_network(synapse_network: &LinkedList<Synapse>, vector_memory: usize, number_memory: &mut [f64], cardinal_memory: &'a mut [usize], ranking: Option<&mut Vec<f64>>, vector_of_ranking: Option<&mut Vec<LoopRanking>>, patches: Vec<Patch>) -> KindsOfNeurons{
	let mut iter = synapse_network.iter();
	let mut i: usize = 0;
	let mut last = None;
	let mut vector_of_patch_vector: LinkedList<PatchVector> = Vec::New();
	vector_of_ranking.push(Vec::New());
	while let Some(value) = iter.next(){		
		let usize_variable = 0;
		while/*if*/ vector_of_patch_vector.len() > 0{
			let option1 = None;
			let option2 = None;
			let option_output = None;
			match value{
				OtherSynapses(other_synapses) => {
					option_output = Some(other_synapses.output);
					match other_synapses => {
						TwoInputSynapse(two_input_synapse) => {
							option1 = Some(two_input_synapse.input_1);
							option2 = Some(two_input_synapse.input_2);			
						}, OneInputSynapse(one_input_synapse) => {
							option1 = Some(one_input_synapse.input_1);
						}, ArraySynapse(array_synapse) => {
							option1 = Some(array_synapse.vector_neuron);							
						}, _ => {}
					}
				}, _ => {
					continue;
				}
			}
			one_input_synapse = if_one_input_synapse(value);
			two_input_synapse = if_two_input_synapse(value);
			array_synapse = if_two_input_synapse(value);
			/*possibly change this*/
			for patch_ in vector_of_patch_vector{
				let patch_vector = patch_.vector;
				for patch_number in patch_vector{
					match option1 {
						Some(value1) => {
							if value1 == patch_number{
								patch_.vector.push( if_not_loop_synapse(value).output );
							}
						},
						_ => {
							Some(value2) => {
								if value2 == patch_number{
									patch_.vector.push( if_not_loop_synapse(value).output );
								}
							}, _ => {
								match option_output {
									Some(value_output) => {
										if value_output == patch_number{
											//TODO: Remove the element
										}										
									}, 
									_ => {
										
									}
								}
							}
						}, 
					}
				} 			
			}
			break;
		}
		a: for j in 0..(patches.len()) {
			let patch = patches[j];
			for input in patch.input {
				if input == i{
					for patch_vector in vector_of_patch_vector{
						if patch_vector.what_patch == j{
 							patch_vector.vector.push(if_not_loop_synapse(value).output);		
						}
					}					
					vector_of_patch_vector.push(PatchVector{vector: synapse_network, what_patch: usize_variable, number_arguments: Vec<f64>, vector_arguments: Vec<usize>, cardinal_arguments: Vec<usize>});
					last = Some(value);
					i++;		
					
				} 	
			}
			if patches[patch.what_patch].output == i {
				let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
				let length = list_of_vector_memories.len();
				list_of_vector_memories.push(Vec::New());
				let mut number_memory = Vec::New();
				let mut cardinal_memory = Vec::New();
				drop(list_of_vector_memories);
				run_synapse_network( patches[patch.what_patch].patch, number_memory, cardinal_memory ); //TODO: fix this	
			}
			usize_variable++;
		}
		execute_synapse(vector_memory, number_memory, cardinal_memory, value, vector_of_ranking.last().ranking);
		if vector_of_ranking.last().ranking.len() > 0 {
			vector_of_ranking.last().index_of_the_loop_synapse = value;		
			vector_of_ranking.push(Vec::New());
		}					
		last = Some(value);
		i++;
	}
	vector_of_ranking.pop();
	match last{
		Some(x) => {
			match x {
				Synapse::OtherSynapsesStruct(other_synapses_struct) => {
					return read_the_value_at!(other_synapses_struct.output, vector_memory, number_memory);
				}, Synapse::LoopSynapse(_) => {
					panic!("Invalid last synapse");
				}			
			}
		}, None => {
			panic!("synapse network length 0");
		}
	}
}
