# Query evaluation and query optimisation

## Query evaluation
mapping sql -> RA expr
- usually use templates to map similar forms, >= 1 for each kind of query
- mapping rules fir translating matched query into RA
- may need to apply > 1 templates to map whole sql statement

after mapping, apply rewriting rules to improve RA expr, which will be equivalent, simpler and more efficient.

postgres has provided user defined mapping rules using `CREATE RULE`

the join operation can be done in multiple ways, for a join with `n` tables, there are potentially `n!` possible ways to join, it is query optimiser's job to find most efficient way.

## Query rewrite
- commutative and associative laws
- selection splitting
- selection pushing
- selection pushing with join
- projections can be pushed into joins

## Algebra equivalences
- Selection -> cascade, commute
- projection -> cascade
- cross product -> associative, commute

joins are commutative and associative, but beware that it might become a cross product and larger if not dealt correctly

## Physical equivalences
implementation wise, is to choose what algorithm fits the best under certain circumstances

single table selection/projection
- heap scan
- index scan (if index is available on referenced columns)

equijoins
- BNLJ simple but needs memory
- INLJ good if 1 table is small and the other is indexed on join col
- SMJ when memory is small, table sizes are similar, if data is mostly sorted
- GHJ better than sort when 

non-equijoins
- BNLJ


## Heuristics
selection cascade and push down, apply selection as soon as the relavent columns occur

projection cascade and push down, apply projection to keep columns only needed for evaluating downstream operators

push selections down, consider only left deep join trees to minimise the plan combinations and able to do join in a pipeline, left hand side join is always a middle value of next join operator, it doesn't need materialise output.

however, pushing selection down on RHS does not help minimise IO, a full scan is still needed to satisfy selection predicate


all above are assuming no index


## Cost based query optimiser

some techniques of query optimisation
- algebraic (equivalences, rewriting, heuristics)
- physical (execution costs, search based)
- semantic (application properties, heuristic) ... good idea but hard to implement

in reality, algebraic + physical are the most common way to implement.

## Plan space
In system R like query optimisation, only left deep join trees are considered, left deep join trees 
are used to generate pipelined plans.
- no materialization, no files are written to temp files.
- not all deep left join tree are fully pipelined (SMJ).

in reality, query optimisation and execution is,
- materialization
- pipelining

pipelining can't avoid all disk access, some join require SMJ or HJ 
data have to be written to disk anyway and read by subsequent passes.

within operations, disks RW can occur

between operations, no disk RW is required


a full plan space contains
- all equivalent RelAlg expr
- all mixes of physical implementations of RelAlg, using different algorithms, indexes...

and to prune the space by
- selection/projection push down
- use left deep join trees
- avoid cartesian product

also take care some physical properties like sorting, the sequences of data will determine the 
downstream results, SMJ and BNLJ result depending on the previous operator sequence, and to enforce
the property later like being sorted, is really expensive when pipeline flows.




## Cost models and analysis
the cost of evaluating a query is determined by:
- size of relations (db relations and temp relations)
- access mechanisms (indexing, hashing, sorting, join algorithms)
- size/number of main mem buffers (also replacement strategy)

analysis of costs involves estimating:
- size of itermediate results
- num of secondary storage accesses

priority of choosing access methods
```
index scan: if reln has index on select/join column
    |
    v
hash scan: if reln hashed on select/join column or range scan on clustered index on select/join column
    |
    v
binary search: if reln is sorted on select/join column
    |
    v
linear search

```
### Selectivity
a lot of the stats are from database maintained catalog table, it has various of variables storing
attributes like number of tuples, min/max tuple in column, number of pages, index height,
number of pages of an index, etc.

The catalog update periodically in the background, it is too expensive to maintain in continously,
lots of approximation so those data might be a bit sloppy.

modern DBMS uses more advanced techniques like histogram.


max output cardinality = product of input cardinality

selectivity reflect the impact of the term in reducing result size,
selectivity = |output| / |input|, 

### Result size estimate
to choose query plan, especially for join algorithms, 
estimate size of output is helpful for choosing BNLJ, SMJ or GHJ


it is hard to get accurate size/cost estimate, it is either
- more time, complex computation of selectivity
- more space, storage for histograms of data values

there is a trade off between optimiser performance and query performance.

in System R, for simplicity, `result cardinality = Max number of tuples x product of all selectivities`

some examples,

for equi predicate like col = value given Nkeys(i) on col, `sel = 1/Nkeys(i)`

for equi predicate like col1 = col2 (joins), `sel = 1/MAX(Nkeys(i1), Nkeys(i2))`,
why MAX(x, y)? - it is a probability question.

for non-equi predicate like col > value, `sel = (High(i) - value)/(High(i) - Low(i) + 1)`

otherwise, for all other stats, assume it is 1/10.


Cost estimation for single relation plan,

all index are considered as btree index.

if index I on primary key matches selection: cost = (Height(I) + 1) + 1

if index I is clustered, cost = (Npages(I) + Npages(R)) * product of all selectivity

if index is unclustered, cost = (Npages(I) + NTuples(R)) * product of all selectivity

sequential scan cost = NPages(R)


in Postgres, in `include/utils/selfuncs.h`, a bunch of 
interesting selectivity estimations are defined.

```C
/* default selectivity estimate for equalities such as "A = b" */
#define DEFAULT_EQ_SEL	0.005

/* default selectivity estimate for inequalities such as "A < b" */
#define DEFAULT_INEQ_SEL  0.3333333333333333

/* default selectivity estimate for range inequalities "A > b AND A < c" */
#define DEFAULT_RANGE_INEQ_SEL	0.005

/* default selectivity estimate for multirange inequalities "A > b AND A < c" */
#define DEFAULT_MULTIRANGE_INEQ_SEL	0.005

/* default selectivity estimate for pattern-match operators such as LIKE */
#define DEFAULT_MATCH_SEL	0.005

/* default selectivity estimate for other matching operators */
#define DEFAULT_MATCHING_SEL	0.010

/* default number of distinct values in a table */
#define DEFAULT_NUM_DISTINCT  200

/* default selectivity estimate for boolean and null test nodes */
#define DEFAULT_UNK_SEL			0.005
#define DEFAULT_NOT_UNK_SEL		(1.0 - DEFAULT_UNK_SEL)
```
## Histograms
equiwidth - easier to maintain and arthimatics

equidepth - finer granularity

there are trade off between those 2 histograms

to compute selectivity from histograms

if the predicate is `AND` the selectivity = sel(Left) * sel(Right), since left and right probablity are independent.

if the predicate is `OR` on the same column, the selectivity = sel(Left) + sel(Right)

but across columns `OR` is that the total selectivity - the selectivity satisfy both
 sel(Left) + sel(Right) - (sel(Left) * sel(Right))

selectivity of a join is simply the size of 2 tables |R| * |S| * selectivity of the join predicate

column equality of the histogram,

an example,




## Query optimiser in Postgres
Postgres query optimisation is majority based on cost.

postgres takes sql input to generate a relalg operations `path tree` and query optimiser convert it to a `plan tree`, which is a evaluation order for query.

a generic `Path` node is like
```C
typedef struct path {
    NodeTag     type;           // scan/join/....
    NodeTag     pathtype;       // specific method
    RelOptInfo  *parent;        // output reln
    PathTarget  *pathtarget;    // list of vars/exprs, width
    Cost        startup_cost;   // setup cost
    Cost        total_cost;     // total cost
    List        *pathkeys;      // sort order
} Path;

```

where `pathkeys` is a collection like `(opfamily: Oid, strategy: Sort direction, nulls first: bool)`, where opfamily is for retrieve information from catalog about the sorting key, strategy is `ASC` or `DESC`, nulls first is to indicate if sorting should put null to first or last.


all `Node` types are defined in `include/nodes/*.h`

