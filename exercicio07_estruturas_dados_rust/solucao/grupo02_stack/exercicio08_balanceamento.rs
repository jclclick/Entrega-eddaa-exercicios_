fn balanceado(expressao: &str) -> bool {
    let mut pilha: Vec<char> = Vec::new();

    for c in expressao.chars() {

        match c {

            '(' | '[' | '{' => {
                pilha.push(c);
            }

            ')' => {
                if pilha.pop() != Some('(') {
                    return false;
                }
            }

            ']' => {
                if pilha.pop() != Some('[') {
                    return false;
                }
            }

            '}' => {
                if pilha.pop() != Some('{') {
                    return false;
                }
            }

            _ => {}
        }
    }

    pilha.is_empty()
}

fn main() {

    let exemplos = vec![
        "{[()]}",
        "([)]",
        "(((",
        "{[]()}",
        "{[(])}",
    ];

    for expr in exemplos {
        println!("{} -> {}", expr, balanceado(expr));
    }
}

# Complexidade

Tempo: O(n)

Espaço: O(n)

Neste exercício foi utilizada uma pilha para armazenar os símbolos de abertura encontrados na expressão. Sempre que encontro um símbolo de fechamento, verifico se ele corresponde ao último símbolo aberto armazenado na pilha.

Cada caractere da expressão é analisado apenas uma única vez durante a execução do algoritmo. Além disso, as operações realizadas na pilha (push e pop) possuem custo constante.

Dessa forma, a complexidade de tempo da solução é O(n), onde n representa a quantidade de caracteres da expressão. A complexidade de espaço também é O(n), pois, no pior caso, todos os símbolos de abertura podem ser armazenados simultaneamente na pilha.