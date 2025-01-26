# One Chain
Dead simple Blockchain library built on top of rocksdb.

### First Attempt:

```mermaid
graph TD
    A[Ring Buffer] -->|New Block| B[Bitmap Tracking]
    A -->|New Block| C[Merkle Tree]
    B -->|Block Presence| D[Bitmap Operations]
    C -->|Verification| E[Merkle Proof]
    B -->|Sync Mechanism| F[Peer Bitmap Comparison]
    F -->|Resolve Gaps| G[Block Sync Logic]

    subgraph Ring Buffer
        RB1[Pre-allocated Block Array]
        RB2[Tail Index Management]
        RB3[Circular Buffer Mechanics]
        RB1 --> RB2
        RB2 --> RB3
    end

    subgraph Bitmap Operations
        BO1[Set Bit]
        BO2[Clear Bit]
        BO3[Check Bit]
        BO4[XOR Sync]
    end

    subgraph Merkle Tree
        MT1[Build from Blocks]
        MT2[Hash Nodes]
        MT3[Root Hash]
        MT4[Verification Proofs]
        MT1 --> MT2
        MT2 --> MT3
        MT3 --> MT4
    end
```
### Storage of smaller groups and smaller blockchains
A small block chain is usually referring to < 10,000 members
or blocks, which is ideal for private blockchains, data
is serialized using mmap and is faster.

```ascii 
+-------------------------------+
| Header (128 bytes)            |
| - Capacity (usize)            |
| - Head (usize)                |
| - Tail (usize)                |
| - Cumulative Hash ([u8; 16])  |
| - Bitmap ([u8; 128])          |
+-------------------------------+
| Block 0 (32 bytes)            |
| Block 1 (32 bytes)            |
| ...                           |
| Block N (32 bytes)            |
+-------------------------------+
```