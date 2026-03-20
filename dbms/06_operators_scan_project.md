# Scanning

with file structures:
- heap file ... tuples added to any page which has space
- sorted file ... tuples arranged in key order
- hash file ... tuples placed in pages using hash function


scanning data structures are defined in `/backend/access/heap/heapam.c` which implements iterator data/operations.

- `HeapScanDesc` iteration state struct
```C
scan = heap_beginscan(rel, ..., nkeys, keys);
// uses initscan() to initialise iterator and rescan
tup = heap_getnext(scan, direction);
// uses heapgettup() to get related tuples
heap_endscan(scan); // free the scan ds

```

postgres keeps a iterator data structure to scan the tuples within a file which includes a data file handle, a buffer, tracking of page number, record number and any condition apply to the query. Iterator would init a scan from a file/relation. 

some scanning functions are defined in `/backend/access/heap/heapam.c`, a simple scan with its overflow file would be like
`select * from T where id = 123`
```C
cond = {id : 123};

for f in files {
	for tup in f {
		if (scan(tup, cond)) {
			writeToOutput(tup);
		}
	}
	
for ovf in overflowfiles {
	for tup in ovf {
		if (scan(tup, cond)) {
			writeToOutput(tup);
		}
	}
}

```

copying is involved as a less common operation, for example,
```sql
create table T as (select * from R)
```

example iterator for scanning 
```C
typedef struct iterator {
	File inf; 		// input file handle
	Buffer buf; 	// input buffer
	int curpage; 	// current scanning page
	int curidx; 	// current record number
	Expr cond; 		// representation of condition
} Iterator
```


usually, scanning are using heap files and above data structure is for heap file.

heap file is simple, unordered, no index, no hashing.

There are other access file structures
- btree
- hash
- gist
- gin

they all implement
- startscan, getnext, endscan
- insert, delete
- other file specific operators



# Projection
sort based projection
 - scan input relation R and produce a file of tuples containing only the projected attrs
 - sort the file of tups using the combination of all attrs as the sort key
 - scan the sorted result, comparing adjacent tups and discard dups
 - requires the same amount of tmp files/reln in order to copy in and out for sorting

hash based projection for finding the distinct values, first partitions the page use hash function h1, then hash the partitions with
hash function h2 to find the distinct value

- scan input relation and produce a set of hash partitions based on the projected attrs
- scan each hash partition looking for dups
- once each partition is dup free, write out the remaining tups

