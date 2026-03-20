# Indexing

## 1-d index and 2-d index 
based on value of single attr A.
- may be used to sort data file
- values may be unique

indexes can be constructed in different ways
- dense: every tuple is reference by an entry in the index file
- sparse: only some tuples are referenced by an entry in the index file
- single level: tuples are accessed directly from index file
- multi level: tuples are assessed via several index pages to reach tuple

delete index is cheap, but update it (including removing it) is expensive as it needs to move the index seqeunce and keep the ordering. can only remove the index on key when all related tuples on key are gone.


clustered index
- to build a clustered index, first sort the heap file
- leave some free space on each block for future insert
- index entries direct search for data entries

...

## hash

## b+tree
trade offs of btree and ISAM, to build ISAM is not hard. However, ISAM has disadvantage when inserting, it will eventually adding a lot of overflow pages and become linear scan, which lost the point of indexing.

`<key, page ptr>` pairs for each node, search routine is the same as binary search trees.

Always balanced and grows at root not leaves, splits from root level when insert and delete.

suppose `d` is the order of the tree (max fanout = 2d + 1), each interior node is at least partially full, where $$ d <= \#entries < = 2d $$

each inner node at least half full, except root node

btree can be serialised into files in the disk into sequences of bytes.

searching, same as searching in binary search tree, follow the pointer to leaf node, leaf node contains of which heap file and offset of the tuple with key.

insert. when inserting to a full leaf node, split leaf node and insert, for example

```
insert 5 into
[2, 3, 7, 8]

split
[2, 3, _, _] [7, 8, _, _]

insert 5 to new split node
[2, 3, _, _] [5, 7, 8, _]

```

copy up from leaf node using lowest value of new split node, so 5 will be used as inner node in this case.

if no room in parent, redistribute the rightmost `d` keys, push up the middle key (usually index 0 of rhs keys), and pushed up pointer (page number) always come to the rhs of the where it should be inserted.
```
        a b+tree with order 1

            [4]
           /   \
        [1,2]  [5]

        put 3

            [2, 4]
           /  |    \
        [1]  [2,3]  [5]
```

if splitting the root, need to alloc a new root to hold right hand side of the split and push up the middle key among the keys in old key to be the root key.

```
1. find correct leaf L (binary search)
2. put data entry onto L
    if L has enough space, insert
    else, split L (into L and new node L2)
        redistribute entries evenly, copy up middle key (smallest of L2)
        insert index entry pointing to L2 into parent of L

```

step 2 can be recursive, split grows tree evenly. To split index node, redistribute entries evenly, but push up middle key (smallest of L2) (contrast with leaf splits)

split grows tree, root split increase height, it gets wider or one level taller at top.

Bulk loading needs data to be sorted, it minimise split and io, it doesn't need to data to be put and check split then put over and over again. It sorts data initially, always load them to the left most leaf node and with given split factor, say 3/4 full then split. Usually, $$ 0 < split factor <= 1 $$.

Composite search key is arranged based on 1st key and if 1st key is dup, sort by 2nd key, etc. so if there is a schema R(a,b,c), data is sorted with following order
```
[a1,b1,c1]
[a2,b2,c2]
[a2,b3,c1]
...

```
it is the best to arrange query to be `=` in the first and do range `<, >, ...` in the last with lexicalgraphic order of a,b,c, for example `select * from R where a = 10 and b = 20 and c > 10;` to make query efficient by making query optimiser select the most ideal strategy. 


## Data Entry Storage
alt.1
- by value
- leaf node contains a copy of all tuples (key, value)
- record contents are stored in the index file
- no need to follow pointer

alt.2
- by reference
- leaf node contains (key, rid of matcing tuple), where rid is file number + offset in file

alt.3
- by list of references
- leaf node contains (key, [list of rids of matching data records])
- more compact than alternative 2, if there are duplicates of key
- for very large rid lists, single data entry spans multiple blocks

variable length keys
- can't use naive definition of occupancy invariant like 1/2 full
- may use alt.3 method to store the tree

redefine occupancy invariant
- measured by bytes and at least half full
- only reclaim space when a page is completely empty

some strategies of compress keys to allow more index key entries
- prefix
    + compress by prefix
    + on leaf split, determine min splitting prefix and copy up the prefix
- suffix
    + all keys have common prefix
    + make a header containing prefix and only suffix in inner node keys, on search, copy the prefix to match the whole key
    + useful in composite keys like <postcode, last name, first name> 



## multi-dimension index

kd-tree, quad-tree, R-tree
...
