# Johnson's APSP

>The executable can be [downloaded here](https://github.com/anvit25/johnson-apsp-rust/releases/)

This is a simple implementation of [Johnson's All Pair Shortest Path](https://en.wikipedia.org/wiki/Johnson%27s_algorithm "Johnson's algorithm") algorithm. Dijkstra's algorithm is implemented using a Binary Heap.
The time complexity of this implementation is O(VE logV)

The application reads input from "input.txt" and writes output to "output.csv".

WARNING: If "output.csv" already exists in the current directory, it will be deleted. 

The format of the input is as following :-
```
V=3, E=3
1: (2,3), (3,-1)
3: (1,4)
```
This represents a graph with three edges and three vertices. The program assumes that the edges are named 1,2,...,n . Here, we have three edges, from 1 to 2 with weight 3, from 1 to 3 with weight -1 and from 3 to 1 with weight 4. Note that there is no outgoing edge from 2.
Note that the bracket, commas and colons are not needed. The following is also a valid input file
```
3 3
1 2 3 3 -1
3 1 4
```

The output for the above input will be
```
0,3,-1
inf,0,inf
4,7,0
```
The output is a V x V matrix with all the required lengths.

## Benchmark Results
The benchmarking was done on Core-i3 8300H 2.3Ghz
|Vertex|Edge|Time (ms)| Time (ms)|
| | |Just function| Inc I/O|
|:---|:---:|:---:|---:|
|50|50|0|14|
|50|50|0|13|
|50|100|0|13|
|50|100|0|13|
|50|200|0|13|
|50|200|0|13|
|50|500|0|14|
|50|500|0|13|
|100|100|0|14|
|100|100|0|15|
|100|250|1|17|
|100|250|1|17|
|100|500|2|17|
|100|500|2|17|
|100|750|2|17|
|100|750|2|18|
|250|500|9|36|
|250|500|8|37|
|250|750|12|40|
|250|750|12|40|
|250|1000|14|41|
|250|1000|14|43|
|500|500|9|66|
|500|500|8|65|
|500|1000|40|105|
|500|1000|36|105|
|500|1500|51|130|
|500|1500|51|129|
|1000|1000|32|235|
|1000|1000|31|228|
|1000|2000|169|426|
|1000|2000|155|369|
|1000|3000|221|432|
|1000|3000|215|484|
|2500|2500|207|1124|
|2500|2500|196|1166|
|2500|5000|1168|2304|
|2500|5000|1147|2301|
