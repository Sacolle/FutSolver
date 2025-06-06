use futsolver::{get_input, greedy_upper_bound, groups_iterative, Solution};

fn main() {
    let Some(filename) = std::env::args().nth(1) else {
        panic!("Missing filename from arguments");
    };

    let inp = get_input::<24>(filename.as_str()).expect("puts, deu erro no IO");

    let upperbound = greedy_upper_bound::<24, 4>(inp);
    println!("upper bound solution: {}", upperbound);

    let groups_iter = groups_iterative::<24, 4>(inp, upperbound);

    //indo do menor para o maior
    for Solution {
        solution,
        amplitude,
    } in groups_iter.into_sorted_vec().into_iter().take(10)
    {
        println!("Amplitude: {}\nSolução: {:?}", amplitude, solution);
    }
}
