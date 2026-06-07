// exercicio07_editor.rs

struct Editor {
    texto: String,
    desfazer: Vec<String>,
    refazer: Vec<String>,
}

impl Editor {

    fn digitar(&mut self, novo: &str) {
        self.desfazer.push(self.texto.clone());
        self.texto.push_str(novo);
        self.refazer.clear();
    }

    fn desfazer(&mut self) {
        if let Some(estado) = self.desfazer.pop() {
            self.refazer.push(self.texto.clone());
            self.texto = estado;
        }
    }

    fn refazer(&mut self) {
        if let Some(estado) = self.refazer.pop() {
            self.desfazer.push(self.texto.clone());
            self.texto = estado;
        }
    }
}

fn main() {}

# Complexidade
digitar = O(n)

desfazer = O(1)

refazer = O(1)

Para implementar o mecanismo de desfazer e refazer foram utilizadas duas pilhas, armazenando estados anteriores do texto.

As operações de empilhar e desempilhar estados possuem custo constante. Entretanto, para salvar um novo estado é necessário copiar o conteúdo atual do texto. Dessa forma, o custo da operação de digitação depende do tamanho da string armazenada.

Por esse motivo, a operação de digitar possui complexidade O(n), enquanto desfazer e refazer possuem complexidade O(1)