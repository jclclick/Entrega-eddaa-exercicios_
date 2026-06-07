fn avaliar(expr: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a+b);
            }
            "-" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a-b);
            }
            "*" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a*b);
            }
            "/" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a/b);
            }
            _ => pilha.push(token.parse().unwrap())
        }
    }

    pilha.pop().unwrap()
}

# Complexidade 
push/pop = O(1)

Percorre expressão uma vez

Total = O(n)

A calculadora foi implementada utilizando uma pilha baseada em Vec<f64>. Cada número encontrado na expressão é empilhado e cada operador remove os operandos necessários da pilha para realizar o cálculo.

As operações de empilhar e desempilhar possuem custo constante. Como cada token da expressão é processado apenas uma vez, o algoritmo executa uma quantidade de operações proporcional ao tamanho da entrada.

Assim, a complexidade de tempo é O(n) e a complexidade de espaço também é O(n) devido ao uso da pilha.