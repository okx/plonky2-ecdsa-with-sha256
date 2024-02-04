start proof
invoking intt_batch, total_nums: 136, log_n: 17, num_gpus: 4
invoking ntt_batch, total_nums: 136, log_n: 20, num_gpus: 4
invoking intt_batch, total_nums: 20, log_n: 17, num_gpus: 4
invoking ntt_batch, total_nums: 20, log_n: 20, num_gpus: 4
invoking ntt_batch, total_nums: 16, log_n: 20, num_gpus: 4
3.7730s to root
| 0.6636s to run 342191 generators
| 0.1065s to compute full witness
| 0.0061s to compute wire polynomials
| 1.6877s to compute wires commitment
| | 0.1058s to IFFT
| | 0.5988s to FFT + blinding
| | 0.6207s to transpose LDEs
| | 0.3551s to build Merkle tree
| 0.0653s to compute partial products
| 0.1676s to commit to partial products, Z's and, if any, lookup polynomials
| | 0.0080s to IFFT
| | 0.0523s to FFT + blinding
| | 0.0047s to transpose LDEs
| | 0.0890s to build Merkle tree
| 0.4700s to compute quotient polys
| 0.0054s to split up quotient polys
| 0.1389s to commit to quotient polys
| | 0.0527s to FFT + blinding
| | 0.0040s to transpose LDEs
| | 0.0706s to build Merkle tree
| 0.0161s to construct the opening set, including lookups
| 0.4434s to compute opening proofs
| | 0.3068s to reduce batch of 259 polynomials
| | 0.0023s to reduce batch of 2 polynomials
| | 0.0931s to perform final FFT 1048576
| | 0.0228s to fold codewords in the commitment phase
| | 0.0024s to find proof-of-work witness
elapsed: 4.219504467s