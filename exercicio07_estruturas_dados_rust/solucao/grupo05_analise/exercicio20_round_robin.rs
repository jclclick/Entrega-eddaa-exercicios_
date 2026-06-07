use std::collections::VecDeque;

struct Processo {
    id: u32,
    tempo_restante: u32,
}

fn main() {

    let quantum = 2;

    let mut fila = VecDeque::new();

    fila.push_back(
        Processo {
            id: 1,
            tempo_restante: 5,
        }
    );

    fila.push_back(
        Processo {
            id: 2,
            tempo_restante: 7,
        }
    );

    while let Some(mut p) = fila.pop_front() {

        if p.tempo_restante > quantum {

            p.tempo_restante -= quantum;

            fila.push_back(p);

        } else {

            println!(
                "Processo {} concluído",
                p.id
            );
        }
    }
}

#Complexidade 
enqueue = O(1)

dequeue = O(1)

Round Robin completo:

O(n + ciclos)

A fila circular permite que os processos sejam reposicionados rapidamente ao final da fila sempre que ainda possuem tempo restante. Como as operações de inserção e remoção possuem custo constante, o desempenho do algoritmo depende principalmente da quantidade de ciclos necessários para concluir todos os processos.