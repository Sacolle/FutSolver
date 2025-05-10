const list = [24.74, 21.2, 13.67, 7.32, 6.6, 4.48, 23.17, 22.28, 
19.63, 9.46, 2.28, 1.21, 20.54, 18.54, 15.7, 14.1, 5.93, 3.18, 17.77, 16.4, 12.92, 11.22, 10.73, 8.96]

const N = 24;
const K = 4;

const avg = list.reduce((acc, curr) => acc + curr, 0) / N;
let solutions = new Array(K).fill(0)

for(let i = 0; i < K; i++){
	for(let j = 0; j < (N/K); j++){
		solutions[i] += list[i * (N/K) + j]
	}
}

console.log(solutions)