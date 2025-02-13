use itertools::{chain, Itertools};
use serde::{Deserialize, Serialize};
use stwo_prover::{
    constraint_framework::TraceLocationAllocator,
    core::{
        backend::simd::SimdBackend,
        channel::Blake2sChannel,
        fields::m31::M31,
        pcs::{CommitmentSchemeProver, PcsConfig},
        poly::circle::{CanonicCoset, PolyOps},
        prover::prove,
        vcs::blake2_merkle::Blake2sMerkleChannel,
    },
    examples::state_machine::{
        components::{
            StateMachineComponents, StateMachineElements, StateMachineOp0Component,
            StateMachineOp1Component, StateMachineProof, StateMachineStatement0,
            StateMachineStatement1, StateTransitionEval, STATE_SIZE,
        },
        gen::{gen_interaction_trace, gen_trace},
    },
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProverOutput {
    proof: String,
    channel: String,
}
#[wasm_bindgen]
impl ProverOutput {
    #[wasm_bindgen(constructor)]
    pub fn new(proof: String, channel: String) -> ProverOutput {
        ProverOutput { proof, channel }
    }

    #[wasm_bindgen(getter)]
    pub fn proof(&self) -> String {
        self.proof.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn channel(&self) -> String {
        self.channel.clone()
    }
}
#[wasm_bindgen]
pub fn prove_state_machine(number_to_prove: u32) -> ProverOutput {
    let initial_state = [M31::from_u32_unchecked(number_to_prove); STATE_SIZE];
    let log_n_rows = 8;
    let row_x = 4;
    let row_y = 4;
    let mut channel = Blake2sChannel::default();
    let mut channel: &mut Blake2sChannel = &mut channel;
    let config = PcsConfig::default();
    let (x_axis_log_rows, y_axis_log_rows) = (row_x, row_y);
    // assert!(y_axis_log_rows >= LOG_N_LANES && x_axis_log_rows >= LOG_N_LANES);

    let mut intermediate_state = initial_state;
    intermediate_state[0] += M31::from_u32_unchecked(1 << x_axis_log_rows);
    let mut final_state = intermediate_state;
    final_state[1] += M31::from_u32_unchecked(1 << y_axis_log_rows);

    // Precompute twiddles.
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_n_rows + config.fri_config.log_blowup_factor + 1)
            .circle_domain()
            .half_coset,
    );
    // Setup protocol.
    let mut commitment_scheme =
        CommitmentSchemeProver::<_, Blake2sMerkleChannel>::new(config, &twiddles);

    // Trace.
    let trace_op0 = gen_trace(x_axis_log_rows, initial_state, 0);
    let trace_op1 = gen_trace(y_axis_log_rows, intermediate_state, 1);

    // Commitments.
    let tree_builder = commitment_scheme.tree_builder();
    tree_builder.commit(&mut channel);

    let stmt0 = StateMachineStatement0 {
        n: x_axis_log_rows,
        m: y_axis_log_rows,
    };
    stmt0.mix_into(channel);

    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(chain![trace_op0.clone(), trace_op1.clone()]);
    tree_builder.commit(channel);

    // Draw lookup element.
    let cloned_channel = channel.clone();
    let lookup_elements: StateMachineElements = StateMachineElements::draw(channel);

    // Interaction trace.
    let (interaction_trace_op0, claimed_sum_op0) =
        gen_interaction_trace(&trace_op0, 0, &lookup_elements);
    let (interaction_trace_op1, claimed_sum_op1) =
        gen_interaction_trace(&trace_op1, 1, &lookup_elements);

    let stmt1 = StateMachineStatement1 {
        x_axis_claimed_sum: claimed_sum_op0,
        y_axis_claimed_sum: claimed_sum_op1,
    };
    stmt1.mix_into(channel);

    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(chain![interaction_trace_op0, interaction_trace_op1].collect_vec());
    tree_builder.commit(channel);

    // Prove constraints.
    let tree_span_provider = &mut TraceLocationAllocator::default();
    let component0 = StateMachineOp0Component::new(
        tree_span_provider,
        StateTransitionEval {
            log_n_rows: x_axis_log_rows,
            lookup_elements: lookup_elements.clone(),
            claimed_sum: claimed_sum_op0,
        },
        claimed_sum_op0,
    );
    let component1 = StateMachineOp1Component::new(
        tree_span_provider,
        StateTransitionEval {
            log_n_rows: y_axis_log_rows,
            lookup_elements,
            claimed_sum: claimed_sum_op1,
        },
        claimed_sum_op1,
    );

    let components = StateMachineComponents {
        component0,
        component1,
    };
    let stark_proof = prove(&components.component_provers(), channel, commitment_scheme).unwrap();
    let proof = StateMachineProof {
        public_input: [initial_state, final_state],
        stmt0,
        stmt1,
        stark_proof,
    };
    let proof_string = serde_json::to_string(&proof).unwrap();
    let channel_string = serde_json::to_string(&cloned_channel).unwrap();
    ProverOutput::new(proof_string, channel_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use server::verify_state_machine;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;

    #[test]
    fn test_prove_and_verify() {
        let prover_output = prove_state_machine(1132);
        let proof_str = prover_output.proof();
        let channel = prover_output.channel();
        let proof: StateMachineProof<Blake2sMerkleHasher> =
            serde_json::from_str(&proof_str).unwrap();
        let result = proof.public_input[1][1];
        println!("Result: {}", result);

        let result = verify_state_machine(channel, proof_str);
        assert!(result.is_ok());
    }
}
