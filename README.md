# One Chain
Dead simple Blockchain library built for identity management
using following:
1. Ring Buffer for MU blocks on Block 'chain' called BlockRing Buffer.
2. Block Ring buffer is flushed to Memtable when full 
3. Memtable is implemented as a skiplist 
4. Memtable is flushed to disk using `mmap` (no or less IO) to SSTable.

### Basic Flows 

NOTE: Merkle tree build is optional and would not be part of MVP.
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
+----------------------------------------------------+
|                   SSTableSegment                   |
+----------------------+----------------------------+
| Bloom Filter        | Index Block                  |
| (For fast lookup)   | (Key → Offset mapping)       |
+----------------------+----------------------------+
|                     Data Block                     |
|  (Sorted key-value pairs, stored in sorted order)  |
+----------------------------------------------------+
|                  Metadata Block                    |
| (Compression, timestamps, merge info, etc.)        |
+----------------------------------------------------+
|                     Footer                         |
| (Magic number, version, checksum, etc.)            |
+----------------------------------------------------+
```

Data Block values:
```ascii
+-----------+-----------+--------------------+
| Key       | Offset    | Value              |
+-----------+-----------+--------------------+
| "apple"   | 0x2000    | "fruit"            |
| "banana"  | 0x2010    | "yellow fruit"     |
| "cherry"  | 0x2020    | "red fruit"        |
+-----------+-----------+--------------------+
```
	•	Keys are sorted lexicographically (e.g., "apple" < "banana" < "cherry").
	•	The Index Block maps keys to Data Block offsets.
	•	The Bloom Filter helps avoid unnecessary lookups.

### Novel way of exchanging Diffs for P2P data exchange

```mermaid
graph TD
    A[Client Requests Block] -->|Check in RingBuffer| B{Block in RingBuffer?}
    B -->|Yes| Z[Return Block]
    B -->|No| C{Block in MemTable?}
    C -->|Yes| Z
    C -->|No| D{Block in SSTable?}
    D -->|Yes| Z
    D -->|No| E[Exchange Cumulative Hash with Peer]

    E --> F[Compute XOR Diff]
    F --> G{Missing Blocks Detected?}
    G -->|No| H[Return Block Not Found]
    G -->|Yes| I[Download Missing Blocks]

    I --> J[Insert into RingBuffer]
    J --> K[Insert into MemTable]
    K --> L[Insert into SSTable]

    L --> M[Return Requested Block]
```