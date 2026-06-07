use std::collections::VecDeque;

struct Buffer {

    capacidade: usize,
    dados: VecDeque<String>,
}

impl Buffer {

    fn adicionar(&mut self, msg: String) {

        if self.dados.len() == self.capacidade {
            self.dados.pop_front();
        }

        self.dados.push_back(msg);
    }
}

fn main() {}

# Comlexidade 
O(1)

O buffer foi implementado utilizando uma fila de capacidade limitada. Quando o buffer está cheio, a mensagem mais antiga é descartada automaticamente.

As operações de inserção e remoção realizadas pelo VecDeque possuem custo constante. Por isso, a complexidade de cada operação é O(1).