/* tslint:disable */
/* eslint-disable */
export function prove_state_machine(number_to_prove: number): ProverOutput;
export class ProverOutput {
  free(): void;
  constructor(proof: string, channel: string);
  readonly proof: string;
  readonly channel: string;
}
