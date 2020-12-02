## Blockchain Design

There's probably lots of ways to tackle this design but it seems like the simplest implementation is fine and is just

The block "chain" is really just an append-only linked list of block with some extra properties

- #### Blocks
	- Blocks are just nodes that hold some data:
		- unique block id (address). This can be anything that is serializable (e.g. JSON).
		- the list of all prior transactions. This could be their hashes, or IDs, or something else (not totally clear on how to go about this).
		- timestamp
		- the hash of the last block

## Necessary Components

1. Core block node/chain code with [Merkle tree](https://www.codementor.io/blog/merkle-trees-5h9arzd3n8)
2. Consensus Mechanism (most likely PoW)
3. API for the actual distributed system node network

## (Most Likely) Development Roadmap

Obviously the first couple of stages are going to be research but from a 10,000ft glance it seems like a good path would be:

Block/blockchain code (including merkle tree) -> consensus mechanic -> distributed network (API) code

Subject to change (obviously!) but this seems like a fine enough starting point.
