use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PatchVector{
	pub vector: NodeList<usize>,
	pub what_patch: usize,
	pub vector_arguments: Vec<Vec<f64>>,
	pub number_arguments: Vec<f64>,
	pub cardinal_arguments: Vec<usize>
}
#[derive(Clone, Debug)]
pub struct NodeList<T>{
	pub length: usize,
	pub start: Option<Rc<RefCell<Node<T>>>>,
	pub end: Option<Rc<RefCell<Node<T>>>> 
}
#[derive(Debug)]
pub struct Node<T>{
	pub next: Option<Rc<RefCell<Node<T>>>>,
	pub prev: Option<Rc<RefCell<Node<T>>>>,
	pub value: T
}
impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }

    pub fn link(a: &Rc<RefCell<Node<T>>>, b: &Rc<RefCell<Node<T>>>) {
        a.borrow_mut().next = Some(Rc::clone(b));
        b.borrow_mut().prev = Some(Rc::clone(a));
    }
}
pub struct Patch{
	pub input: Vec<Rc<RefCell<Node<Synapse>>>>,
	pub output: Rc<RefCell<Node<Synapse>>>,
	pub patch: NodeList<Synapse>,
	pub insert_after_this: Rc<RefCell<Node<Synapse>>> 	
}
pub enum KindsOfNeurons{
	Vector(usize),
	Scalar(f64),
}
pub struct FunctionCall<'a>{
	pub synapse_network: &'a NodeList<Synapse>,
	pub arguments: Vec<u32>,	
}
pub struct LoopSynapse<'a, 'b>{
	pub loop_variable_starts_at: Vec<f64>,  
	pub at_the_end_of_the_loop: FunctionCall<'a>,
	pub check_if_it_should_exit_the_loop: &'b NodeList<Synapse>,
	pub synapse_network_to_repeat: NodeList<Synapse>,    
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
pub static LIST_OF_VECTOR_MEMORIESLIST_OF_VECTOR_MEMORIES: Mutex<Vec<Vec<usize>>> = Mutex::new(Vec::new());
pub static LIST_OF_VECTORS: Mutex<Vec<Vec<f64>>> = Mutex::new(Vec::new());
pub fn execute_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &Synapse){
	match synapse {
		Synapse::OtherSynapsesStruct(other_synapse) => {
			execute_other_synapse(vector_memory, number_memory, cardinal_memory, &other_synapse);
		},
		Synapse::LoopSynapse(_) => {
			execute_loop_synapse(vector_memory, number_memory, cardinal_memory, synapse);
		},
		Synapse::PurelyCardinalSynapse(purely_cardinal_synapse) => {
			match purely_cardinal_synapse.kind{ 
				PurelyCardinalSynapseKind::ForwardSynapse => { //PurelyCardinalSynapseKind
					cardinal_memory[purely_cardinal_synapse.cardinal_neuron] += 1;
				}, PurelyCardinalSynapseKind::BackwardsSynapse => { //BackwardsSynapse
					cardinal_memory[purely_cardinal_synapse.cardinal_neuron] -= 1;
				}
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
pub fn execute_other_synapse<'a>(vector_memory: usize, number_memory: &'a mut  [f64], cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct){
	match synapse.kind {
		OtherSynapses::TwoInputSynapse(_) => execute_two_input_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::OneInputSynapse(_) => execute_one_input_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::ConstantSynapse(_) => execute_constant_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::RandomSynapse(_) => execute_random_synapse(vector_memory, number_memory, &synapse),
		OtherSynapses::ArraySynapse(_) => execute_array_synapse(vector_memory, number_memory, cardinal_memory, &synapse),
		OtherSynapses::WriteSynapse(_) => execute_write_synapse(vector_memory, number_memory, cardinal_memory, &synapse),
	}
}
pub fn execute_loop_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &Synapse){
	match synapse {
		 Synapse::LoopSynapse(loop_synapse) => {
			let mut loop_variable = loop_synapse.loop_variable_starts_at.clone();
			loop{
				run_synapse_network(&loop_synapse.synapse_network_to_repeat, vector_memory, number_memory, cardinal_memory, Vec::new());
				let mut new_number_memory: Vec<f64> = Vec::new();
				let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
				let mut len = list_of_vector_memories.len();
				let index = list_of_vector_memories[vector_memory].len();
				list_of_vector_memories.push( vec![] );
				len += 1;
				let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();				
				list_of_vectors[list_of_vector_memories[len - 1][index - 1]] = loop_variable.clone();
				mem::drop(list_of_vectors);
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
								let mut list_of_vectors_2 = LIST_OF_VECTORS.lock().unwrap();
								for j in 0..(list_of_vectors_2[list_of_vector_memories[vector_memory][vect]].len()){
									list_of_vectors_2[list_of_vector_memories[len - 1][i]][j] = list_of_vectors_2[list_of_vector_memories[vector_memory][vect]][j];
								}
								drop(list_of_vectors_2);
							}, _ => {
								panic!("");
							}
						}
					}
				}
				let mut new_cardinal_memory = Vec::new();
				let mut list_of_vectors_3 = LIST_OF_VECTORS.lock().unwrap();
				match run_synapse_network(loop_synapse.at_the_end_of_the_loop.synapse_network, len - 1, &mut new_number_memory, &mut new_cardinal_memory, Vec::new()){
					KindsOfNeurons::Scalar(_) => {
						panic!("wrong last synapse");
					}, KindsOfNeurons::Vector(vector) => {
						loop_variable = list_of_vectors_3[list_of_vector_memories[len - 1][vector]].clone();
					} 
				}
				mem::drop(list_of_vectors_3);
				list_of_vector_memories.pop();
				len -= 1;
				let mut second_number_memory: Vec<f64> = Vec::new(); //Does something to that loop variable
				list_of_vector_memories.push( Vec::new() );
				len += 1;
				let index_2 = list_of_vector_memories[vector_memory].len();
				list_of_vectors[list_of_vector_memories[len - 1][index_2 - 1]] = loop_variable.clone();
				//TODO: This is something that I need to fix...
				let mut another_cardinal_memory = Vec::new();
				match run_synapse_network(loop_synapse.check_if_it_should_exit_the_loop, len - 1, &mut second_number_memory, &mut another_cardinal_memory, Vec::new()){
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
pub fn execute_array_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	match synapse.kind {
		OtherSynapses::ArraySynapse(array_synapse) => {
			let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
			let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();

			let neuron = read_the_value_at!( array_synapse.vector_neuron, vector_memory, number_memory );
			match neuron {
				KindsOfNeurons::Scalar(_) => {
					panic!("Crashed");	
				}, KindsOfNeurons::Vector(vctr) => {
					 number_memory[(synapse.output / 2) as usize] = list_of_vectors[ list_of_vector_memories[vctr] [ cardinal_memory[array_synapse.cardinal_neuron] ] ];		
				}
			}
		}
	}
		
}
pub fn execute_write_synapse<'a>(vector_memory: usize, number_memory: &'a mut [f64], cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	match synapse.kind {
		OtherSynapses::WriteSynapse(write_synapse) => {
			let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
			let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
			let number = read_the_value_at!( array_synapse.number_neuron, vector_memory, number_memory );
			match number{
				KindsOfNeurons::Scalar(scalar) => {
					list_of_vectors[ list_of_vector_memories[vctr] [ cardinal_memory[array_synapse.cardinal_neuron] ] ] = *number;
					drop(list_of_vector_memories);					
				}, KindsOfNeurons::Vector(_) => {
					panic!("Crashed");
				}
			}
		}
	}
		
}
pub fn execute_backwards_synapse<'a>(cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
	cardinal_memory[synapse.output as usize] -= 1;		
}
pub fn execute_forward_synapse<'a>(cardinal_memory: &'a mut [usize], synapse: &OtherSynapsesStruct) {
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
pub fn get_length_of_output_neuron<'a>(vector_memory: &'a mut  usize, synapse: &OtherSynapsesStruct) -> usize{
	let list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
	let list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	let value = list_of_vectors[list_of_vector_memories[*vector_memory][(synapse.output / 2) as usize]].len();
	drop(list_of_vectors);
	drop(list_of_vector_memories);
	value
}
pub fn pop_output_neuron<'a>(vector_memory: &'a mut  usize, synapse: &OtherSynapsesStruct){
	let mut list_of_vectors  = LIST_OF_VECTORS.lock().unwrap();
	let list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	list_of_vectors[list_of_vector_memories[*vector_memory][(synapse.output / 2) as usize]].pop(); 	
	drop(list_of_vectors);
}
pub fn set_nth_value_of_output_neuron_to<'a>(n: usize, vector_memory: &'a mut usize, synapse: &OtherSynapsesStruct, value: f64){
	let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
	let list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	list_of_vectors[ list_of_vector_memories[*vector_memory][ (synapse.output / 2) as usize ] ]  [n] = value;
	drop(list_of_vectors);
	drop(list_of_vector_memories);
}
pub fn set_output_neuron_to<'a>(vector_memory: &'a mut  usize, number_memory: &'a mut  [f64], synapse: &OtherSynapsesStruct, value: KindsOfNeurons){
	if (synapse.output % 2) == 0{
		match value {
			KindsOfNeurons::Scalar(scalar) => {number_memory[(synapse.output / 2) as usize] = scalar},
			KindsOfNeurons::Vector(_) => {panic!("mismatch set_output_neuron_to function")}  
		}
	}else{
		match value {
			KindsOfNeurons::Scalar(_) => {panic!("mismatch set_output_neuron_to function")},
			KindsOfNeurons::Vector(vector) => {
				let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
/*
	list_of_vectors[list_of_vector_memories[*vector_memory][(synapse.output / 2) as usize]]
*/				
				list_of_vector_memories[*vector_memory][(synapse.output / 2) as usize] = vector;
				drop(list_of_vector_memories);
			}  
		}
	}
}
pub fn clearing_output_neuron<'a>(vector_memory: &'a mut usize, synapse: &OtherSynapsesStruct){
	if(synapse.output % 2) == 1{
		let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
		let list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
		list_of_vectors[ list_of_vector_memories[*vector_memory][ (synapse.output / 2) as usize ] ].clear();
		drop(list_of_vectors);
		drop(list_of_vector_memories);
	}	
}
pub fn adding_to_output_neuron<'a>(vector_memory: &'a mut usize, synapse: &OtherSynapsesStruct, value: f64){
	if(synapse.output % 2) == 1{
		let mut list_of_vectors = LIST_OF_VECTORS.lock().unwrap();
		let list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
		list_of_vectors[ list_of_vector_memories[*vector_memory][ (synapse.output / 2) as usize ] ].push(value);
		drop(list_of_vectors);
		drop(list_of_vector_memories);
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
pub fn run_synapse_network<'a>(synapse_network: &NodeList<Synapse>, vector_memory: usize, number_memory: &mut [f64], cardinal_memory: &'a mut [usize], patches: Vec<Patch>) -> KindsOfNeurons{
	let mut synapse: Rc<RefCell<Node<Synapse>>> = synapse_network.start.unwrap();
	let mut nodelist_vec: Vec<Option<[NodeList< Option<u32> >; 2]>> = vec![None; patches.len()];
	let mut nodelists = nodelist_vec.into_boxed_slice(); 
	let mut i: usize = 0;
/*
pub struct NodeList{
	pub length: usize,
	pub start: Option<Node>,
	pub end: Option<Node> 
}
*/
	'outerest_loop: loop{
		'outer_loop: for nodelist in nodelists{
		match nodelist { 
			None => {
				
			}, 
			Some(y) => {
				let mut synapse_ = y[0].start.unwrap();
				let mut synapse_2 = y[1].start.unwrap();
				let mut first_nodelist_none = None;
				let mut first_nodelist_none_2 = None;
				loop{
					match (*synapse).value{
						Synapse::OtherSynapsesStruct(other_synapses_struct) => {
							match (*synapse_).value {
								Some(x) => { 
									match other_synapses_struct.kind{
										OtherSynapses::TwoInputSynapse(two_input_synapse) => {
											if (two_input_synapse.input_1 == x) || (two_input_synapse.input_2 == x){
												if first_nodelist_none.is_none(){
													loop{
														if (*synapse_).value.is_none() {
															(*synapse_).value = Some(other_synapses_struct.output);
															i += 1;
															continue 'outerest_loop;
														} 
														if (*synapse_).next.is_none(){
															let some_rc = Rc::new(RefCell::new(Node {
															    value: other_synapses_struct.output,
															    next: None,
															    prev: None,
															}));
															Node::link(y[0].end.clone(), &some_rc);
															y[0].end = (*(y[0].end)).unwrap().next;
															i += 1;
															continue 'outerest_loop;
														}
														synapse_ = (*synapse_).next;														
													}
													
												}else{
													*(first_nodelist_none.unwrap()).value	= Some(other_synapses_struct.output);
													i += 1;
													continue 'outerest_loop;
												}
											}
										}, OtherSynapses::OneInputSynapse(one_input_synapse) => {
											if  one_input_synapse.input == synapse_ {
												if first_nodelist_none == None{
													loop{
														if (*synapse_).value.is_none(){
															*synapse_.value = Some(other_synapses_struct.output);
															i += 1;
															continue 'outerest_loop;
														} 
														if (*synapse_).next.is_none(){
															let some_rc = Rc::new(RefCell::new(Node {
															    value: other_synapses_struct.output,
															    next: None,
															    prev: None,
															}));
															Node::link(y[0].end.clone(), &some_rc);
															y[0].end = (*(y[0].end)).unwrap().next;
															i += 1;
															continue 'outerest_loop;
														}
														synapse_ = Some(synapse_.next.unwrap());														
													}
													
												}else{
													*(first_nodelist_none.unwrap()).value	= Some(other_synapses_struct.output);
													i += 1;
													continue 'outerest_loop;
												}
											}
										}, OtherSynapses::ArraySynapse(array_synapse) => {
											if array_synapse.vector_neuron == synapse_{
												if first_nodelist_none == None{
													loop{
														if (*synapse_).value.is_none() {
															*synapse_.value = Some(other_synapses_struct.output);
															i += 1;
															continue 'outerest_loop;
														} 
														if (*synapse_).next.is_none(){
															let some_rc = Rc::new(RefCell::new(Node {
															    value: other_synapses_struct.output,
															    next: None,
															    prev: None,
															}));
															Node::link(y[0].end.clone(), &some_rc);
															y[0].end = (*(y[0].end)).unwrap().next;
															let some_rc_2 = Rc::new(RefCell::new(Node {
															    value: other_synapses_struct.output,
															    next: None,
															    prev: None,
															}));
															Node::link(y[1].end.clone(), &some_rc_2);
															y[1].end = (*(y[1].end)).unwrap().next;
															i += 1;
															continue 'outerest_loop;
														}
														synapse_ = (*synapse_).next.unwrap();														
													}
													
												}else{
													*(first_nodelist_none.unwrap()).value	= Some(other_synapses_struct.output);
													i += 1;
													continue 'outerest_loop;
												}
											}
										}, OtherSynapses::WriteSynapse(write_synapse) => {
											if write_synapse.number_neuron == synapse_{
												if first_nodelist_none == None{
													loop{
														if (*synapse_).value.is_none() {
															*synapse_.value = Some(other_synapses_struct.output);
															continue 'outerest_loop;
														} 
														if (*synapse_).next.is_none(){
															let some_rc = Rc::new(RefCell::new(Node {
															    value: other_synapses_struct.output,
															    next: None,
															    prev: None,
															}));
															Node::link(y[0].end.clone(), &some_rc);
															y[0].end = (*(y[0].end)).unwrap().next;
															let some_rc_2 = Rc::new(RefCell::new(Node {
															    value: other_synapses_struct.output,
															    next: None,
															    prev: None,
															}));
															Node::link(y[1].end.clone(), &some_rc_2);
															y[1].end = (*(y[1].end)).unwrap().next;
															continue 'outerest_loop;
														}
														synapse_ = (*synapse_).next.unwrap();														
													}
													
												}else{
													*(first_nodelist_none.unwrap()).value	= Some(other_synapses_struct.output);
													continue 'outerest_loop;
												}
											}
										}, _ => {
											break 'outer_loop;
										}
									}
								},
								None => {
									if first_nodelist_none.is_none() {
										first_nodelist_none = Some(Rc::new(RefCell::new(synapse_)));		
									} 
								}
							}
							if synapse_.next == None{
								break;
							}
							synapse_ = synapse_.next;
							
						}, Synapse::PurelyCardinalSynapse(purely_cardinal_synapse) => {
							match *synapse_2 {
								Some(x) => {
									if purely_cardinal_synapse.cardinal_neuron == *synapse_2.unwrap(){
										let some_rc = Rc::new(RefCell::new(Node {
										    value: purely_cardinal_synapse.cardinal_neuron,
										    next: None,
										    prev: None,
										}));
										Node::link(x[1].end.clone(), &some_rc);
										x[1].end = (*(x[1].end)).unwrap().next;
										continue 'outerest_loop; 								
									}
								}, None => {
									if first_nodelist_none_2.is_none(){
										first_nodelist_none_2 = Some(Rc::new(RefCell::new(synapse_2)));
									}
								}
							}
							if synapse_2.next == None{
								break;
							}
							synapse_2 = synapse_2.next;
							
						}, _ => {
							break 'outer_loop;
						}
					}
				}
			}
			}
		}
		match ((*synapse).into_inner()).value{
			Synapse::OtherSynapsesStruct(other_synapses_struct) => {
				let mut j = 0;		
				for patch in patches {
					for argument in patch.input{
						if argument == i{
							if nodelists[j].is_none(){
								let node = Node{value: other_synapses_struct.output, prev: nodelists[j].unwrap()[0].end, next: None};						
								let pointer = Rc::new(RefCell::new(node));
								nodelists[j] = Some([ NodeList{ length: 1, start: pointer, end: pointer}, NodeList{length: 0, start: None, end: None} ]);
							}else{
								let some_rc = Rc::new(RefCell::new(Node {
								    value: other_synapses_struct.output,
								    next: None,
								    prev: None,
								}));							
								Node::link(nodelists[j].unwrap()[0].end, &some_rc);
								nodelists[j].unwrap()[0].end = value.unwrap().next;					
							}
						}		
					}
					j += 1;
				}	
			}, _ => {
				
			}
		}
		execute_synapse(vector_memory, number_memory, cardinal_memory, &(*synapse).value);
		i += 1;							
		if ((*synapse).into_inner()).next.is_none() == false{
			synapse = ((*synapse).into_inner()).next.unwrap()
		}else{
			break;
		}		
	}
}
