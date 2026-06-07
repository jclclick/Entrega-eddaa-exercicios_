use std::collections::VecDeque;

fn processar_em_lotes(
    fila: &mut VecDeque<i32>,
    tamanho_lote: usize,
) {

    while !fila.is_empty() {

        for _ in 0..tamanho_lote {

            if let Some(valor) =
                fila.pop_front()
            {
                print!("{} ", valor);
            }
        }

        println!();
    }
}

fn main() {}

# Complexidade 
O(n)

Cada elemento é removido e processado apenas uma vez. Como pop_front() possui custo O(1), o custo total depende apenas da quantidade de elementos existentes na fila.
