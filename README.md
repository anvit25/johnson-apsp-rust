# johnson-apsp-rust

This is a simple implementation of Johnson's All Pair Shortest path algorithm. Dijkstra's algorithm is implemented using a Binary Heap.
The time complexity of this implementation is O(VE logV)

The application reads input from "input.txt" and writes output to "output.csv".

WARNING: If "output.csv" already exists in the current directory, it will be deleted. 

The format of the input is as following :-
'''
V=3, E=3
1: (2,3), (3,-1)
3: (1,4)
'''
This represents a graph with three edges and three vertices. The program assumed edges are names 1,2,...,n . Here, we have three edges, from 1 to 2 with weight 3, from 1 to 3 with weight -1 and from 3 to 1 with weight 4. Note that there is no outgoing edge from 2.
Note that the bracket, commas and colons are not needed. The following will also work
'''
3 3
1 2 3 3 -1
3 1 4
'''

The output for the above input will be
'''
0,3,-1
inf,0,inf
4,7,inf
'''
The output is a V x V matrix with all the required lengths



