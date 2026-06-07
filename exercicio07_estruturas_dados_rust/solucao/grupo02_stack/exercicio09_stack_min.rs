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
