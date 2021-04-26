use std::fs;
use std::time::SystemTime;
use std::io::Write;
use johnson::adj_list::{Edge, Graph, dijkstra, bellman_ford};

fn main() {
	//Reading the file and converting to a list of list of numbers
	let contents: Vec<_> = fs::read_to_string("input.txt")
		.expect("Error Reading file")
		.lines().map(read_nums).collect();

	//Creating the empty Adjacency List 
	let nvert = contents[0][0] as usize + 1;
	let mut edges: Vec<Vec<Edge>> = Vec::new();
	edges.push(Vec::new());
	for i in 1..nvert {
		edges.push(Vec::new());
		edges[0].push(Edge::new(i, 0));
	}

	//Creating the graph as a Graph object
	for i in contents.iter().skip(1){
		let n = i.len();
		for j in (1..n).step_by(2){
			edges[i[0] as usize].push(Edge::new(i[j] as usize, i[j+1]));
		}
	}
	let mut g = Graph {adj_list: edges, n: nvert};

	// Calculating APSP and timing
	let now = SystemTime::now();
	let apsp = johnson(&mut g);
	match now.elapsed() {
		Ok(expr) => {println!("{:?}", expr.as_millis());},
		Err(e) => {println!("Error {:?}", e);},
	}

	let out = apsp.iter()
				.map(|x| x.iter().skip(1)
							.map(write_num)
							.collect::<Vec<String>>()
							.join(","))
				.collect::<Vec<String>>()
				.join(&"\n");

	let mut file = fs::File::create("output.csv")
					.expect("Error opening File");
	file.write_all(out.as_bytes()).expect("Error Writing");
}

//Simple implementation of johnson's algorithm
fn johnson(g: &mut Graph) -> Vec<Vec<i32>> {
	//Checks if there is any negative cycle and calculates the d array
	//O(mn)
	let d = bellman_ford(g).expect("ERROR: Negative Cycle found");

	let mut apsp = Vec::new(); //Empty matrix

	//Modify Edge Costs so that all are non negative
	for u in 0..g.n {
		for e in g.adj_list[u].iter_mut() {
			e.cost += d[u] - d[e.end];
		}
	}

	//Run Dijkstra from eveny vertex and modify the 
	//final distance in terms of the original graph
	for u in 1..g.n {
		apsp.push(dijkstra(&g, u));
		for v in 1..g.n {
			if apsp[u-1][v] != i32::MAX {
				apsp[u-1][v] += d[v] - d[u];
			}
		}
	}

	apsp 
}

//Converts a string to array of integers
//For exmaple "a 23d 6s2 -20b" -> [23,6,2,-20]
fn read_nums(s: &str) -> Vec<i32> {
	s.split(|x| !(x == '-' || char::is_numeric(x)))
		.into_iter()
		.filter_map(|e| e.parse::<i32>().ok())
		.collect()
}

//Converts a number to a string with i32::MAX being converted to "inf"
fn write_num(s: &i32) -> String {
	if s == &i32::MAX {return "inf".to_string();}
	s.to_string()
}