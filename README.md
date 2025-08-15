# Jogo da onça hibrido

Este projeto é uma implementação híbrida do [Jogo da Onça](https://pt.wikipedia.org/wiki/Jogo_da_on%C3%A7a), unindo a simplicidade e flexibilidade do **Python** à alta performance do **Rust**.  
A arquitetura foi projetada para explorar ao máximo as vantagens de cada linguagem: o **Python** é responsável pela interface e pelo gerenciamento geral do jogo, enquanto o **Rust** executa operações críticas com máxima eficiência — incluindo o processamento da inteligência artificial adversária, que utiliza o algoritmo *minimax* com suporte a *multithreading*, aproveitando todo o potencial de paralelismo do Rust.


---

### Código Fonte

* `src/jogo.rs`: O principal arquivo fonte em **Rust** que contém a lógica do jogo.
* `src/lib.rs`: O arquivo de biblioteca **Rust** que provavelmente expõe funções a serem chamadas do Python. É o núcleo da biblioteca compilada `pintada`.
* `board_state.py`: Um arquivo **Python** que gerencia o estado do tabuleiro do jogo.
* `main.py`: O ponto de entrada principal para a aplicação **Python**. Este script conecta todas as partes, usando a lógica do tabuleiro em Python e chamando funções da biblioteca Rust compilada.

---

### Arquivos de Configuração e Compilação

* `Cargo.toml`: O manifesto da parte em **Rust** do projeto. Ele lista informações do projeto e suas dependências.
* `requirements.txt`: Uma lista simples dos pacotes **Python** necessários para o projeto.
---
