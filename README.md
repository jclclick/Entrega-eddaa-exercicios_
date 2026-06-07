# Estruturas de Dados e Análise de Algoritmos

## Aluno
- Nome : Júlio César De Lima Moreira
- Turma: Ciência da Computação turno: noite
- Ra   : 1232021411
# Exercícios Entregues
Atividades pendentes conforme as orientações, o exercício 04 e 07 se encontram nesse repositório. Já o exercício 1 a parte do texto 2 sobre resenha crítica do podcast ("C Versus Python no Aprendizado de Algoritimos") se encontra no final deste documento.

### Exercício 04 – Algoritmos de Busca
## Descrição

Este projeto implementa e compara dois algoritmos de busca em vetores utilizando Rust:

1. Busca Sequencial Simples
2. Busca Sequencial com Interrupção Antecipada

O objetivo é analisar a quantidade de operações executadas e o tempo de execução em diferentes cenários.

## Estrutura de Dados Utilizada

- Vec<i32>

## Complexidade

### Busca Sequencial Simples

Tempo: O(n)

Espaço: O(1)

Executa sempre n operações, independentemente da posição do elemento.

### Busca Sequencial com Interrupção

Melhor caso: O(1)

Pior caso: O(n)

Espaço: O(1)

Interrompe a busca assim que encontra o elemento procurado.

## Como Executar

Instalar Rust:

https://www.rust-lang.org/tools/install

Compilar e executar:

```bash
cargo run --release
```

## Cenários Testados

- Elemento no início do vetor
- Elemento no meio do vetor
- Elemento no final do vetor
- Elemento inexistente

## Conclusão

A busca com interrupção antecipada apresenta melhor desempenho quando o elemento está próximo do início do vetor, reduzindo significativamente a quantidade de operações realizadas.

### Exercício 07 – Estruturas de dados rust

### Grupo 1 — Vec

1. Inversão com push/pop
2. Contador de ocorrências
3. Remoção de pares
4. Mescla ordenada

### Grupo 2 — Pilha

5. Calculadora RPN
6. Histórico navegador
7. Editor com desfazer/refazer
8. Balanceamento de símbolos
9. StackMin

### Grupo 3 — Fila

10. Simulador de banco
11. Impressora compartilhada
12. Buffer circular
13. Fila de prioridade

### Grupo 4 — Deque

14. Palíndromo
15. Janela deslizante máxima
16. Fila de tarefas prioritárias

### Grupo 5 — Reflexão

17. Benchmark de filas
18. Escolha de TAD
19. Processamento em lotes
20. Round Robin

## Como executar

```bash
cargo run
```

## Complexidades

Cada exercício possui comentários explicando a complexidade das operações implementadas.

### Exercício 01/ texto 2 – Resenha crítica do Podcast

# Resenha Crítica – A Dualidade no Ensino de Algoritmos: Abstração vs. Baixo Nível

O podcast aborda uma discussão muito comum entre estudantes de tecnologia: qual é a melhor linguagem para começar a aprender algoritmos e estruturas de dados. Durante o debate, são apresentadas duas visões diferentes. Uma defende o aprendizado com linguagens de baixo nível, como C, enquanto a outra acredita que linguagens mais simples e modernas, como Python, são mais adequadas para quem está começando.

Quem defende a linguagem C acredita que o aluno precisa entender desde cedo como o computador realmente funciona. Nessa abordagem, o estudante aprende conceitos como ponteiros, alocação de memória e gerenciamento manual de recursos. A ideia é que, ao lidar diretamente com esses aspectos, ele desenvolva uma compreensão mais profunda das estruturas de dados e do funcionamento interno dos programas. Além disso, problemas de desempenho e eficiência ficam mais evidentes, permitindo que o aluno perceba na prática a importância de escrever códigos bem estruturados.

Por outro lado, os defensores do Python argumentam que um iniciante não deveria se preocupar imediatamente com detalhes complexos do funcionamento da máquina. Segundo essa visão, o mais importante no começo é aprender a pensar de forma lógica e desenvolver a capacidade de resolver problemas. Como o Python possui uma sintaxe mais simples e cuida automaticamente de várias tarefas internas, o estudante consegue focar mais na construção dos algoritmos e menos em detalhes técnicos que podem gerar confusão.

Um ponto interessante levantado no podcast é a existência do Rust, uma linguagem mais moderna que oferece maior controle sobre a memória, mas de forma mais segura. Isso mostra que o conhecimento de baixo nível continua sendo importante para a área de tecnologia. No entanto, isso não significa necessariamente que seja a melhor porta de entrada para todos os estudantes.

Na minha opinião, o debate mostra que não existe uma única resposta correta. A melhor escolha depende muito dos objetivos de cada pessoa. Quem pretende trabalhar com sistemas operacionais, desenvolvimento embarcado ou áreas que exigem maior proximidade com o hardware provavelmente se beneficiará mais aprendendo C desde o início. Já quem deseja atuar com desenvolvimento web, ciência de dados, inteligência artificial ou automação pode obter resultados mais rápidos começando com Python.

A comparação feita no final do podcast com um carro de Fórmula 1 foi bastante interessante. Assim como ninguém aprende a dirigir em um carro de corrida, também não parece fazer sentido exigir que um iniciante domine conceitos avançados de memória ao mesmo tempo em que está aprendendo estruturas básicas de programação. Isso pode gerar dificuldades desnecessárias e até desmotivação.

Como conclusão, acredito que a abstração não deve ser vista como algo negativo. Pelo contrário, ela facilita o aprendizado e permite que o estudante construa uma boa base de raciocínio lógico. Depois de compreender os conceitos fundamentais, fica muito mais fácil aprofundar os estudos e entender como tudo funciona internamente. Por isso, considero que começar com uma linguagem mais simples e avançar gradualmente para conceitos de baixo nível é uma abordagem equilibrada e eficiente para a maioria dos estudantes.

