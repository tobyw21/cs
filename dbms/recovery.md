# Database Recovery

In ACID properties, recovery manager covers Atomicity and Durability, also rollback transaction that violates Consistency.

A naive way to achieve atomicity and durability is 

No steal + Force:
- steal: buffer pool "steals" the page before pages are committed.
- force: buffer pool "forces" each change into disk before commit.

No steal: don't allow buffer pool frames with uncommitted pages to be replaced or otherwise flushed to the disk.

Useful to achieve atomicity without UNDO logging, but poor performance (pinned pages limit buffer replacement).

Force: make sure every update is "forced" to the disk before commit. Provides durability without REDO logging, but lots of random IO.

Preferred policy is Steal + No force.


## WAL
1. Must **FORCE** the log record for an update **BEFORE** the corresponding data page flushed to the disk.
2. Must **FORCE ALL LOG RECORDS** for a xact before commit.
    + xact is not count as committed until all of its log records including its "commit" record are on the stable log.

The 1st point guarantees UNDO (atomicity), the 2nd point guarantees REDO (durability).

The allows steal/no force.

usually a log record contains `<XID, pageID, offset, length, old data, new data>`

Log is an ordered file with a write buffer called "tail" in RAM. 

Each log record has a Log Sequence Number (LSN), which is unique and increasing.

Variable `flushedLSN` tracked in RAM, this is the latest flushed LSN when log record is already in the disk.

Each data page in the disk contains a `pageLSN` is a pointer points to the log, it is the most recent log record for an update to that page.

Before page is flushed to the DB, log must satisfy: `pageLSN <= flushedLSN`, when a log record is flushed to the disk, `flushedLSN++`.

Buffer frame is not necessary to be stolen if page is hot, as long as the log record is written to the disk, page can be flushed later.

## ARIES
read this [paper](https://cs.stanford.edu/people/chrismre/cs345/rl/aries.pdf)

ARIES log records contains prevLSN is the previous log record by this XID, so that the log can form a linked list backwards in time.

log record fields usually come with:
- LSN
- prevLSN
- XID
- type

Update records only fields:
- pageID
- length
- offset
- before image
- after image

Possible log record types:
- update
- commit
- abort
- checkpoint (for log maintenance)
- compensation log records (for UNDO)
- end (end of commit/abort)

to do REDO/UNDO, log record has sufficient information to do a physical diff on the bytes in a page.

Two in-memory tables:
1. transaction table
    - one entry per currently active xact, removed when xact commits or aborts.
    - contains
        + hash table hashed by XID
        + status (running, commit, abort)
        + last LSN (the most recent LSN written by xact)
2. dirty page table
    - one entry per dirty page currently in buffer pool, removed when dirty page is flushed to disk.
    - contains
        + hash table hashed by pageID
        + recovery LSN (the LSN of the log record which first caused the page to be dirty)

## Checkpoint
naive checkpoint:
- Stop accepting new transactions
- wait until all transactions complete (commit/abort)
- flush log to disk
- flush all dirty pages to disk
- write a checkpoint log record, flush log again
- at this point, changes by committed txns are written to disk, and aborted txns are rolled back.
- resume txns

fuzzy checkpoint:
- save state of all txns and page statues
    + txn table
    + dirty page table

-------

ARIES logging during forward processing
- xact starts
- write START record in log
- update xact table, set last LSN to null and status to running

when xact makes update
- write update record in log
- update following
    + in WAL, prevLSN = lastLSN
    + in the dirty page, pageLSN = LSN
    + in xact table, lastLSN = LSN
    + in DPT, recLSN = if null then LSN

when page flushes
- flush log up to (including) pageLSN
- remove page from DPT and buffer pool (if needed)

when need to fetch new page
- create entry in DPT set recLSN = null
- bring page into buffer pool (if not in there)
- `goto` when xact makes update section.

when xact commits
- write a commit record into log
- flush log up to this entry
    + xact is now considered committed
- update xact table, set status = commit
- flush dirty page to disk (later)
- write an end record into log when all dirty pages are written to the disk
- update xact table, set status = complete

when xact aborts
- write abort record into log
- find the first action to undo from lastLSN
- start undo changes
- write compensation log record to log
    + CLR's `undoNextLSN` points to next record to undo (fault tolerant crash during undo)
- follow prevLSN to retrace more (if any) actions to undo
- once done, change xact table status = abort
- flush dirty page to disk (later)
- write an end record into log when all dirty pages are written to the disk
- update xact table, set status = complete