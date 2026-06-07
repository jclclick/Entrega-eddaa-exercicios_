fn mesclar(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();

    resultado.extend(v1);
    resultado.extend(v2);

    resultado.sort();

    resultado
}

# Complexidade 

extend = O(n+m)

sort = O((n+m) log(n+m))

Total:
O((n+m) log(n+m))

A solução consistiu em juntar os dois vetores utilizando extend() e, em seguida, ordenar o vetor resultante utilizando sort().

A operação de junção percorre todos os elementos dos dois vetores, enquanto a ordenação possui custo maior e acaba dominando a análise de desempenho.

Por isso, a complexidade final é O((n+m) log(n+m)), onde n e m representam os tamanhos dos vetores originais.