start proof
invoking intt_batch, total_nums: 136, log_n: 17, num_gpus: 1
invoking ntt_batch, total_nums: 136, log_n: 20, num_gpus: 1
invoking intt_batch, total_nums: 20, log_n: 17, num_gpus: 1
invoking ntt_batch, total_nums: 20, log_n: 20, num_gpus: 1
invoking ntt_batch, total_nums: 16, log_n: 20, num_gpus: 1
5.5226s to root
| 0.6598s to run 342185 generators
| 0.1090s to compute full witness
| 0.0051s to compute wire polynomials
| 2.9564s to compute wires commitment
| | 0.1582s to IFFT
| | 1.8659s to FFT + blinding
| | 0.5645s to transpose LDEs
| | 0.3644s to build Merkle tree
| 0.0691s to compute partial products
| 0.4322s to commit to partial products, Z's and, if any, lookup polynomials
| | 0.0123s to IFFT
| | 0.2845s to FFT + blinding
| | 0.0221s to transpose LDEs
| | 0.1034s to build Merkle tree
| 0.5036s to compute quotient polys
| 0.0055s to split up quotient polys
| 0.3208s to commit to quotient polys
| | 0.2315s to FFT + blinding
| | 0.0039s to transpose LDEs
| | 0.0769s to build Merkle tree
| 0.0196s to construct the opening set, including lookups
| 0.4394s to compute opening proofs
| | 0.3023s to reduce batch of 259 polynomials
| | 0.0023s to reduce batch of 2 polynomials
| | 0.0961s to perform final FFT 1048576
| | 0.0225s to fold codewords in the commitment phase
| | 0.0005s to find proof-of-work witness
elapsed: 5.950371537s