# Rusty Merkle Tree

A Merkle Tree implementation in Rust.

## Design

The key aspect to understand of this implementation is that leaves length will ALWAYS be a power of 2 (2^n), padded with zero-byte streams at the end if the actual non-zero leaf count is other than a power of 2.

Consider the following situation which is an attempt to build a root from an array of 6 elements:

```
   [root]
    /  \
  ab    cd    ef  ← Unpaired hash
 /  \  /  \  /  \ 
[a, b, c, d, e, f]
```

We can't compute the root smoothly without performing some kind of trick, this is not a big issue, in fact, there are many solutions to handle this scenario (i.e in Bitcoin Core these odd nodes are cloned and paired together with their own copies to generate a hash for the next level).

And because this is a quick project and there's not a specific usecase in mind for this MerkleTree I opted for just setting the leaf count to a power of 2
(whether at creation or insertions) and fill gaps with zeroes, this way further operations should run fluently without facing a scenario where a level of the tree has odd leaf count.

```
      [Merkle Root]
       /        \
    abcd        ef00
    /  \        /  \
  ab    cd    ef    00
 /  \  /  \  /  \  /  \
[a, b, c, d, e, f, 0, 0]
 |--------------|  |--|
 non-zero leaves    ^^ padding
```

Since 6 is not a power of 2 the tree will be expanded to the next power of 2 which is 8 and pad with zeroes the remaining space if any.

## Build

```bash
make build
```

## Test

```bash
make test
```

## Lint

```bash
make lint
```
