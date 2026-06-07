// exercicio06_navegador.rs

struct Navegador {
    atual: String,
    historico_back: Vec<String>,
    historico_forward: Vec<String>,
}

impl Navegador {

    fn visitar(&mut self, pagina: &str) {
        self.historico_back.push(self.atual.clone());
        self.atual = pagina.to_string();
        self.historico_forward.clear();
    }

    fn voltar(&mut self) {
        if let Some(pagina) = self.historico_back.pop() {
            self.historico_forward.push(self.atual.clone());
            self.atual = pagina;
        }
    }

    fn avancar(&mut self) {
        if let Some(pagina) = self.historico_forward.pop() {
            self.historico_back.push(self.atual.clone());
            self.atual = pagina;
        }
    }
}

fn main() {}

# Complexidade
visitar = O(1)

voltar = O(1)

avançar = O(1)
Neste exercício foram utilizadas duas pilhas: uma para armazenar o histórico de páginas visitadas e outra para armazenar as páginas disponíveis para avanço.

As operações de visitar, voltar e avançar utilizam apenas inserções e remoções no topo das pilhas. Como essas operações possuem custo constante, todas as funcionalidades principais possuem complexidade O(1).