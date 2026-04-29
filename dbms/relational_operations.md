# Relation Operations

<!-- advantage of relational structure compare to IMS and CODASYL -->
<!-- - many to many relations without redundancy
- declarative
- physical data independence
- logical data independence -->

relational algebra defines the operation of dbms data manipulation, the core of relational dbms.

eg

| type      | SQL      | RelAlg |
| ------------- | ------------- | ------------- |
| scan | select * from R | R |
| proj | select x,y from R | Proj[x,y]R |
| sort | select * from R order by y | sort[y]R |
| cross prod | combine 2 tables | R1 x R2 |
| join | select * from R1 join R2 on pred | join(R1 x R2, pred) |


all relational operations return a set of tuples. a simple idea of a typical operation program:
```C
ResultSet = {}
while (t - nextRelevantTuple()) {
	newt = formatResultTuple(t, Projection)
	ResultSet += newt
}
return ResultSet

```

`nextRelevantTuple()` responsiblity:
- selection
	+ find next possible result tup in table
	+ check if satisfy cond
- join
	+ find next possible pair of tuples from R1 R2
	+ check if pair satisfy join cond



<!-- ## Recursive Queries
`WITH RECURSIVE` clause can join with itself

example of R(n), iteratively join with itself
```SQL
with recursive R(n) as (
	(select 1 as n
	union
	select n + 1 from R
	where n < 100)
	select sum(n) from t;
)
``` -->


## unary
on single reln
- proj
- sel
- renaming

## binary
on pair of reln
- union
	+ `UNION` vs `UNION ALL`: `UNION ALL` keep duplicates but union is distinct
- set-diff
	+ in sql exists as `EXCEPT` or `EXCEPT ALL`, `EXCEPT` returns only distinct results but `EXCEPT ALL` returns all results. S1 - S2 $ S_n $ is reln, will get a moon shaped S1, without the common elements in the venn diagram
- cross prod

## compound
- intersection
	+ in sql exists as `INTERSECT` where it is S1 - (S1 - S2), $ S_n $ is reln
- joins
