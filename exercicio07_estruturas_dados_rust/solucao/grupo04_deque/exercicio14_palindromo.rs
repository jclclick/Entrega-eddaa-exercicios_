use std::collections::VecDeque;

fn eh_palindromo(frase: &str) -> bool {

    let texto: String = frase
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut deque: VecDeque<char> = VecDeque::new();

    for c in texto.chars() {
        deque.push_back(c);
    }

    while deque.len() > 1 {

        let inicio = deque.pop_front().unwrap();
        let fim = deque.pop_back().unwrap();

        if inicio != fim {
            return false;
        }
    }

    true
}

fn main() {

    let frase = "A man a plan a canal Panama";

    if eh_palindromo(frase) {
        println!("É palíndromo!");
    } else {
        println!("Não é palíndromo!");
    }
}

# Complexidade 
push_back() = O(1)

Foi utilizado um VecDeque para comparar simultaneamente os caracteres do início e do final da frase. A cada comparação removo um caractere da frente e outro do fundo.

Como cada caractere participa de apenas uma comparação, o algoritmo percorre a sequência apenas uma vez.

Assim, a complexidade de tempo é O(n) e a complexidade de espaço também é O(n).