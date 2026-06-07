use std::collections::VecDeque;
use std::time::Instant;

// =====================
// FILA COM VEC
// =====================

fn teste_vec() {

    let mut fila: Vec<i32> = Vec::new();

    let inicio = Instant::now();

    for i in 0..10_000 {
        fila.push(i);
    }

    while !fila.is_empty() {
        fila.remove(0);
    }

    let tempo = inicio.elapsed();

    println!("Vec Ingênua: {:?}", tempo);
}

// =====================
// FILA COM VECDEQUE
// =====================

fn teste_vecdeque() {

    let mut fila = VecDeque::new();

    let inicio = Instant::now();

    for i in 0..10_000 {
        fila.push_back(i);
    }

    while !fila.is_empty() {
        fila.pop_front();
    }

    let tempo = inicio.elapsed();

    println!("VecDeque: {:?}", tempo);
}

// =====================
// FILA CIRCULAR
// =====================

struct FilaCircular {

    dados: Vec<Option<i32>>,
    inicio: usize,
    fim: usize,
    tamanho: usize,
    capacidade: usize,
}

impl FilaCircular {

    fn new(capacidade: usize) -> Self {

        Self {
            dados: vec![None; capacidade],
            inicio: 0,
            fim: 0,
            tamanho: 0,
            capacidade,
        }
    }

    fn enqueue(&mut self, valor: i32) {

        if self.tamanho == self.capacidade {
            return;
        }

        self.dados[self.fim] = Some(valor);

        self.fim = (self.fim + 1) % self.capacidade;

        self.tamanho += 1;
    }

    fn dequeue(&mut self) -> Option<i32> {

        if self.tamanho == 0 {
            return None;
        }

        let valor = self.dados[self.inicio].take();

        self.inicio = (self.inicio + 1) % self.capacidade;

        self.tamanho -= 1;

        valor
    }
}

fn teste_fila_circular() {

    let mut fila = FilaCircular::new(10_000);

    let inicio = Instant::now();

    for i in 0..10_000 {
        fila.enqueue(i);
    }

    while fila.dequeue().is_some() {}

    let tempo = inicio.elapsed();

    println!("Fila Circular: {:?}", tempo);
}

// =====================
// MAIN
// =====================

fn main() {

    println!("Benchmark com 10.000 elementos\n");

    teste_vec();

    teste_vecdeque();

    teste_fila_circular();
}

# Complexidade 
enqueue = O(1)

dequeue = O(1)
Exercício 17 – Comparação de Desempenho

Vec:
- enqueue: O(1)
- dequeue: O(n)

VecDeque:
- enqueue: O(1)
- dequeue: O(1)

Fila Circular:
- enqueue: O(1)
- dequeue: O(1)

Conclusão:

VecDeque e Fila Circular apresentaram desempenho
significativamente superior à implementação ingênua
com Vec, pois evitam o deslocamento de elementos
durante a remoção.