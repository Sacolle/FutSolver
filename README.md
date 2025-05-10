# Fut Solver

Esse projeto busca gerar todas as permutações de uma lista de **n** elementos em **k** grupos de **s** tamanho, considerando que **k * s = n**. Nisso, me baseei neste [post do stack overflow](https://stackoverflow.com/questions/58079910/find-all-ways-to-partition-a-set-into-given-sized-subsets)


Há muitos artigos escritos sobre isso, considerando que esse é o problema [Optimal Multi-Way Number Partitioning](https://escholarship.org/content/qt30g6n09q/qt30g6n09q_noSplash_ef9faa8716151ddad94c088224843a04.pdf?t=nmfl8z). Eu li uns artigos para ter uma ideia. Existe formas mais optimais de fazer isso, mas do jeito que está sendo feito é suficiente para os valores de **n** que eu espero.

## Como usar

Altera o input.txt com os valores, tem que ter 24 elementos.
Roda com:
```
	cargo run --release
```