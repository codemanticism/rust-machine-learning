fn if_constant_synapse(synapse: &mut Synapse) -> Option<f64>{
	match synapse{
		OtherSynapsesStruct(other_synapses_struct) => {
			match other_synapses_struct.kind {
				ConstantSynapse(constant_synapse) => {
					Some(constant_synapse.constant)
				},
				_ => {
					None
				}
			}
		},
		LoopSynapse(_) => {
			None
		}
	}	
}
fn if_two_input_synapse(synapse: &mut Synapse) -> Option<TwoInputSynapse>{
	match synapse{
		OtherSynapsesStruct(other_synapses_struct) => {
			match other_synapses_struct.kind {
				TwoInputSynapse(two_input_synapse) => {
					Some(two_input_synapse)
				},
				_ => {
					None
				}
			}
		},
		LoopSynapse(_) => {
			None
		}
	}	
}
fn if_one_input_synapse(synapse: &mut Synapse) -> Option<OneInputSynapse>{
	match synapse{
		OtherSynapsesStruct(other_synapses_struct) => {
			match other_synapses_struct.kind {
				OneInputSynapse(one_input_synapse) => {
					Some(one_input_synapse)
				},
				_ => {
					None
				}
			}
		},
		LoopSynapse(_) => {
			None
		}
	}	
}
fn if_not_loop_synapse(synapse: &mut Synapse) -> Option<usize>{
	match synapse{
		OtherSynapsesStruct(other_synapses_struct) => {
			Some(other_synapses_struct.output)
		},
		LoopSynapse(_) => {
			None
		}
	}	
}

fn if_cardinal_synapse(synapse: &mut Synapse) -> Option<usize>{
	match synapse{
		PurelyCardinalSynapse(purely_cardinal_synapse) => {
			purely_cardinal_synapse.output_neuron
		}, _ => {
			None
		}
	}	
}
