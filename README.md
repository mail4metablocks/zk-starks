# zk-starks

zk-STARKs (Zero-Knowledge Scalable Transparent ARguments of Knowledge) are a type of zero-knowledge proof system that allows one party (the prover) to prove to another party (the verifier) that a statement is true, without revealing any information about the statement itself. They are an improvement over zk-SNARKs (Zero-Knowledge Succinct Non-Interactive ARguments of Knowledge), as they are more scalable, transparent, and secure.

zk-STARKs use a different approach to constructing zero-knowledge proofs than zk-SNARKs. Instead of using a mathematical structure called a "commitment scheme," they use a technique called "friable functions," which allows them to be more efficient and transparent.

zk-STARKs have several properties that make them appealing for use in various applications:

    Scalability: zk-STARKs can prove statements about large datasets efficiently, without requiring a trusted setup.

    Transparency: zk-STARKs do not require a trusted setup, and the proof construction and verification process is publicly verifiable.

    Security: zk-STARKs are based on strong cryptographic assumptions and have been proven to be secure against various attacks.

zk-STARKs have been used in various applications, such as privacy-preserving transactions, verifiable computations, and secure multi-party computation.

## libraries for zk-Starks

There are several libraries available for implementing zk-STARKs (Zero-Knowledge Scalable Transparent ARguments of Knowledge) in Rust, including:

    starkware-rs: This is a Rust implementation of zk-STARKs that is based on the STARKware library. It provides a high-level API for generating and verifying zk-STARKs, and it supports multiple cryptographic hash functions and field sizes.

    stark-rs: This is another Rust implementation of zk-STARKs that is based on the STARKware library. It provides a lower-level API for working with zk-STARKs, and it supports custom circuit implementations and proof serialization.

    curve25519-dalek: This is a Rust library for working with the curve25519 elliptic curve, which is used in the construction of zk-STARKs. It provides various functions for working with curve25519 points, scalars, and other data types.

    bls12_381: This is a Rust library for working with the bls12-381 elliptic curve, which is another curve that is used in the construction of zk-STARKs. It provides various functions for working with bls12-381 points, scalars, and other data types.

These libraries can be used to implement zk-STARKs in Rust for various applications, such as privacy-preserving transactions, verifiable computations, and secure multi-party computation. You can find more detailed documentation and examples in the documentation of each library.
