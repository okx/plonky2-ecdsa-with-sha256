cargo run --release --features=cuda,parallel
gpus=4
[src/main.rs:119:5] builder.num_gates() = 97678
[2024-02-04T10:21:37Z DEBUG plonky2::plonk::circuit_builder] Degree before blinding & padding: 105174
[2024-02-04T10:21:37Z DEBUG plonky2::plonk::circuit_builder] Degree after blinding & padding: 131072
[2024-02-04T10:21:45Z DEBUG plonky2::plonk::circuit_builder] Building circuit took 8.867609s
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] 5.1419s to root
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.5970s to run 342185 generators
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.1057s to compute full witness
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.0057s to compute wire polynomials
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 2.7752s to compute wires commitment
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.6980s to FFT + blinding
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 1.2526s to transpose LDEs
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.7277s to build Merkle tree
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.0443s to compute partial products
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.2542s to commit to partial products, Z's and, if any, lookup polynomials
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0700s to FFT + blinding
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0080s to transpose LDEs
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.1632s to build Merkle tree
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.7206s to compute quotient polys
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.0026s to split up quotient polys
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.2191s to commit to quotient polys
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0856s to FFT + blinding
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0066s to transpose LDEs
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.1224s to build Merkle tree
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.0153s to construct the opening set, including lookups
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | 0.3965s to compute opening proofs
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.2783s to reduce batch of 259 polynomials
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0021s to reduce batch of 2 polynomials
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0781s to perform final FFT 1048576
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0245s to fold codewords in the commitment phase
[2024-02-04T10:21:51Z DEBUG plonky2::util::timing] | | 0.0047s to find proof-of-work witness

cargo run --release --features=parallel
src/main.rs:119:5] builder.num_gates() = 97678
[2024-02-04T10:25:50Z DEBUG plonky2::plonk::circuit_builder] Degree before blinding & padding: 105173
[2024-02-04T10:25:50Z DEBUG plonky2::plonk::circuit_builder] Degree after blinding & padding: 131072
[2024-02-04T10:25:57Z DEBUG plonky2::plonk::circuit_builder] Building circuit took 6.9702716s
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] 3.9838s to root
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.5911s to run 342182 generators
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.1063s to compute full witness
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.0026s to compute wire polynomials
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 1.6536s to compute wires commitment
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0167s to IFFT
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.3689s to FFT + blinding
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.5303s to transpose LDEs
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.7328s to build Merkle tree
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.0438s to compute partial products
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.2283s to commit to partial products, Z's and, if any, lookup polynomials
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0056s to IFFT
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0519s to FFT + blinding
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0051s to transpose LDEs
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.1617s to build Merkle tree
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.7065s to compute quotient polys
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.0029s to split up quotient polys
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.1821s to commit to quotient polys
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0505s to FFT + blinding
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0040s to transpose LDEs
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.1240s to build Merkle tree
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.0163s to construct the opening set, including lookups
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | 0.4469s to compute opening proofs
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.3333s to reduce batch of 259 polynomials
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0020s to reduce batch of 2 polynomials
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0769s to perform final FFT 1048576
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0232s to fold codewords in the commitment phase
[2024-02-04T10:26:01Z DEBUG plonky2::util::timing] | | 0.0020s to find proof-of-work witness