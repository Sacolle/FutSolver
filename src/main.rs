
use num_bigint::BigUint;


fn fac(mut i: usize) -> BigUint {
	assert!(i > 0);
	let mut acc = BigUint::ZERO + 1usize;
	while i > 0 {
		acc = acc * i;
		i -= 1usize;
	}
	acc
}

struct State<const K: usize> {
	num_i: usize,
	group_i: usize,
	bk_gp_sizes: [usize;K]
}

//https://stackoverflow.com/questions/58079910/find-all-ways-to-partition-a-set-into-given-sized-subsets
fn groups_iterative<const N: usize, const K: usize>(input: [f32;N]) -> BigUint{
	let mut bookkeep: Vec<State<K>> = vec![];

	let mut groups: [f32;N] = [0.0f32; N];
	let mut groups_sizes: [usize;K] = [0usize; K];
	let mut i = 0;

	let mut num_of_solutions = BigUint::ZERO;
	//N is input size
	while i < N || bookkeep.len() > 0{
		if i < N {
			'acess: for j in 0..K{
				//segundo elemento da tupla representa o tamanho
				if groups_sizes[j] == 0 {
					//se grupo é vazio
					bookkeep.push(State { num_i: i, group_i: j, bk_gp_sizes: groups_sizes.clone() });
					//nessa regra sai da iteração
					break 'acess;
				}else if groups_sizes[j] < (N/K) { //N/K is the size of the group
					//se a lista está incompleta 
					//NOTE: checa aqui se estoraria o upperBound, 
					//se não adiciona
					bookkeep.push(State { num_i: i, group_i: j, bk_gp_sizes: groups_sizes.clone() });
				}
			}
		}else{
			//aqui a list está completa, mas o bookeep não necessariamente está vazio
			//ou seja, groups é uma solução
			/*if num_of_solutions.clone() % 10000000u32 == BigUint::ZERO {
				println!("■");
			}*/
			num_of_solutions += 1u32;
		}
		//pega a ação no topo da pilha
		let Some(State { num_i, group_i, bk_gp_sizes }) = bookkeep.pop() else {
			// se não tem mais ações para tomar, acabou
			break;
		};
		//modifica os grupos
		groups_sizes = bk_gp_sizes;
		//insere o valor na lista de grupos
		groups[group_i * K + groups_sizes[group_i]] = input[num_i];
		//incrementa o tamanho do grupo
		groups_sizes[group_i] += 1;

		//coloca o índice na posição certa
		i = num_i + 1;
	}

	num_of_solutions += 1u32;
	return num_of_solutions;
}


fn greedy_upper_bound<const N: usize, const K: usize>(input: [f32;N]) -> f32{
	let mut groups = [0.0f32;N];
	let mut meta = [(0.0f32, 0usize);K];

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
		.reduce(f32::max)
		.expect("meta should not be empty");
}

fn main() -> std::io::Result<()> {
	const N: usize = 24usize;
	const K: usize = 4usize;
	let inp = [1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32,
		7.0f32, 8.0f32, 9.0f32, 10.0f32, 11.0f32, 12.0f32,
		13.0f32, 14.0f32, 15.0f32, 16.0f32, 17.0f32, 18.0f32,
		19.0f32, 20.0f32, 21.0f32, 22.0f32, 23.0f32, 24.0f32
	];

	let permutations = fac(N)/(fac(N/K).pow(K as u32) * fac(K));

    println!("permutations: {}", permutations);

	let upperbound = greedy_upper_bound::<24, 4>(inp);
	println!("upper bound solution: {}", upperbound);
	//let groups_iter = groups_iterative::<24, 4>(inp);
    //println!("groups iter: {}", groups_iter);

	Ok(())
}
