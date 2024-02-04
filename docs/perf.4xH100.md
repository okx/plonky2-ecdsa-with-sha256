## CPU with NTT on GPU

num of gpus: 4
[src/main.rs:119:5] builder.num_gates() = 97678
[2024-02-04T11:07:56Z DEBUG plonky2::plonk::circuit_builder] Degree before blinding & padding: 105174
[2024-02-04T11:07:56Z DEBUG plonky2::plonk::circuit_builder] Degree after blinding & padding: 131072
invoking intt_batch, total_nums: 87, log_n: 17, num_gpus: 4
invoking ntt_batch, total_nums: 87, log_n: 20, num_gpus: 4
8.9948s to preprocess
| 6.9583s to generate sigma polynomials
| 0.0637s to IFFT
| 0.4338s to FFT + blinding
| 0.7869s to transpose LDEs
| 0.2485s to build Merkle tree
[2024-02-04T11:08:05Z DEBUG plonky2::plonk::circuit_builder] Building circuit took 8.994918s
start proof
invoking intt_batch, total_nums: 136, log_n: 17, num_gpus: 4
invoking ntt_batch, total_nums: 136, log_n: 20, num_gpus: 4
invoking intt_batch, total_nums: 20, log_n: 17, num_gpus: 4
invoking ntt_batch, total_nums: 20, log_n: 20, num_gpus: 4
invoking ntt_batch, total_nums: 16, log_n: 20, num_gpus: 4
3.8751s to root
| 0.6636s to run 342185 generators
| 0.1048s to compute full witness
| 0.0068s to compute wire polynomials
| 1.6568s to compute wires commitment
| | 0.1159s to IFFT
| | 0.5988s to FFT + blinding
| | 0.5732s to transpose LDEs
| | 0.3649s to build Merkle tree
| 0.0648s to compute partial products
| 0.2426s to commit to partial products, Z's and, if any, lookup polynomials
| | 0.0090s to IFFT
| | 0.1054s to FFT + blinding
| | 0.0106s to transpose LDEs
| | 0.1056s to build Merkle tree
| 0.4808s to compute quotient polys
| 0.0058s to split up quotient polys
| 0.1879s to commit to quotient polys
| | 0.0912s to FFT + blinding
| | 0.0039s to transpose LDEs
| | 0.0807s to build Merkle tree
| 0.0195s to construct the opening set, including lookups
| 0.4392s to compute opening proofs
| | 0.3021s to reduce batch of 259 polynomials
| | 0.0023s to reduce batch of 2 polynomials
| | 0.0939s to perform final FFT 1048576
| | 0.0239s to fold codewords in the commitment phase
| | 0.0006s to find proof-of-work witness
elapsed: 4.321696324s
