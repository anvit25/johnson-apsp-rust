use std::cmp::min;

#[derive(Copy, Clone, Debug)]
struct State {
	vertex: usize,
	d: i32
}

#[derive(Debug)]
pub struct Edge {
	pub end: usize,
	pub cost: i32
}

#[derive(Debug)]
pub struct Graph {
	pub adj_list: Vec<Vec<Edge>>,
	pub n: usize
}

#[derive(Debug)]
struct Heap {
	vec: Vec<State>,
	len: usize,
	pos: Vec<usize>
}

impl Edge {
	pub fn new(end: usize, cost: i32) -> Self{
		Self {end: end, cost: cost}
	}
}


impl Heap {
	// Create a new heap from an array. O(n)
	pub fn new(vec: Vec<i32>) -> Self {
		let n = vec.len();
		let mut heap = Self {
			vec: (0..n).zip(vec)
					.map(|(x,y)| State {vertex: x, d: y})
					.collect(),
			len: n,
			pos: (0..n).collect()
		};
		heap.heapify();
		heap
	}

	//Look at the cost of some particular vertex. O(1)
	pub fn look(&self, i: usize) -> i32 { self.vec[self.pos[i]].d }

	//Extract the minimum vertex and its cost from the heap
	//O(log n)
	pub fn get_min(&mut self) -> Option<State>{
		if self.len == 0 {return None;}
		self.len -= 1;
		self.swap(0, self.len);
		self.pushdown(0);
		self.vec.pop()
	}

	//Decrease the key of a particular vertex
	// O(log n)
	// Note that this DOES NOT check the key is actually being decreased
	// or increased. 
	pub fn dec_key(&mut self, state: State) {
		let index = self.pos[state.vertex];
		assert!(self.vec[index].vertex == state.vertex);
		self.vec[index] = state;
		self.bubble(index);
	}

	//Simple comparision function O(1)
	fn cmp(&self, p1: usize, p2: usize) -> bool {
		self.vec[p1].d > self.vec[p2].d
	}

	// Swap two node while also maintining the hash map (a simple array here)
	// O(1)
	fn swap(&mut self, p1: usize, p2: usize){
		self.vec.swap(p1, p2);
		self.pos.swap(self.vec[p1].vertex, self.vec[p2].vertex);
	}

	//Pushdown operation on index i.
	//If heap property is maininted on the two child trees
	//Then pushdown will maintain heap property on whole 
	//tree (with root i). O(log n) in worst case. 
	fn pushdown(&mut self, i: usize){
		let l = 2*i+1;
		let smaller = l + (l+1 < self.len && self.cmp(l, l+1)) as usize;
		if smaller < self.len && self.cmp(i, smaller) {
				self.swap(i, smaller);
				self.pushdown(smaller);
		}
	}

	//For converting an array to a heap
	//O(n)
	fn heapify(&mut self){
		for i in (0..self.len).rev() {self.pushdown(i);}
	}

	//Bubble Up operation. Used in conjunction is decrease
	// key. O(log n)
	fn bubble(&mut self, i: usize){
		let mut i = i;
		while i > 0 && self.cmp((i-1)>>1, i) {
			self.swap((i-1)>>1, i);
			i = (i-1)>>1;
		}
	}

	// For Checking if heap property is mainting.
	// For debugging only. O(n)
	fn check_heap(&self){
		for i in 0..self.len {
			let l = 2*i + 1;
			let r = 2*i + 2;
			if r < self.len {
				assert!(!self.cmp(i, r));
				assert!(!self.cmp(i, l));
			} else if l < self.len {
				assert!(!self.cmp(i, l));
			}
		}
	}
}


// Binary heap implementation of Dijkstra 
// O(m log n)
pub fn dijkstra(g: &Graph, origin: usize) -> Vec<i32> {
	let mut dis: Vec<i32> = vec![0; g.n]; //Distance array, output
	let mut visited: Vec<bool> = vec![false; g.n]; //Keeps track of visited vertex
	let heap = vec![i32::MAX; g.n]; 
	let mut heap = Heap::new(heap); //Create a heap with all vertices having inf distance
	heap.dec_key(State{vertex: origin, d: 0});//Change the distance of starting node to 0
	//All the above operations have a combined complexity of O(n)

	//While heap is non empty, extract the min distance vertex
	//The loop runs a maximum of n times
	while let Some(State {vertex, d}) = heap.get_min() {
		dis[vertex] = d; //change the distance array
		visited[vertex] = true; //mark the vertex as visited
		if d != i32::MAX{
			// If the distance is finite, relax the adjacent edges
			for Edge {end, cost} in &g.adj_list[vertex] {
			//Combining this and the outer while loops, they run m times
				if !visited[*end] && heap.look(*end) > cost + d {
					heap.dec_key(State {vertex: *end, d: cost + d});
				}
				//dec_key has complexity O(log n)
			}
		}
	}
	dis
}

//Simple implementation of bellman ford. Relaxes every edge n-1 times
//O(mn)
// return the distance from 0 if there is no negative cycle else returns
// None 
pub fn bellman_ford(g: &Graph) -> Option<Vec<i32>> {
	let mut d: Vec<i32> = vec![i32::MAX; g.n];
	d[0] = 0;
	for _ in 1..g.n {
		for u in 0..g.n {
			for Edge {end, cost} in &g.adj_list[u]{
				d[*end] = min(d[*end], cost + d[u]);
			}
		}
	}

	for u in 0..g.n {
		for Edge {end, cost} in &g.adj_list[u]{
			if d[*end] > cost + d[u] {return None;}
		}
	}
	Some(d)
}

