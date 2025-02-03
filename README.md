# Stwo-Proof Login App

## Idea

The application allows users to log in or register using a custom hashing-based proof system (`stwo`). The process involves:

1. **Client Input**: The user enters a login and password.
2. **Hashing Function**: The input is sent to a prover which hashes it using the function `f(x) = x^5`.
3. **Verification**: The client proves they know the input to the hash function that matches the hash stored in the database.

The app will be built using **SvelteKit** and will include a service worker to spawn additional threads for running the prover. The app will also explore running the prover across multiple threads.

---

## Roadmap for Client-Side Proving

### 1. **Initialize Project**
   - Set up a new **SvelteKit** project.
   - Install required dependencies.

### 2. **Minimal GUI**
   - Create a basic user interface for login and registration.
   - Include fields for username and password.

### 3. **WASM Minimal Example**
   - Integrate WebAssembly (WASM) into the project.
   - Test a basic example to ensure WASM integration works.

### 4. **Simple Constraints**
   - Define and implement simple constraints for the proof.
   - Test constraint generation.

### 5. **WASM Stwo**
   - Implement the `stwo` hashing function (`f(x) = x^5`) using WebAssembly.

### 6. **Proof Generation**
   - Build the functionality to generate proofs using the hashing function and constraints.
   - Ensure compatibility with WebAssembly.

### 7. **Proof Verification**
   - Implement proof verification logic on the client side.
   - Verify that proofs match the hash stored in the database.

### 8. **Full Pipeline**
   - Integrate all components into a complete login and registration pipeline.
   - Optimize performance and ensure the prover runs efficiently across multiple threads using the service worker.

---

## Notes
- The hashing function `f(x) = x^5` is chosen for simplicity and can be adjusted later. (sigma poseidon)
- Explore WebAssembly threading capabilities and service workers for scaling proof generation.
- Ensure all cryptographic and hashing operations are secure and efficient.
