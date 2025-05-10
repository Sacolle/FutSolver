use std::collections::BinaryHeap;

struct State<const K: usize> {
	num_i: usize,
	group_i: usize,
	bk_meta: [(usize, f64);K]
}

#[derive(Debug)]
struct Solution<const N: usize> {
	solution: [f64;N],
	variancia: f64
}

impl<const N: usize> Solution<N> {
	fn new(solution: &[f64;N], variancia: f64) -> Self{
		Solution { 
			solution: solution.clone(), 
			variancia
		}
	}
}

impl<const N:usize> Ord for Solution<N>{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.variancia.total_cmp(&other.variancia)
	}
}

impl<const N:usize> PartialOrd for Solution<N>{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other))
	}
}

impl<const N:usize> PartialEq for Solution<N>{
	fn eq(&self, other: &Self) -> bool {
		self.variancia == other.variancia
	}
}
impl<const N:usize> Eq for Solution<N>{}

fn variancia<const K: usize>(meta: &[(usize, f64);K], average: f64) -> f64 {
	meta.iter()
		.fold(0.0f64, 
			|acc, (_, sum)| acc + (sum - average).powf(2.0)
	) / (K as f64)
}
//https://stackoverflow.com/questions/58079910/find-all-ways-to-partition-a-set-into-given-sized-subsets

//N is input size
//K é o número de grupos
// N/K é o tamanho de um grupo
fn groups_iterative<const N: usize, const K: usize>(input: [f64;N], upper_bound: f64, average: f64) -> BinaryHeap<Solution<N>>{

	let mut groups: [f64;N] = [0.0f64; N];
	let mut groups_meta: [(usize, f64);K] = [(0usize, 0.0f64); K];
	let mut bookkeep: Vec<State<K>> = vec![];

	const MAX_NUMBER_OF_SOLUTIONS: usize = 100;
	let mut num_of_solutions: usize = 0;
	let mut heap: BinaryHeap<Solution<N>> = BinaryHeap::with_capacity(MAX_NUMBER_OF_SOLUTIONS);

	let mut i = 0;
	while i < N || bookkeep.len() > 0{
		if i < N {
			'acess: for j in 0..K{
				//primeiro elemento da tupla representa o tamanho do grupo de mesmo índice
				if groups_meta[j].0 == 0 {
					//se grupo é vazio
					bookkeep.push(State { num_i: i, group_i: j, bk_meta: groups_meta.clone() });
					//nessa regra sai da iteração
					break 'acess;
				}else if groups_meta[j].0 < (N/K) { //N/K is the size of the group
					//se a lista está incompleta 
					//NOTE: checa aqui se estoraria o upperBound, 
					//correto seria upper_bound - min(input), mas tem q testar primeiro	
					if groups_meta[j].1 <= upper_bound {
						bookkeep.push(State { num_i: i, group_i: j, bk_meta: groups_meta.clone() });
					}
				}
			}
		}else{
			//aqui a list está completa, mas o bookeep não necessariamente está vazio
			//ou seja, groups é uma solução melhor que o upperbound
			//se não coletou MAX_NUMBER_OF_SOLUTIONS, coleta
			if num_of_solutions < MAX_NUMBER_OF_SOLUTIONS {
				heap.push(Solution::new(&groups, variancia(&groups_meta, average)));
				num_of_solutions += 1;
			}else{ 
				//se não, como o topo da heap é a maior variância
				//se a variância da solução atual for menor que a do topo
				//remove o topo e insere a nova na heap
				let variance = variancia(&groups_meta, average);
				let mut replace = false;
				if let Some(sol) = heap.peek() {
					if variance < sol.variancia {
						replace = true; //fazer because borrowchecker
					}
				};
				if replace {
					heap.pop();
					heap.push(Solution::new(&groups, variance));
				}
			}
		}
		//pega a ação no topo da pilha
		let Some(State { num_i, group_i, bk_meta }) = bookkeep.pop() else {
			// se não tem mais ações para tomar, acabou
			break;
		};
		//modifica os grupos
		groups_meta = bk_meta;
		//insere o valor na lista de grupos
		groups[group_i * (N/K) + groups_meta[group_i].0] = input[num_i];
		//incrementa o tamanho do grupo
		groups_meta[group_i].0 += 1;
		//incrementa a soma do grupo
		groups_meta[group_i].1 += input[num_i];

		//coloca o índice na posição certa
		i = num_i + 1;
	}

	num_of_solutions += 1;

	return heap;
}


fn greedy_upper_bound<const N: usize, const K: usize>(input: [f64;N]) -> f64{
	let mut groups = [0.0f64;N];
	let mut meta = [(0.0f64, 0usize);K];

	for i in 0..N {
		//seleciona o grupo não cheio com o menor valor
		let (g_i, _) = meta.iter()
			.enumerate()
			.reduce(|acc, x| {
				if x.1.0 < acc.1.0 { x } else { acc }
			})
			.expect("espera-se que meta seja não vazio");
		

		groups[g_i * (N/K) + meta[g_i].1] = input[i];
		meta[g_i].0 += input[i];
		meta[g_i].1 += 1;
	}
	//println!("groups {:?}", groups);
	//println!("meta {:?}", meta);
	return meta.into_iter()
		.map(|v| v.0)
		.reduce(f64::max)
		.expect("meta should not be empty");
}

//const N: usize = 24usize;
//const K: usize = 4usize;

fn get_input<const N: usize>() -> std::io::Result<[f64;N]>{
	let mut inp = [0.0f64; N];

	let contents = std::fs::read_to_string("input.txt")?;
	let mut nums = contents.split_whitespace().map(|w| w.parse::<f64>().unwrap()).collect::<Vec<f64>>();
	if nums.len() != N {
		panic!("entrada com tamanho incorreto, deve ser inserido 24 números!")
	}
	//ordena do maior pro menor
	nums.sort_by(|a, b| b.partial_cmp(a).unwrap());

	for (i, num) in nums.into_iter().enumerate(){
		inp[i] = num;
	}
	println!("inp: {:?}", inp);

	Ok(inp)
}


fn main(){
	let inp = get_input().expect("puts, deu erro no IO");
	//let permutations = fac(N)/(fac(N/K).pow(K as u32) * fac(K));
    //println!("total possible permutations for input: {}", permutations);

	let upperbound = greedy_upper_bound::<24, 4>(inp);
	println!("upper bound solution: {}", upperbound);
	let avg = inp.clone().into_iter().sum::<f64>() / 24.0f64;

	let groups_iter = groups_iterative::<24, 4>(inp, upperbound, avg);
    println!("groups iter: {:?}", groups_iter);

}
