use std::collections::VecDeque;

fn maximos_janelas(
    nums: Vec<i32>,
    k: usize,
) -> Vec<i32> {

    let mut resultado = Vec::new();
    let mut deque = VecDeque::new();

    for i in 0..nums.len() {

        while let Some(&j) = deque.back() {

            if nums[j] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        if deque.front() == Some(&(i - k)) {
            deque.pop_front();
        }

        if i + 1 >= k {

            resultado.push(
                nums[*deque.front().unwrap()]
            );
        }
    }

    resultado
}

fn main() {}

# Complexidade
Tempo = O(n)

Para resolver este problema foi utilizado um VecDeque para armazenar os índices dos elementos candidatos ao valor máximo de cada janela.

Apesar de existirem laços internos, cada elemento entra e sai do deque no máximo uma vez durante toda a execução.

Por esse motivo, a complexidade de tempo da solução é O(n) e a complexidade de espaço é O(k), onde k representa o tamanho da janela