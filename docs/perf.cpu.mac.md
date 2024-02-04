[src/main.rs:119] builder.num_gates() = 97678
[2024-02-04T11:16:04Z DEBUG plonky2::plonk::circuit_builder] Degree before blinding & padding: 105174
[2024-02-04T11:16:04Z DEBUG plonky2::plonk::circuit_builder] Degree after blinding & padding: 131072
4.3674s to preprocess
| 2.3668s to generate sigma polynomials
| 0.0238s to IFFT
| 0.2762s to FFT + blinding
| 0.1010s to transpose LDEs
| 1.1926s to build Merkle tree
[2024-02-04T11:16:08Z DEBUG plonky2::plonk::circuit_builder] Building circuit took 4.3674574s
start proof
6.5099s to root
| 0.5101s to run 342184 generators
| 0.1145s to compute full witness
| 0.0052s to compute wire polynomials
| 2.5562s to compute wires commitment
| | 0.0392s to IFFT
| | 0.3550s to FFT + blinding
| | 0.3286s to transpose LDEs
| | 1.7955s to build Merkle tree
| 0.0749s to compute partial products
| 0.5401s to commit to partial products, Z's and, if any, lookup polynomials
| | 0.0071s to IFFT
| | 0.0624s to FFT + blinding
| | 0.0369s to transpose LDEs
| | 0.4257s to build Merkle tree
| 1.9332s to compute quotient polys
| 0.0013s to split up quotient polys
| 0.3986s to commit to quotient polys
| | 0.0478s to FFT + blinding
| | 0.0193s to transpose LDEs
| | 0.3264s to build Merkle tree
| 0.0349s to construct the opening set, including lookups
| 0.3371s to compute opening proofs
| | 0.1924s to reduce batch of 259 polynomials
| | 0.0008s to reduce batch of 2 polynomials
| | 0.0672s to perform final FFT 1048576
| | 0.0496s to fold codewords in the commitment phase
| | 0.0095s to find proof-of-work witness
elapsed: 6.909505875s