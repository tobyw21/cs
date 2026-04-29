# Sorting

file sort has to be 2 way merge sort, it sorts every pages tuple first, then sort with group of 2
sets of tuples, then sort with group of 4 sets of tuples and etc to finalise the sorting.

requires $ ceil(log_{2}(b)) $ passes to sort, each pass requires b page read and write in total =
$ 2b \times ceil(log_{2}(b)) $

an improved way to do sort is n-way merge sort which has B blocks of buffer and n read buffers and B - n write buffers, sort n buffers, use B - n write buffers to write to disks, this can reduce number of RW, read in n pages and sort them, write them out, and do it over and over again.

in total, number of passes are $ 1 + ceil(log_{B-1}ceil(n/B)) $, cost = 2n * number of passes, the total memory usage is around sqrt(n), where n is size of data.

## double buffering
double buffering is a technique to make n-way sorting more flexible using multithreading. usually 2 way sorting, use 2 buffers, a thread in behind running sorting and loading to output buffer, another thread responsible for getting the input to read buffer waiting the sorting thread to switch to current buffer, if read buffer is drained, sorting thread will swtich to the other thread and keep running until file is sorted.

```
            switch t1 to read buffer 2 when read buffer 1 is full, t2 start draining read buffer 1 and filling output buffer 1, when output buffer is full, flush to disk. when read buffer is empty, switch this buffer to t1 to fill up from file. t2 is sorting read buffer 2. 

    thread1                    thread2
       |                          |
       v                          v
        [read buffer1]                      [output buffer1]
[file]                  (sorting algorithm)
        [read buffer2]                      [output buffer2]

```

## Parallel sorting