use stwo_prover::{
    constraint_framework::{Relation, TraceLocationAllocator},
    core::{
        channel::Blake2sChannel,
        fields::qm31::QM31,
        pcs::CommitmentSchemeVerifier,
        prover::{verify, VerificationError},
        vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
    },
    examples::state_machine::components::{
        StateMachineComponents, StateMachineElements, StateMachineOp0Component,
        StateMachineOp1Component, StateMachineProof, StateTransitionEval,
    },
};

pub fn verify_state_machine(
    prover_channel: String,
    proof: String,
) -> Result<u32, VerificationError> {
    let channel = &mut Blake2sChannel::default();
    let mut prover_channel: Blake2sChannel = serde_json::from_str(&prover_channel).unwrap();
    let proof: StateMachineProof<Blake2sMerkleHasher> = serde_json::from_str(&proof).unwrap();
    std::fs::write(format!("proof_{}.json",proof.public_input[1][1]), serde_json::to_string(&proof).unwrap()).unwrap();
    let lookup_elements: StateMachineElements = StateMachineElements::draw(&mut prover_channel);

    let tree_span_provider = &mut TraceLocationAllocator::default();

    let x_axis_log_rows = proof.stmt0.n;
    let y_axis_log_rows = proof.stmt0.m;
    let claimed_sum_op0 = proof.stmt1.x_axis_claimed_sum;
    let claimed_sum_op1 = proof.stmt1.y_axis_claimed_sum;

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
    let commitment_scheme =
        &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(proof.stark_proof.config);
    // Decommit.
    // Retrieve the expected column sizes in each commitment interaction, from the AIR.
    let sizes = proof.stmt0.log_sizes();

    // Preprocessed columns.
    commitment_scheme.commit(proof.stark_proof.commitments[0], &sizes[0], channel);
    // Trace columns.
    proof.stmt0.mix_into(channel);
    commitment_scheme.commit(proof.stark_proof.commitments[1], &sizes[1], channel);

    // Assert state machine statement.
    let lookup_elements = StateMachineElements::draw(channel);
    let initial_state_comb: QM31 = lookup_elements.combine(&proof.public_input[0]);
    let final_state_comb: QM31 = lookup_elements.combine(&proof.public_input[1]);
    assert_eq!(
        (proof.stmt1.x_axis_claimed_sum + proof.stmt1.y_axis_claimed_sum)
            * initial_state_comb
            * final_state_comb,
        final_state_comb - initial_state_comb
    );

    // Interaction columns.
    proof.stmt1.mix_into(channel);
    commitment_scheme.commit(proof.stark_proof.commitments[2], &sizes[2], channel);
    if verify(
        &components.components(),
        channel,
        commitment_scheme,
        proof.stark_proof,
    ).is_ok(){
        Ok(proof.public_input[1][1].0)
    }else{
        Err(VerificationError::ProofOfWork)
    }
    

}
