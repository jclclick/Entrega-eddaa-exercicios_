struct Item {
    valor: String,
    prioridade: u32,
}

struct FilaPrioridade {
    itens: Vec<Item>,
}

impl FilaPrioridade {

    fn inserir(
        &mut self,
        valor: String,
        prioridade: u32
    ) {

        self.itens.push(
            Item {
                valor,
                prioridade,
            }
        );
    }

    fn remover(&mut self) -> Option<String> {

        if self.itens.is_empty() {
            return None;
        }

        let mut indice = 0;

        for i in 1..self.itens.len() {

            if self.itens[i].prioridade
                > self.itens[indice].prioridade
            {
                indice = i;
            }
        }

        Some(
            self.itens.remove(indice).valor
        )
    }
}

# Complexidade 
Inserir = O(1)

Remover = O(n)

Nesta implementação a inserção dos elementos ocorre normalmente no final da estrutura. Entretanto, para remover o elemento de maior prioridade é necessário percorrer toda a fila procurando o item adequado.

Por esse motivo, a inserção possui complexidade O(1), enquanto a remoção possui complexidade O(n).