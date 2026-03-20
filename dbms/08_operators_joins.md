# Joins

- sort merge join
- hash join (works for equijoins)
    + simple
    + grace
    + hybrid
- nested loop join
    + simple (SNLJ)
    + page (PNLJ)
    + block (BNLJ) (works on non-equal joins)
    + index (INLJ)

nested loop join animation can be found [here](https://cs186.gitbook.io/project/common/misc/nested-loop-join-animations)

## Costs
1. NLJ
    1. most common NLJ algo - BNLJ:

        assume R is outer relation, S is inner relation. The cost is $ N_r + ceil(\frac{N_r}{b - 2}) \times N_s $ where $ N_s $ is number of pages in inner relation and $ N_r $ is number of pages in outer relation.

        the idea is to parition R into b-2 blocks and load b-2 blocks into memory, leave 1 buffer for S to scan each page of S and one for output.

    2. index
        similar to BNLJ (or any other NLJ algorithm), a outer loop for the first relation `R` and inner loop for another relation `S`, in the nested loop look up phase, instead of checking the equality of $ r_i $ vs $ s_i $, the algorithm put $ r_i $ into relation `S`'s index for look up.

        the cost is different between clustered index and unclustered index.

        clustered index is ordered by distinct values, so it only needs to find the beginning of a value and do sequential scan if needed, the heap access is smaller than unclustered index.


    3. others

        they are not as efficient as BNLJ and INLJ ...


2. hash join

    1. simple

        same assumption above, the cost = $ N_r + N_s * \#hashes $

        partition R into buckets, hash tuples of R to get $ b - 3 $ buckets, load a page of R into buffer, hash each tuple, distribute into buckets, load a page of S into buffer, as long as there is 1 full bucket, hash tuples of S in buffer on join key, check each tuple in S buffer and R's corresponding hash bucket, if match add to output. After processing, clean R's hash table and redo it until no more tuples to check in R and load next page of S into buffer and do it again.

        if data skew, simple hash join might be having more io than BNLJ, assume memory is big enough for BNLJ to load R into buffer or hash function is not efficient enough like overflow that needs to recompute the join for overflow hash.

    2. grace

    3. hybrid

        a more advanced way combined classical HJ and GHJ... hard to implement correctly...


3. sort merge join

## join in postgres
implementations are under `src/backend/executor`
- `nodeNestloop.c`
- `nodeMergejoin.c`
- `nodeHashjoin.c`

there is no best join algorithm but query optimiser chooses the most efficient one by calculating the cost.