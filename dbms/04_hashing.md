# Hashing

## External hashing
partition phase: with hash function `h(p)`, with 1 input buffer and B - 1 output buffer, fetch 1 page at a time into input buffer. use hash function `h(p)`, write values with same hash result to the same partition and if any output buffer is full, flush to disk.

conquer phase, with another hash function `h(c)`, stream values for each parition, use hash function `h(c)`, build a in memory hash table which is no more than B pages, once full, flush hash table to disk bucket by bucket, so that `h(c)` makes same value is in contigious bucket in the disk.

cost: 2 * N * (number of pass) roughly 4N IOs including initial read and final write

memory usage: hash a table with N pages in about sqrt(N) memory (assume hash function is fair)

what if table is bigger? ...

## Recurisve partitioning
if the table size is bigger than B, then with the first partition, do another recursive partition on top of the large partition.

for frequent key like gender, no matter how many times of recursive partitioning, it is always in the same bucket, say value Female or Male, so it should stop partitioning once this situation is happened.

## Parallel hashing

## Linear Hashing

- if we have 2^k pages, bitwise AND with k low order bits set to 1
- else use mod, val % #pages

hashing has the best performance on selection (assume less overflow chains)

the problem of normal mod hashing is file size is dynamic, need to constantly change hash function size based on file size, rebuild file.

most applicable way is linear hashing, no directory, has overflow pages

flexible hashing would
- treat hash as 32bit string
- adjust hash by using more or less bits

linear hashing split
1. extract n bits from file size if file size = 2^n
2. if split pointer > n, extract last n+1 bits
3. insert tuple into page, if full, use overflow page
4. check if page needs split, partition tuples from page sp into pages sp and sp + 2^n
5. sp++, if sp == 2^n, n++, sp = 0

the point of linear hashing is to make less overflow pages or evenly distributed overflow pages.

in pg src, `backend/access/hash/hashfunc.c` provides detailed hash function interface.

for example, there is a query `select name from R where name = 'John';`, hashing on name = John may give a 32 bit value like `0b....0101`, if we have 2^3 pages, we care about last 3 digits and can find pages containing name = John is `0b101`

However, this is only useful if someone is asking a query based on name, but what if we have query like `select branch, amount from Deposite where branch = 'LA' and amount = 100.0;`??


## MAH - Multi-Attribute Hashing
multi attribute hashing used to resolve problem of selection where multiple attributes are involved.

the formation of mah is using choice vector, which specifies the bits of final hash by using which attribute in reln.

an example of using choice vector:
```
cv contains pairs of (attribute,bit)
cv = [(1,0), (1,1), (2,0), (2,1), (2,2), (3,0)]

R(a,b,c) has hash:
hash(a) = 0b10110010
hash(b) = 0b10110010
hash(c) = 0b10110010

final hash:
bit[0] = 0
bit[1] = 1
bit[2] = 0
...

final hash = 0b010100
```
so when trying to put data or access data, a,b,c = something will be put into page number 0b100 if page size is 2^3.

However, sometimes we ask query like this: `select a,b,c from R where a=1 and c=1` left b = ?, then there is unknow bit

using example above, we can get `final hash = 0b01???0` because we don't have hash value for b, but we can determine that the page containing a=1 and c=1 should be in `000, 010, 100, 110...`, this can minimise the total scanning and file io.
