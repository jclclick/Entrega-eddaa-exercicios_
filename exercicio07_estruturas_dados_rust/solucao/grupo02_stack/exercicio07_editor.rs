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
enqueue = O(1)

dequeue = O(1)