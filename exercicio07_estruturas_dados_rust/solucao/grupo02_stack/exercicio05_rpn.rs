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