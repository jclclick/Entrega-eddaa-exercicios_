use std::collections::VecDeque;

struct Trabalho {
    nome: String,
    paginas: u32,
}

fn main() {

    let mut fila = VecDeque::new();

    fila.push_back(
        Trabalho {
            nome: "Relatorio".to_string(),
            paginas: 10,
        }
    );

    fila.push_back(
        Trabalho {
            nome: "Contrato".to_string(),
            paginas: 3,
        }
    );

    while let Some(job) = fila.pop_front() {
        println!(
            "Imprimindo {} ({} páginas)",
            job.nome,
            job.paginas
        );
    }
}

# Complexidade 
Inserir trabalho (enqueue): O(1)

Remover trabalho para impressão (dequeue): O(1)

Processamento completo da fila: O(n)

Espaço: O(n)

A complexidade total é O(n) porque cada trabalho é processado uma única vez. Como as operações de inserção e remoção em VecDeque possuem custo constante, o tempo de execução cresce proporcionalmente à quantidade de trabalhos presentes na fila.

Além disso, a fila precisa armazenar todos os trabalhos que ainda não foram impressos. Por esse motivo, a complexidade de espaço é O(n).

