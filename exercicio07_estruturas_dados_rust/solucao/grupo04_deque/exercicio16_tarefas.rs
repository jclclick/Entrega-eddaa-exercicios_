use std::collections::VecDeque;

fn main() {

    let mut fila = VecDeque::new();

    fila.push_front("Urgente");

    fila.push_back("Normal 1");

    fila.push_back("Normal 2");

    while let Some(tarefa) = fila.pop_front() {
        println!("{}", tarefa);
    }
}

# Complexidade 

push_front = O(1)

push_back = O(1)

pop_front = O(1)

Nesta implementação tarefas urgentes são inseridas na frente da fila, enquanto tarefas normais são inseridas no final.

Como o VecDeque oferece operações eficientes nas duas extremidades, todas as operações principais possuem custo constante.

Dessa forma, inserir tarefas urgentes, inserir tarefas normais e executar tarefas possuem complexidade O(1).