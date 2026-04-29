# Postgres Architecture

client connect to postgres daemon, daemon fork backend process for sql queries

```
+-----------------+             +----------------+    
|    PG  daemon   |             | pg_backend     |    
|       pid=1234  |  fork()     | for operation  |    
|                 |-------------| like select    |    
+-----------------+             |     pid=1235   |    
        |                       |                |    
        |                       +----------------+    
        |                               |             
        |                               |             
        |                               |             
        |                               |             
        |                               |             
        |                       +--------------------+
        |                       |internal exec()     |
        |                       |like implementation |
        |-----------------------| to execute query   |
        |                       | pid=1235           |
        |                       +--------------------+
        |                   at some point user input via psql                                      
        |                     make session ends
        |                     but postmaster keeps lisening       
        v                                             
    continue                                          

```

PG_DATA
 - base
   - db1
     - rel1
 - global
   - global relns
 - pg_xlog
   - logs
 - pg_hba.conf (limit user access)

postgres uses OID as name or dir for each db/files for each table

`pg_wal` dir contains some tranx data, which is used to write to log before to db/file/disk, incase the interrupt of transcation

`pg_xact` is the transcation metadata logs, potentially to be used by `VACUUM` 

`pg_catalog` contains tables to find dir of db/data files for db
`pg_database` .. 
`pg_class` ..

remove postmaster.pid if server is start but cant connect


relational ops and access methods see data as
 - seq of fixed size pages 1K-8K
 - where each page contains tuple data or index data

file manager sess db obj and file store as
 - maps (tableName, pageIndex) -> (file, offset)


SQL standard `information_schema.*`

lower level of pg's own representation via
 - global schema `pg_catalog`
   - set of tables/views 

The files with _fsm suffixes contain free space maps which indicate where space is available in the data file (see Section 68.3 of the PostgreSQL documentation).

The files with _vm suffixes contain visibility maps which indicate pages that contain tuples visible to all active transactions; this allows vacuuming to be optimised (see Section 68.4 of the PostgreSQL documentation).

The files with OIDs close to those of the table data files, but without any free space maps are index files (each table has an index on its defined primary key).


related [reading](http://db.cs.berkeley.edu/papers/fntdb07-architecture.pdf) sections: 1, 2, and 4 (up to subsection 4.5 included).