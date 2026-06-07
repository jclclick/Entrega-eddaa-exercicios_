fn inverter(mut v: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();

    while let Some(valor) = v.pop() {
        resultado.push(valor);
    }

    resultado
}

fn main() {
    let v = vec![1,2,3,4,5];

    println!("{:?}", inverter(v));
}

# Complexidade 

pop() = O(1)
push() = O(1)

Total = O(n)
Para inverter o vetor, utilizei apenas as operações pop() e push(), conforme solicitado no enunciado. A cada iteração removo o último elemento do vetor original e o adiciono em um novo vetor. Como cada elemento é removido e inserido apenas uma vez, o algoritmo percorre todos os elementos do vetor uma única vez.

Dessa forma, a complexidade de tempo é O(n), onde n representa a quantidade de elementos do vetor. Em relação ao espaço, foi necessário criar um novo vetor para armazenar o resultado da inversão, resultando em complexidade de espaço O(n).