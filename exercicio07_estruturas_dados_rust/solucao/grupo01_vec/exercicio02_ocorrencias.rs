use std::collections::HashMap;

fn contar(v: Vec<char>) {
    let mut mapa = HashMap::new();

    for letra in &v {
        *mapa.entry(*letra).or_insert(0) += 1;
    }

    println!("{:?}", mapa);
}

# Complexidade
Iteração: O(n)
HashMap: O(1) médio

Total: O(n)

Neste exercício percorri o vetor de caracteres utilizando apenas for x in &vec, conforme solicitado. Para armazenar a quantidade de ocorrências de cada letra utilizei um HashMap.

Como cada caractere é visitado apenas uma vez durante a iteração, o custo principal da solução é proporcional ao tamanho do vetor. As operações de inserção e atualização no HashMap possuem custo médio constante.

Por esse motivo, a complexidade de tempo é O(n). Já a complexidade de espaço depende da quantidade de caracteres diferentes encontrados, sendo O(k), onde k representa o número de letras distintas.