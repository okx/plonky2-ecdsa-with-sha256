num of gpus: 1
invoking intt_batch, total_nums: 87, log_n: 17, num_gpus: 1
27.5937s to preprocess
| 13.8498s to generate sigma polynomials
| 0.1352s to IFFT
| 2.0469s to FFT + blinding
| 2.2724s to transpose LDEs
| 8.2233s to build Merkle tree
start proof
invoking intt_batch, total_nums: 136, log_n: 17, num_gpus: 1
invoking intt_batch, total_nums: 20, log_n: 17, num_gpus: 1
36.8431s to root
| 1.2702s to run 342188 generators
| 0.3357s to compute full witness
| 0.0239s to compute wire polynomials
| 17.3851s to compute wires commitment
| | 0.3318s to IFFT
| | 2.9553s to FFT + blinding
| | 1.7548s to transpose LDEs
| | 12.2961s to build Merkle tree
| 0.1865s to compute partial products
| 3.4951s to commit to partial products, Z's and, if any, lookup polynomials
| | 0.0283s to IFFT
| | 0.5659s to FFT + blinding
| | 0.1161s to transpose LDEs
| | 2.7765s to build Merkle tree
| 10.4056s to compute quotient polys
| 0.0063s to split up quotient polys
| 2.5404s to commit to quotient polys
| | 0.4018s to FFT + blinding
| | 0.0493s to transpose LDEs
| | 2.0802s to build Merkle tree
| 0.0851s to construct the opening set, including lookups
| 1.0993s to compute opening proofs
| | 0.5083s to reduce batch of 259 polynomials
| | 0.0041s to reduce batch of 2 polynomials
| | 0.1633s to perform final FFT 1048576
| | 0.2602s to fold codewords in the commitment phase
| | 0.1489s to find proof-of-work witness
elapsed: 37.559068272s
proof size: 0
