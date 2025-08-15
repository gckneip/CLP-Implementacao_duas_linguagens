# Documentação da Implementação - Jogo da Onça Híbrido

## Visão Geral

Este projeto é uma implementação híbrida do [Jogo da Onça](https://pt.wikipedia.org/wiki/Jogo_da_on%C3%A7a), combinando a simplicidade e flexibilidade do **Python** com a alta performance do **Rust**.  

A arquitetura do projeto foi projetada para aproveitar as vantagens de cada linguagem:

- **Python**: Responsável pela interface do usuário, gerenciamento geral do jogo e controle do fluxo da aplicação.  
- **Rust**: Executa operações críticas com alta performance, incluindo o cálculo da inteligência artificial adversária utilizando o algoritmo *minimax* com suporte a *multithreading*.

---

## Estrutura do Código Fonte

| Arquivo | Linguagem | Função |
|---------|-----------|--------|
| `src/jogo.rs` | Rust | Contém a lógica principal do jogo, incluindo regras e movimentação da IA. |
| `src/lib.rs` | Rust | Biblioteca que expõe funções para serem chamadas pelo Python. Núcleo da biblioteca `pintada`. |
| `board_state.py` | Python | Gerencia o estado do tabuleiro do jogo, armazenando posições e movimentos. |
| `main.py` | Python | Ponto de entrada da aplicação, conecta a lógica do tabuleiro em Python com funções da biblioteca Rust. |

---

## Configuração e Dependências

**Rust**: Configurado via `Cargo.toml` com todas as dependências necessárias para a compilação da biblioteca `pintada`.

**Python**: Dependências listadas no arquivo `requirements.txt`.  
Para instalar:

```bash
pip install -r requirements.txt
```

## Interface entre Python e Rust

A comunicação entre **Python** e **Rust** é feita por meio da biblioteca `pintada`, compilada com **[maturin](https://www.maturin.rs/tutorial.html)**, que permite chamar funções Rust diretamente em Python. O **maturin** utiliza a interface de interoperabilidade em C, compatível tanto com Rust quanto com Python, para realizar a integração entre as duas linguagens de forma eficiente.

Processo de compilação e execução:

#### 1. Ative o ambiente Python e instale as dependências:
```bash
pip install -r requirements.txt
```
#### 2. Compile a biblioteca Rust para Python:
```bash
 maturin develop
 ```
#### 3. Execute o programa principal
 ```bash
 python main.py
```

## Criado por
- **Gustavo Cunha Kneip**
- **Antônio Araújo de Brum**
- **Matheus Renan Freitas de Freitas**