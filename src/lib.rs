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
/*
fn variancia<const N:usize, const K: usize>(meta: &[(usize, f64);K], average: f64) -> f64 {
    meta.iter()
        .fold(0.0f64,
            |acc, (_, sum)| acc + (sum - average * ((N/K) as f64)).powf(2.0)
    ) / (K as f64)
}

*/
//https://stackoverflow.com/questions/58079910/find-all-ways-to-partition-a-set-into-given-sized-subsets

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
    let mut num_of_solutions: usize = 0;
    let mut heap: BinaryHeap<Solution<N>> = BinaryHeap::with_capacity(MAX_NUMBER_OF_SOLUTIONS);

    let mut i = 0;
    while i < N || !bookkeep.is_empty() {
        if i < N {
            'acess: for j in 0..K {
                //primeiro elemento da tupla representa o tamanho do grupo de mesmo índice
                if groups_meta[j].0 == 0 {
                    //se grupo é vazio
                    bookkeep.push(State {
                        num_i: i,
                        group_i: j,
                        bk_meta: groups_meta,
                    });
                    //nessa regra sai da iteração
                    break 'acess;
                } else if groups_meta[j].0 < (N / K) {
                    //N/K is the size of the group
                    //se a lista está incompleta
                    //e se o adicionar o valor do item no grupo não estora o upperbound
                    if groups_meta[j].1 + input[i] <= upper_bound {
                        bookkeep.push(State {
                            num_i: i,
                            group_i: j,
                            bk_meta: groups_meta,
                        });
                    }
                }
            }
        } else {
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
    heap
}

pub fn greedy_upper_bound<const N: usize, const K: usize>(input: [f64; N]) -> f64 {
    let mut groups = [0.0f64; N];
    let mut meta = [(0.0f64, 0usize); K];

    for item in input.iter().take(N) {
        //seleciona o grupo não cheio com o menor valor
        let (g_i, _) = meta
            .iter()
            .enumerate()
            .reduce(|acc, x| if x.1 .0 < acc.1 .0 { x } else { acc })
            .expect("espera-se que meta seja não vazio");

        groups[g_i * (N / K) + meta[g_i].1] = *item;
        meta[g_i].0 += *item;
        meta[g_i].1 += 1;
    }
    //println!("groups {:?}", groups);
    //println!("meta {:?}", meta);
    meta.into_iter()
        .map(|v| v.0)
        .reduce(f64::max)
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
