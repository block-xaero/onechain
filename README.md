# One Chain
Dead simple Blockchain library built for identity management
using following:
1. Ring Buffer for MU blocks on Block 'chain' called BlockRing Buffer.
2. Block Ring buffer is flushed to Memtable when full 
3. Memtable is implemented as a skiplist 
4. Memtable is flushed to disk using `mmap` (no or less IO) to SSTable.

### First Attempt:

```mermaid

graph TD
    A[Ring Buffer] -->|New Block| B[Bitmap Tracking]
    A -->|New Block| C[Memtable]
    C -->|Flush to Disk| D[SSTable]
    D -->|Indexed by| E[Merkle Tree]
    B -->|Sync Mechanism| F[Peer Bitmap Comparison]
    F -->|Resolve Gaps| G[Block Sync Logic]

    subgraph Ring Buffer
        RB1[Circular Block Storage]
        RB2[Constant-Time Append]
        RB3[Fixed-Size Allocation]
        RB1 --> RB2
        RB2 --> RB3
    end

    subgraph Bitmap Operations
        BO1[Block Presence Tracking]
        BO2[Efficient Set/Clear]
        BO3[Fast Lookup]
        BO4[Peer Synchronization]
    end

    subgraph Memtable
        MT1[In-Memory Write Buffer]
        MT2[Concurrent Updates]
        MT3[Write Optimization]
        MT1 --> MT2
        MT2 --> MT3
    end

    subgraph SSTable
        ST1[Immutable Data Segments]
        ST2[Tiered Compaction]
        ST3[Persistent Storage]
        ST1 --> ST2
        ST2 --> ST3
    end

    subgraph Merkle Tree
        MT4[Efficient Key Indexing]
        MT5[Cryptographic Hashing]
        MT6[Integrity Verification]
        MT7[Lightweight Proofs]
        MT4 --> MT5
        MT5 --> MT6
        MT6 --> MT7
    end
```
### Storage of smaller groups and smaller blockchains
A small block chain is usually referring to < 10,000 members
or blocks, which is ideal for private blockchains, data
is serialized using mmap and is faster.

#### Proposed Memory-Mapped IO Design

#### File Layout

#### A single file will contain:
	1.	Header: Metadata about the ring buffer (e.g., capacity, head, tail, cumulative hash).
	2.	Blocks: Fixed-size blocks representing members.

```ascii 
+-------------------------------+
| Header (256 bytes)            |
| - Capacity (usize)            | Total number of blocks in the file
| - Head (usize)                | Index of the head block
| - Tail (usize)                | Index of the tail block
| - Cumulative Hash ([u8; 16])  | XOR of all block hashes for chain state
| - Bitmap ([u8; 128])          | Block presence bitmap
| - SSTable Version (u32)       | For compatibility across versions
| - Timestamp (u64)             | Last write timestamp (UTC)
| - Reserved Space (40 bytes)   | Reserved for future use
+-------------------------------+
| Block Index (256 bytes)       | Metadata for quick navigation
| - Block 0 Offset (u32)        | Byte offset of Block 0
| - Block 1 Offset (u32)        | Byte offset of Block 1
| - ...                         |
| - Block N Offset (u32)        |
+-------------------------------+
| Block Data Section            | Actual block data
| Block 0 (64 bytes)            | Metadata and content
| - Block Hash ([u8; 16])       | Hash of block data
| - Data Length (u16)           | Length of the content
| - Data (variable, up to 48B)  | Actual block data
| Block 1 (64 bytes)            |
| ...                           |
| Block N (64 bytes)            |
+-------------------------------+
| Merkle Tree Root ([u8; 32])   | Cryptographic root hash
+-------------------------------+
```

### Quick Review of QUIC (used in syncing the blockchain buffers)

```mermaid
sequenceDiagram
    participant Peer A (Requester)
    participant Peer B (Responder)
    participant QUIC Transport Layer
    
    Note over Peer A (Requester): Initiates sync request
    Peer A (Requester) ->> QUIC Transport Layer: Establish QUIC connection
    QUIC Transport Layer ->> Peer B (Responder): Request connection
    Peer B (Responder) ->> QUIC Transport Layer: Accept connection

    Note over Peer A (Requester), Peer B (Responder): Metadata exchange
    Peer A (Requester) ->> Peer B (Responder): Request file metadata (file hash, chunk list)
    Peer B (Responder) ->> Peer A (Requester): Respond with metadata (chunk hashes, sizes)

    Note over Peer A (Requester): Request missing chunks
    Peer A (Requester) ->> Peer B (Responder): Request missing chunks (e.g., Chunk 1, 3, 5)

    Note over QUIC Transport Layer: Multiplexing streams for chunks
    Peer B (Responder) ->> QUIC Transport Layer: Send Chunk 1 on Stream 1
    Peer B (Responder) ->> QUIC Transport Layer: Send Chunk 3 on Stream 2
    Peer B (Responder) ->> QUIC Transport Layer: Send Chunk 5 on Stream 3

    QUIC Transport Layer ->> Peer A (Requester): Deliver Chunk 1 on Stream 1
    QUIC Transport Layer ->> Peer A (Requester): Deliver Chunk 3 on Stream 2
    QUIC Transport Layer ->> Peer A (Requester): Deliver Chunk 5 on Stream 3

    Note over Peer A (Requester): Chunk validation
    Peer A (Requester) ->> Peer A (Requester): Validate Chunk 1 (hash match)
    Peer A (Requester) ->> Peer A (Requester): Validate Chunk 3 (hash match)
    Peer A (Requester) ->> Peer A (Requester): Validate Chunk 5 (hash match)

    Note over Peer A (Requester), Peer B (Responder): Sync completion
    Peer A (Requester) ->> Peer B (Responder): Acknowledge receipt of chunks
    Peer B (Responder) ->> Peer A (Requester): Acknowledgment confirmed

    Note over Peer A (Requester): File reconstruction
```

