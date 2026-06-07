use std::collections::VecDeque;

fn main() {

    let mut fila = VecDeque::new();

    fila.push_back("Cliente 1");
    fila.push_back("Cliente 2");
    fila.push_back("Cliente 3");

    while let Some(cliente) = fila.pop_front() {
        println!("Atendendo {}", cliente);
    }
}

# Complexidade 
enqueue = O(1)

dequeue = O(1)

A fila foi implementada utilizando VecDeque, permitindo inserções no final e remoções no início em tempo constante.

Cada cliente entra na fila uma única vez e é atendido uma única vez. Dessa forma, a simulação completa possui complexidade O(n).