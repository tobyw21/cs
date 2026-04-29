
# Storage

rel -> files -> pages -> tuple data at index

examples:
pageId = FileId + offset
where offset gives location of block in file

tupleId = pageId + offset
where offset gives location of tuple within page

offset maybe indexes into mapping tables gives real addr


single file dbms
sqlite
```
+------------+          
|params      |          
| -----------|          
|catalog     |          
|            |          
|------------|          
|update      |          
|logs        |          
|----------- |          
|table1      |          
|            |          
|------------|          
|table 2     |          
|            |          
|            |          
|----------- |          
|catalog (cont.)        
|            |          
|------------|          
|table 1 index          
|            |          
|----------- |          
|table 1     |          
|(cont.)     |          
|------------|          
|            |          
|            |          
|            |          
|            |          
+------------+                                                
```
unix seek doesnt alloc space for blank file, only allocs disk storage only when data is written in

## File Management

fd pool, to storage opened files and manage it to make it not to collide from unix fd limit

fd pool also provides indexes to the reln related to the file, to minimise the open() and close() which are expensive operations

PG stores data file in `PGDATA/pg_database.oid` dir 
often in multiple files like:

  Oid: table data pages
  Oid.1: table data pages (if table data pages is more than 1G, max file size is 1G)
  Oid_fsm: free space mapping (to see which page has free space in the table)
  Oid.vm: visibility map (whether the tuples are visible to currently executing transcations)

for fsm and vm pages will be ignored by vacuum.
free space is only "free" after vacuum, delete tuples is simply tag them as should be removed by vacuum, say when one tuple is tagged as to be deleted and another on going transcation still needs the tuple, it is still visible to the transcation, vacuum only execute after all transcation is done and tuple is no longer needed

vacuum is expensive, those files are also used for index pages to index all entries visible, allowing index scan to be more efficient. Postgres uses index only scan to check if tuples are visible, if no vm files, it would scan the whole page/file to figure out if the tuple is visible to active transcations, but if vm file exists, scan vm file first to find out which page is visible to current transcation and vacuum can skip it


reln files are identified via `pg_class.relfilenode` it can be chnaged if want to move data files around, most of the time it is same as `pg_class.oid`

some relns have no `pg_class.relfilenode`, use `pg_filenode.map`

## Buffer Management

buffer pool

pages -> frames[NBUFS]

frame data -> directory[NBUFS] (buffer descriptors, metadata)
  - which page it contains, if free?
  - dirty?
  - lock holder?
  - timestamp for most recent access

pages are referenced by page ID

pageID = buffer tag = (rnode, forkNum, blockNum)

page = byte[BUFSIZE]

in postgres, buffer manager defined in `src/include/storage/buf*.h` and `src/backend/storage/buffer/*.c`

all backend uses this shared pool of memory buffers

all access methods get data from disk via buffer manager

```
PostgreSQL Buffer Pool:

+-------------------+    +----------------------+    +--------------------------+
|   Page on Disk    | -> |   Frame (memory)     | -> | Descriptor / Directory   |
|  (e.g., 8KB page) |    |  BufferBlocks[i]     |    | BufferDesc[i]            |
+-------------------+    +----------------------+    +--------------------------+

        ^                          ^                           ^
        |                          |                           |
        |                          +-------- one-to-one -------+
        |
        +---> when page is read, loaded into frame, register into directory


```

directory is a list of frame info, where dir[i] contains metadata of frame[i], frame[i] has a pointer points to the page on disk, it might be empty..

page replacement policy

LRU

MRU

Random



clock sweep strategy:

treat buffer pool as a circular linked list of buffer slots

`NextVictimBuffer` holds index of next possible evictee

if page is popular, leave it 

    `usage_count` implements 'popularity/recency' measure

    incremented on each access to buffer up to a small amount

    decremented each time considered for eviction

increment `NextVictimBuffer` and try it again

basically clock sweep has a pointer points to the circular linked list starting from some point, from 0 usually, then set nextvictimbuffer to next, check the usage and change its count, when usage count is 0, evict this page, if not 0, usage count -1

some buffer pool functions worth to peek

```C
Buffer ReadBuffer(Relation reln, ForkNumber forkNum, BlockNumber blockNum)

Buffer ReadBufferExtended(Relation reln, ForkNumber forkNum, BlockNumber blockNum,
                                   ReadBufferMode mode, BufferAccessStrategy strategy)

```

 - ensures nth page of file for reln r is loaded (may need to remove an existing unpinned page and read data from file)
 - increments ref (pin) count for buffer
 - returns index of loaded page in bufpool (buffer value)
 - assumes main fork,  no fork num required

a special case of ReadBufferExtended which also handles variations like different replacement strategy, fork, temp buffer...


```C
void ReleaseBuffer(Buffer buffer)
 - dec pin count on buf
 - if pin == 0, ensure all activity on buffer is completed before returning

void MarkBufferDirty(Buffer buffer)
 - marks buffer as modified
 - requires that buffer is pinned and locked
 - actual write is done later (eg, when buf is replaced)

also
  BufferGetPage - finds actual data associated which buffer in pool
  BufferIsPinned - check if backend holds a pin on buf
  CheckPointBuffers - write data for recovery logs, flush all data from pool to the disk 
  and so on...
```

an important function is 
```C
static BufferDesc *
BufferAlloc(SMgrRelation smgr, char relpersistence, ForkNumber forkNum,
                        BlockNumber blockNum,
                        BufferAccessStrategy strategy,
                        bool *foundPtr)

Simplified to

BufferDesc *BufferAlloc(Reln r, ForkNum f, BlockNum n, book *found)
```
 - used by ReadBuffer to find a buffer for (r,f,n)
 - if (r,f,n) is in pool, pin it and return bufferdesc
 - if no available buf, select one to be replaced
 - returned bufdesc is pinned and marked as holding (r,f,n)
 - this doesnt read, ReadBuffer has to do the actual I/O


## Pages

ultimately, a page is simply `bytes[]`

usually a page size if 8k, but if overflows, store elsewhere like TOAST

how postgres formats its page,

for fixed length schema, it is simple,
```sql
create table Car (
  make char(10),
  year integer,
  seat integer
)
```
it is only 10 + 4 + 4 bytes long where in the page, db only need to have a fixed data type to hold the data,

scenario 1
```C
typedef struct page {
  int nrec;
  int maxrec;
  Record data[1]; 
  /* to take a offset,
   * you can do malloc(sizeof(Page) + MAXTUPLES * sizeof(Record))
   * in order to access page->data[index] 
   */
} Page;

```

when insert a page, check if theres free space, if not, make a overflow page or etc, if has free space, insert, nrec++, update record metadata to keep track of the offset, however, this structure has a problem when removing a record, imagine `void delete(Page *p, Index i)`, when trying to remove the page you want to compact the page without leaving a gap, so essentially needs to move the top tuple to fit the gap between, but that moves the position and offset to make it a problem for future access (no tracking), some db implements a map to keep track of position of tuple to get around this, something like `map[tupleId]offset` to keep track of where the tuple goes when being compacted.

scenario 2 is to implement a bitmap to keep track of which index is free
```C
typedef struct page {
  Bit    bitmap[MAXRECS];
  Record data[1]; 
} Page;

```

when inserting, find the immediate index to insert, when deleting, just mark the index as 0 for next write.

if variable length
```sql
create table Person (
  name text,
  age  integer,
  job  text
)
```

this needs to be handled differently, page needs to store a slot directory where keeps metadata of page offset of tuple and its length,
example:
```
Offset     Length      Data
--------   -------     -------------------------
8040       48 bytes    tuple header + varlen data A
7992       32 bytes    tuple header + varlen data B
```
and lists of free spaces, it might cause fragmentation, but when necessary, compact/merge the page. In this structure, the slot directory is written from bottom of the page to upper page and data is from upper to lower, in postgres, the page struct defined a lower and upper offset tracking, `pd_upper` and `pd_lower` (iirc), when `pd_upper >= pd_lower` means there is no space in the page, and should open a new page or fork. Of course there should be some tracker of next available offset


## Tuple Structure

a tuple is something like `(1, 'john', 28)` where the corresponding schema is `(id int, name varchar(10), age int)`, can be represented as C struct

the record is an array of bytes eg, `[0001 1101010 1101111 1101000 1101110 11100]`

some common operations, like get certain record
```C
Record get_record(Reln rel, RecId rid) {
    (pid,tid) = rid; // rid is consist of page id and table id
    Page *buf = request_page(rel, pid);
    return get_bytes(buf, tid);
}

```

once get the array of bytes, need to `Tuple t = make_tuple(rel, rec)`, relation is passed in that maybe relation contains how schema is defined, then need to extract the fields.
<code>
*Type* get*Type*Field(Tuple t, int fieldno)
</code>

for example `int i = getIntField(t, 1); char *s = getStrField(t, 2)`

and db internal would have a iterator to scan the relation, page and records

```C
Scan s = start_scan(Reln r, ...);

Tuple next_tup(Scan s);

```

start scan may have options for `WHERE` clause, going thru files

next tuple function simply itrating thru scanner and returns null if no more tuples left in relation

for fix sized records, it is easy, all it takes is just a tuple descriptor contains sizeof each index of the array of data and use that can calculate the offset and get the correct data store

variable length records
 - prefix each field by len
 - terminate fields by delimiter
 - have a header, length, array of offsets

dbms types relies on the fundamental C types

so a tuple can be defined as 

```C
typedef struct {
    ushort    nfields;  // num of fields
    ushort    data_off; // offset in struct for data
    FieldDesc fields[]; // field descriptors
    Record    data;     // pointer to record in buffer
} Tuple

```
