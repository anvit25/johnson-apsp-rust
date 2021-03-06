# Johnson's APSP

>The executable can be [downloaded here](https://github.com/anvit25/johnson-apsp-rust/releases/)

This is a simple implementation of [Johnson's All Pair Shortest Path](https://en.wikipedia.org/wiki/Johnson%27s_algorithm "Johnson's algorithm") algorithm. Dijkstra's algorithm is implemented using a Binary Heap.
The time complexity of this implementation is O(VE logV)

## How to Use
The program can be used by typing the following in terminal/CMD/Powershell
```
johnson.exe <path/to/input/file> <path/to/output/file>
```
The paths can be relative or absolute. In case the parameters are omitted, the application defaults to reading input from "input.txt" and writing output to "output.csv".

WARNING: If the output file already exists, it will be deleted/overwritten. 

## Input/Output Format

The format of the input is as following :-
```
V=3, E=3
1: (2,3), (3,-1)
3: (1,4)
```
This represents a graph with three edges and three vertices. The program assumes that the edges are named 1,2,...,n . Here, we have three edges, from 1 to 2 with weight 3, from 1 to 3 with weight -1 and from 3 to 1 with weight 4. Note that there is no outgoing edge from 2.
Note that the bracket, commas and colons are not needed. The following is also a valid input file for the same graph
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
The output is a V x V matrix with all the required lengths with ith row corresponding to starting from vertex i and jth column corresponding to ending at column j. 

## Benchmark Results
The time is given in ms. The random test cases were produced using Python. For generating a edge, first pick a sample of size 2 from 1..n and then pick a weight (benchmarking done with weight ~ Unif(-5,94) with seed(1836)). The benchmarking was done on Core-i3 8300H 2.3Ghz. Three iterations were done for each vertex edge combination. 

|#Vertex|#Edges|#Negative Edge|Func Only|With I/O|
|:---:|:---:|:---:|:---:|:---:|
|50|50|3,3,0|0,0,0|14,14,13|
|50|100|3,5,4|0,0,0|13,14,14|
|50|200|10,14,18|0,0,0|13,13,13|
|100|100|4,3,2|0,0,0|15,16,15|
|100|250|11,17,14|1,1,1|17,18,17|
|100|500|19,19,21|2,2,2|19,17,18|
|250|500|23,23,28|8,7,8|36,37,36|
|250|750|49,29,38|12,12,12|41,41,42|
|250|1000|52,42,48|13,14,13|43,43,44|
|500|500|23,23,25|8,8,8|79,73,72|
|500|1000|58,44,48|34,33,35|115,107,116|
|500|1500|76,72,91|51,64,57|148,136,130|
|1000|1000|50,49,62|30,30,30|232,213,262|
|1000|2000|99,99,103|166,178,159|403,437,390|
|1000|3000|155,172,167|231,226,318|461,459,488|
|2500|2500|117,126,123|238,232,242|1309,1335,1639|
|2500|5000|258,243,232|1527,1693,1208|3046,2927,2593|
