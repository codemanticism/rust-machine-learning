use rand::prelude::*;
pub use crate::synapse_network::*;
pub static simplicity_weight = 0;
pub static accuracy_weight = 0;
enum Enum{
	StartTheSearchFromWhere([usize; usize]),
	NoSearch(usize)
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
fn function(synapse_network: &mut [Synapse], start_from_where: Enum, how_many_to_go: mut u32, vector_of_bools: &mut Vec<usize>){
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
fn train(minimum_score: u32, synapse_network: &mut [Synapse]){
	let mut variables = Vec<bool>;
	while calculate_score() < minimum_score{
		let mut number_vector: Vec<f64> = Vec::New();
		let mut option: Some(SynapseID) = Some(SynapseID{
			synapse_index: 0 as usize,
			synapse_network: synapse_network
		});
		let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap(); 
		list_of_vector_memories.push(Vec::New());
		run_synapse_network(synapse_network, list_of_vector_memories.len() - 1, &mut number_vector, &mut option);
		match option {
			Option::Some => {
				function(option.synapse_network, Enum::NoSearch(option.synapse_index), generate_random_number())
			}, Option::None => {
				error!("QUIT");				
			}
		}
		drop(list_of_vector_memories);	
						
	}
}
