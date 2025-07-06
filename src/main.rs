mod synapse_network;
pub use crate::synapse_network::*;

fn main(){
	let synapse1 = Synapse::OtherSynapsesStruct(
	        OtherSynapsesStruct {
	            output: 0,
	            kind: OtherSynapses::ConstantSynapse(
	                ConstantSynapse {
	                    constant: 1.0,
	                }
	            ),
	        }
	    );
	let synapse2 = Synapse::OtherSynapsesStruct(
	        OtherSynapsesStruct {
	            output: 0,
	            kind: OtherSynapses::TwoInputSynapse(
	                TwoInputSynapse {
                    	input_1: 0,
                    	input_2: 0,
                    	operation: TwoInputSynapseTypes::Add,
	                }
	            ),
	        }
	    );
	let chain_of_synapses = vec![synapse1, synapse2];
	let mut number_vector: Vec<f64> = vec![0_f64];
	let mut list_of_vector_memories = LIST_OF_VECTOR_MEMORIES.lock().unwrap();
	list_of_vector_memories.push(vec![vec![]]);	
	println!("$");
	drop(list_of_vector_memories);
	match run_synapse_network(&chain_of_synapses, 0_usize, &mut number_vector){
		KindsOfNeurons::Scalar(scalar) => {
			println!("It is the number {}.", scalar);
		},
		KindsOfNeurons::Vector(_) => {
			println!("It is a vector.");
		} 
	}	
}
