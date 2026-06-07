fn remover_pares(v: &mut Vec<i32>) {
    let mut i = 0;

    while i < v.len() {
        if v[i] % 2 == 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

# Complexidade

remove(i) desloca elementos

Pior caso:
O(n²)

remove(i) desloca elementos

Pior caso:
O(n²)

Para remover os números pares sem utilizar o método retain(), percorri o vetor verificando cada elemento. Quando um número par é encontrado, utilizo remove() para excluí-lo.

O ponto importante é que a operação remove() desloca todos os elementos que estão à direita da posição removida. Portanto, em vetores grandes, esse deslocamento pode ocorrer muitas vezes.

No pior caso, quando há muitos números pares para remover, a complexidade da solução torna-se O(n²). A vantagem é que não foi necessário criar estruturas auxiliares, mantendo a complexidade de espaço em O(1)