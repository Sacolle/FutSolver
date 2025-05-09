use std::fs;
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

struct State {
	num_i: usize,
	group_i: usize,
	group_sizes: Vec<usize>
}

impl State {
	fn new<T>(i: usize, j: usize, groups: &Vec<(Vec<Option<T>>, usize)>) -> Self{
		State { 
			num_i: i, 
			group_i: j, 
			group_sizes: groups.iter().map(
				|(_, size)| *size
			).collect::<Vec<usize>>()
		}
	}
}

//https://stackoverflow.com/questions/58079910/find-all-ways-to-partition-a-set-into-given-sized-subsets
fn groups_iterative<T: Copy>(input: &[T], input_size: usize, group_size: usize) -> usize{
	let mut bookkeep: Vec<State> = vec![];

	let mut groups: Vec<(Vec<Option<T>>,usize)> = vec![(vec![None; group_size], 0); input_size / group_size];
	let mut i = 0;

	let mut num_of_solutions = BigUint::ZERO;

	while i < input_size || bookkeep.len() > 0{

		if i < input_size {
			'acess: for j in 0..groups.len(){
				//segundo elemento da tupla representa o tamanho
				if groups[j].1 == 0 {
					//se grupo é vazio
					bookkeep.push(State::new(i, j, &groups));
					//nessa regra sai da iteração
					break 'acess;
				}else if groups[j].1 < group_size {
					//se a lista está incompleta 
					bookkeep.push(State::new(i, j, &groups));
				}
			}
		}else{
			//aqui a list está completa, mas o bookeep não necessariamente está vazio
			//ou seja, groups é uma solução
			/*if num_of_solutions % 10000000 == 0 {
				println!("■");
			}*/
			num_of_solutions += 1u32;
		}
		//pega a ação no topo da pilha
		let Some(State { num_i, group_i, group_sizes }) = bookkeep.pop() else {
			// se não tem mais ações para tomar, acabou
			break;
		};
		//modifica os grupos
		for j in 0..groups.len() {
			//ajeita o tamanho dos grupos segundo o estado
			groups[j].1 = group_sizes[j];
		}
		//insere o valor na lista de grupos
		let target_size = groups[group_i].1;
		groups[group_i].0[target_size] = Some(input[num_i]);
		//incrementa o tamanho do grupo
		groups[group_i].1 += 1;

		//coloca o índice na posição certa
		i = num_i + 1;
	}

	num_of_solutions += 1u32;
	return num_of_solutions;
}

fn main() -> std::io::Result<()> {
	let constents = fs::read_to_string("info.txt")?;
	let input: Vec<f64> = constents.trim()
		.split_whitespace()
		.map(|s| s.parse::<f64>().unwrap())
		.collect();

	let n = input[0] as usize;
	let s = input[1] as usize;
	let permutations = fac(n)/(fac(s).pow((n/s) as u32) * fac(n/s));


    println!("permutations: {}", permutations);

	let groups_iter = groups_iterative(&input[2..], n, s);
    println!("groups iter: {}", groups_iter);

	Ok(())
}
