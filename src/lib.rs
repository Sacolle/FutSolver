use std::collections::BinaryHeap;

pub struct State<const K: usize> {
    num_i: usize,
    group_i: usize,
    bk_meta: [(usize, f64); K],
}

#[derive(Debug)]
pub struct Solution<const N: usize> {
    pub solution: [f64; N],
    pub amplitude: f64,
}

impl<const N: usize> Solution<N> {
    pub fn new(solution: &[f64; N], amplitude: f64) -> Self {
        Solution {
            solution: *solution,
            amplitude,
        }
    }
}

impl<const N: usize> Ord for Solution<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amplitude.total_cmp(&other.amplitude)
    }
}

impl<const N: usize> PartialOrd for Solution<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> PartialEq for Solution<N> {
    fn eq(&self, other: &Self) -> bool {
        self.amplitude == other.amplitude
    }
}
impl<const N: usize> Eq for Solution<N> {}
//https://stackoverflow.com/questions/58079910/find-all-ways-to-partition-a-set-into-given-sized-subsets
// https://stackoverflow.com/a/58086043

//N is input size
//K é o número de grupos
// N/K é o tamanho de um grupo
pub fn groups_iterative<const N: usize, const K: usize>(
    input: [f64; N],
    mut upper_bound: f64,
) -> BinaryHeap<Solution<N>> {
    let mut groups: [f64; N] = [0.0f64; N];
    let mut groups_meta: [(usize, f64); K] = [(0usize, 0.0f64); K];
    let mut bookkeep: Vec<State<K>> = vec![];

    const MAX_NUMBER_OF_SOLUTIONS: usize = 100;
    let max_items_per_group = N / K;
    let mut num_of_solutions: usize = 0;
    let mut heap: BinaryHeap<Solution<N>> = BinaryHeap::with_capacity(MAX_NUMBER_OF_SOLUTIONS);

    let mut i = 0;
    while i < N || !bookkeep.is_empty() {
        if i < N {
            // se i < N, então ainda tem itens para adicionar
            'access: for j in 0..K {
                //primeiro elemento da tupla representa o tamanho do grupo de mesmo índice
                if groups_meta[j].0 == 0 {
                    //se esse conj. está vazio então adiciona o item,
                    println!("Adicionando {} no grupo vazio {}", input[i], j);
                    bookkeep.push(State {
                        num_i: i,             // índice do item que está sendo adicionado
                        group_i: j,           // índice do grupo onde o item está sendo adicionado
                        bk_meta: groups_meta, // Copia do estado atual dos grupos
                    });
                    //nessa regra sai da iteração, pois o item só pode ser adicionado ao
                    // primeiro grupo vazio, ignorando os outros grupos
                    break 'access;
                } else if groups_meta[j].0 < max_items_per_group {
                    //se conj j não está cheio, então adiciona o item se não estoura o upperbound
                    if groups_meta[j].1 + input[i] <= upper_bound {
                        // adiciona o item ao grupo j
                        println!("Adicionando {} no grupo {}", input[i], j);
                        bookkeep.push(State {
                            num_i: i,
                            group_i: j,
                            bk_meta: groups_meta,
                        });
                    } else {
                        //se estoura o upperbound, não adiciona
                        println!(
                            "Não adicionando {} no grupo {} porque estoura o upperbound",
                            input[i], j
                        );
                    }
                }
            }
        } else {
            // se i >= N, então não tem mais itens para adicionar, mas ainda tem ações no bookkeep
            // get the min and the max of solutions
            let (s_min, s_max) = groups_meta
                .iter()
                .fold((f64::MAX, f64::MIN), |(acc_min, acc_max), (_, v)| {
                    (f64::min(*v, acc_min), f64::max(*v, acc_max))
                });
            // if the new solution is lower then the upperbound, update the upperbound
            // new optimization
            if s_max < upper_bound {
                upper_bound = s_max;
            }
            let amplitude = s_max - s_min;
            //aqui a list está completa, mas o bookeep não necessariamente está vazio
            //ou seja, groups é uma solução melhor que o upperbound
            //se não coletou MAX_NUMBER_OF_SOLUTIONS, coleta
            if num_of_solutions < MAX_NUMBER_OF_SOLUTIONS {
                heap.push(Solution::new(&groups, amplitude));
                num_of_solutions += 1;
            } else {
                //se não, como o topo da heap é a maior amplitude
                //se a amplitude da solução atual for menor que a do topo
                //remove o topo e insere a nova na heap
                let mut replace = false;
                if let Some(sol) = heap.peek() {
                    if amplitude < sol.amplitude {
                        replace = true; //fazer because borrowchecker
                    }
                };
                if replace {
                    heap.pop();
                    heap.push(Solution::new(&groups, amplitude));
                }
            }
        }
        //pega a ação no topo da pilha
        let Some(State {
            num_i,
            group_i,
            bk_meta,
        }) = bookkeep.pop()
        else {
            // se não tem mais ações para tomar, acabou
            break;
        };
        //modifica os grupos
        groups_meta = bk_meta;
        //insere o valor na lista de grupos
        groups[group_i * (N / K) + groups_meta[group_i].0] = input[num_i];
        //incrementa o tamanho do grupo
        groups_meta[group_i].0 += 1;
        //incrementa a soma do grupo
        groups_meta[group_i].1 += input[num_i];

        //coloca o índice na posição certa
        i = num_i + 1;
    }
    // Adiciona a última solução na heap, se ainda não foi adicionada
    let (s_min, s_max) = groups_meta
        .iter()
        .fold((f64::MAX, f64::MIN), |(acc_min, acc_max), (_, v)| {
            (f64::min(*v, acc_min), f64::max(*v, acc_max))
        });
    let amplitude = s_max - s_min;
    if num_of_solutions < MAX_NUMBER_OF_SOLUTIONS {
        heap.push(Solution::new(&groups, amplitude));
    } else {
        // se não, como o topo da heap é a maior amplitude
        // se a amplitude da solução atual for menor que a do topo
        // remove o topo e insere a nova na heap
        let mut replace = false;
        if let Some(sol) = heap.peek() {
            if amplitude < sol.amplitude {
                replace = true; // fazer because borrowchecker
            }
        };
        if replace {
            heap.pop();
            heap.push(Solution::new(&groups, amplitude));
        }
    }

    heap
}

pub fn greedy_upper_bound<const N: usize, const K: usize>(input: [f64; N]) -> f64 {
    let mut meta = [(0.0f64, 0usize); K];
    let max_group_size: usize = N / K;

    println!("Gerando upper bound para N={} e K={}", N, K);
    for item in input.iter() {
        //seleciona o grupo não cheio com o menor valor
        let (g_i, _) = meta
            .iter()
            .enumerate()
            .filter(|(_, v)| v.1 < max_group_size)
            .min_by(|(_, v1), (_, v2)| v1.0.partial_cmp(&v2.0).unwrap())
            .expect("There should be at least one group with space");

        println!("Adicionando {} no grupo {}", item, g_i);
        meta[g_i].0 += *item;
        meta[g_i].1 += 1;
    }
    println!("Upper bound meta: {:?}", meta);
    //println!("groups {:?}", groups);
    //println!("meta {:?}", meta);
    meta.into_iter()
        .map(|v| v.0)
        .max_by(|a, b| a.partial_cmp(b).expect("max should not fail"))
        .expect("meta should not be empty")
}

//const N: usize = 24usize;
//const K: usize = 4usize;

pub fn read_nums_from_file(filename: &str) -> std::io::Result<Vec<f64>> {
    let contents = std::fs::read_to_string(filename)?;
    let nums = contents
        .split_whitespace()
        .map(|w| w.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    Ok(nums)
}

pub fn prepare_input<const N: usize>(mut nums: Vec<f64>) -> [f64; N] {
    if nums.len() != N {
        panic!(
            "entrada com tamanho incorreto, deve ser inserido {} números!",
            N
        );
    }
    // Ordena do maior pro menor
    nums.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut inp = [0.0f64; N];
    for (i, num) in nums.into_iter().enumerate() {
        inp[i] = num;
    }
    println!("inp: {:?}", inp);

    inp
}

pub fn groups<const N: usize, const K: usize>(input: [f64; N]) -> BinaryHeap<Solution<N>> {
    assert!(N % K == 0, "N must be divisible by K");
    assert!(K > 0, "K must be greater than 0");
    assert!(N > 0, "N must be greater than 0");
    assert!(input.len() == N, "Input length must match N");
    let prepared_input = prepare_input::<N>(input.to_vec());
    let upper_bound = greedy_upper_bound::<N, K>(prepared_input);
    println!("Upper bound for N={} and K={} is {}", N, K, upper_bound);
    groups_iterative::<N, K>(prepared_input, upper_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_upper_bound() {
        let input = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let prepared_input = prepare_input::<6>(input.to_vec());
        let upper_bound = greedy_upper_bound::<6, 3>(prepared_input);
        assert_eq!(upper_bound, 7.0);
    }

    #[test]
    fn test_prepare_input() {
        let nums = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let prepared = prepare_input::<5>(nums);
        assert_eq!(prepared, [5.0, 4.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_greedy_upper_bound_2() {
        let input = [100.0, 100.1, 10.0, 10.1, 1.0, 1.1];
        let prepared_input = prepare_input::<6>(input.to_vec());
        let upper_bound = greedy_upper_bound::<6, 3>(prepared_input);
        println!("Upper bound: {}", upper_bound);
        assert_eq!(upper_bound, 101.1);
    }

    #[test]
    fn test_groups_iterative() {
        let input = [100.0, 100.1, 10.0, 10.1, 1.0, 1.1];
        let heap = groups::<6, 3>(input);
        assert!(!heap.is_empty());
        let best_solution = heap.peek().expect("Heap should not be empty");
        assert_eq!(best_solution.solution.len(), 6);
        assert!(best_solution.amplitude >= 0.0);
        assert_eq!(best_solution.amplitude, 81.0);
    }
    #[test]
    fn test_groups_iterative_2() {
        let input = [100.0, 100.1, 10.0, 10.1, 1.0, 1.1];
        let heap = groups::<6, 2>(input);
        assert!(!heap.is_empty());
        let best_solution = heap.peek().expect("Heap should not be empty");
        assert_eq!(best_solution.solution.len(), 6);
        assert!(best_solution.amplitude >= 0.0);
        assert!(
            best_solution.amplitude - 0.1 < f64::EPSILON,
            "Expected amplitude to be close to 0.1, got {}",
            best_solution.amplitude
        );
    }
}
