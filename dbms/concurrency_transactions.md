# Concurrency and transactions

## ACID
- Atomicity: All actions in trx happen or none happen.
- Consistency: If the DB starts out consistent, it ends up consistent at the end of trx.
- Isolation: Execution of each trx is isolated from of others.
- Durability: If a trx commits, the result persists.

Preventing
- dirty read
- non-repeatable read
- phantom read

## Serializability
Serial equivalences
- serial schedule, each trx runs in a sequence without having intervening actions from other trx
- 2 schedules are equivalent if they:
    + involve same trx
    + each trx's actions are ordered the same
    + both schedules leave the DB in the same final state

Schedule S is serializable if S is equivalent to some serial schedule.

## Conflict serializability

2 operations conflict if they
- are by different trx
- are on the same object
- at least one is write

2 schedules are conflict equivalent if they involve the same actions of the same trx, and every pair of conflicting actions is ordered the same way.

Schedule S is conflict serializable if S is conflict equivalent to some serial schedule, which implies S is also serializable.

schedule is conflict serializable if only if dependency graph is acyclic. 

## View serializability
with fewer false negatives

Schedule s1 and s2 are view equivalent if
- same initial reads:
    if Ti reads value of A in S1, then Ti also needs to read value of A in S2
- same dependent reads:
    if Ti reads value of A written by Tj in S1, then Ti also reads value of A written by Tj in S2
- same winning reads:
    if Ti writes final value of A in S1, then Ti also needs to write final value of A in S2

view serializability allows a few more schedules than conflict serializability but difficult to enforce efficiency.


## 2PL

Rules:
- read trx must get S(shared) lock before reading and X(exclusive) lock before writing.
- trx cannot get new lock after releasing any lock.

strict 2PL: same as 2PL, except all locks released together when trx completes.

either
- trx has committed (all writes durable) OR
- trx has aborted (all writes have been undone)

## Lock manager

Maintains a hash table, keyed on names of objects being locked.

Keeps an entry for each currently held lock.
- Granted set: set of xacts currently granted access to the lock.
- Lock mode: type of lock held (S or X).
- Wait queue: queue of lock requests.

when a lock request arrives:
- if granted set has conflicting locks, put request into queue.
- else put the requester into granted set and let them proceed.

xact with shared lock can request to upgrade to exclusive.

Compatibility matrix:
```
    (Boolean value in cell answers is `left` compatible with `top`?)
        | NL  | IS  | IX  |  S  | SIX |  X
    ----+-----+-----+-----+-----+-----+-----
    NL  |  T  |  T  |  T  |  T  |  T  |  T
    ----+-----+-----+-----+-----+-----+-----
    IS  |  T  |  T  |  T  |  T  |  T  |  F 
    ----+-----+-----+-----+-----+-----+-----
    IX  |  T  |  T  |  T  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    S   |  T  |  T  |  F  |  T  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    SIX |  T  |  T  |  F  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    X   |  T  |  F  |  F  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
```


Parent compatibility matrix:
```
    (Boolean value in cell answers can `left` be the parent of `top`?)
        | NL  | IS  | IX  |  S  | SIX |  X
    ----+-----+-----+-----+-----+-----+-----
    NL  |  T  |  F  |  F  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    IS  |  T  |  T  |  F  |  T  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    IX  |  T  |  T  |  T  |  T  |  T  |  T
    ----+-----+-----+-----+-----+-----+-----
    S   |  T  |  T  |  F  |  T  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    SIX |  T  |  F  |  T  |  F  |  F  |  T
    ----+-----+-----+-----+-----+-----+-----
    X   |  T  |  T  |  T  |  T  |  T  |  T
    ----+-----+-----+-----+-----+-----+-----
```

Substitutability Matrix
```
    (Values along left are `substitute`, values along top are `required`)
    
        | NL  | IS  | IX  |  S  | SIX |  X
    ----+-----+-----+-----+-----+-----+-----
    NL  |  T  |  F  |  F  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    IS  |  T  |  T  |  F  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    IX  |  T  |  T  |  T  |  F  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    S   |  T  |  T  |  F  |  T  |  F  |  F
    ----+-----+-----+-----+-----+-----+-----
    SIX |  T  |  T  |  T  |  T  |  T  |  F
    ----+-----+-----+-----+-----+-----+-----
    X   |  T  |  T  |  T  |  T  |  T  |  T
    ----+-----+-----+-----+-----+-----+-----
    
    The boolean value in the cell answers the question:
    "Can `left` substitute `top`?"
    
    or alternatively:
    "Are the privileges of `left` a superset of those of `top`?"
    
```