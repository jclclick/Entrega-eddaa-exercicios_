struct StackMin {
    dados: Vec<i32>,
    mins: Vec<i32>,
}

impl StackMin {

    fn push(&mut self, valor: i32) {

        self.dados.push(valor);

        if self.mins.is_empty()
            || valor <= *self.mins.last().unwrap()
        {
            self.mins.push(valor);
        }
    }

    fn pop(&mut self) -> Option<i32> {

        let valor = self.dados.pop()?;

        if valor == *self.mins.last().unwrap() {
            self.mins.pop();
        }

        Some(valor)
    }

    fn min(&self) -> Option<i32> {
        self.mins.last().copied()
    }
}
# Complexidade 
push = O(1)

pop = O(1)

min = O(1)
Para permitir a consulta do menor elemento em tempo constante, utilizei uma pilha auxiliar que acompanha os menores valores já inseridos.

Dessa forma, não é necessário percorrer toda a pilha para descobrir o menor elemento. Basta consultar o topo da pilha auxiliar.

Com essa estratégia, as operações push(), pop() e min() possuem complexidade O(1).