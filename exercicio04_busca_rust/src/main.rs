use std::time::Instant;

fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
        }
    }
    (resultado, operacoes)
}

fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }
    (None, operacoes)
}

fn gerar_vetor(tamanho: usize) -> Vec<i32> {
    (1..=tamanho as i32).collect()
}

fn main() {
    let vetor = gerar_vetor(1000);

    let inicio = Instant::now();
    let (_, ops1) = busca_sequencial_simples(&vetor, 500);
    let tempo1 = inicio.elapsed();

    let inicio = Instant::now();
    let (_, ops2) = busca_sequencial_interrompida(&vetor, 500);
    let tempo2 = inicio.elapsed();

    println!("Busca simples: {} operações {:?}", ops1, tempo1);
    println!("Busca interrompida: {} operações {:?}", ops2, tempo2);
}
