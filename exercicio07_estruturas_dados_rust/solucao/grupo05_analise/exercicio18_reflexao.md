# Exercício 18 – Quando usar qual TAD?

## a) Implementar o botão "Ctrl+Z" de um editor

### TAD escolhido:
Pilha (Stack)

### Justificativa:
O comando Ctrl+Z desfaz a última ação realizada. Como a última ação inserida é a primeira a ser removida, o comportamento segue o princípio LIFO (Last In, First Out).

### Complexidade:
- Push: O(1)
- Pop: O(1)

---

## b) Processar pedidos de um restaurante em ordem

### TAD escolhido:
Fila (Queue)

### Justificativa:
Os pedidos devem ser atendidos na mesma ordem em que chegam. O primeiro pedido recebido deve ser o primeiro a ser preparado.

Segue o princípio FIFO (First In, First Out).

### Complexidade:
- Enqueue: O(1)
- Dequeue: O(1)

---

## c) Verificar se um arquivo HTML tem tags bem formadas

### TAD escolhido:
Pilha (Stack)

### Justificativa:
Quando uma tag é aberta, ela é empilhada. Quando uma tag de fechamento é encontrada, verifica-se se corresponde à última tag aberta.

Exemplo:

<html>
  <body>
  </body>
</html>

A última tag aberta deve ser a primeira a ser fechada.

### Complexidade:
- Push: O(1)
- Pop: O(1)

---

## d) Navegar nos arquivos de um diretório em largura

### TAD escolhido:
Fila (Queue)

### Justificativa:
A busca em largura (Breadth-First Search - BFS) visita todos os elementos de um nível antes de passar para o próximo.

Esse comportamento exige FIFO.

### Complexidade:
- Enqueue: O(1)
- Dequeue: O(1)

---

## e) Verificar se uma sequência de palavras é palíndromo

### TAD escolhido:
Deque (Double-Ended Queue)

### Justificativa:
Para verificar um palíndromo é necessário comparar simultaneamente os elementos do início e do final da sequência.

O Deque permite:

- push_front()
- push_back()
- pop_front()
- pop_back()

todos em tempo constante.

### Complexidade:
- Inserção: O(1)
- Remoção: O(1)
- Verificação completa: O(n)

---

# Conclusão

Cada TAD foi escolhido de acordo com suas características:

| Situação | TAD |
|-----------|------|
| Ctrl+Z | Pilha |
| Pedidos de restaurante | Fila |
| Verificação de HTML | Pilha |
| Busca em largura (BFS) | Fila |
| Palíndromo | Deque |

A escolha correta da estrutura de dados impacta diretamente na simplicidade da implementação e no desempenho das operações.