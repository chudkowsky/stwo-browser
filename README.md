# Stwo-Proof Login App

## Idea

The application allows users to log in or register using a custom hashing-based proof system (`stwo`). The process involves:

1. **Client Input**: The user enters a login and password.
2. **Hashing Function**: The input is sent to a prover which hashes it using the function `f(x) = x^5`.
3. **Verification**: The client proves they know the input to the hash function that matches the hash stored in the database.

---

## Current state:
The user enters a username and password, where the password is first hashed into a numerical value. This hashed value is then used as input for a zero-knowledge proof generation function. The function runs a predefined computation that increments the hashed password 2⁸ times and produces a proof along with the final output. The proof is then verified for correctness. If the computed output already exists in the database, the user is successfully authenticated. Otherwise, a new user entry is created with the calculated password and associated username.