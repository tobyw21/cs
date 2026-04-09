# Concurrency and transactions

## ACID
- Atomicity: All actions in trx happen or none happen.
- Consistency: If the DB starts out consistent, it ends up consistent at the end of trx.
- Isolation: Execution of each trx is isolated from of others.
- Durability: If a trx commits, the result persists.

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
