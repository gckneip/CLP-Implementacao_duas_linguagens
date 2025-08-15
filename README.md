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

### Como executar
Crie um environment python (mais informações em [env python](https://docs.python.org/pt-br/3.13/tutorial/venv.html)): 
```bash
python -m venv nome-do-env
```
Dentro do ambiente python instale as dependências utilizando o comando abaixo:
```bash
pip install -r requirements.txt
```

Então, dentro do env, rode o maturin para compilar o código em **Rust**:
```bash
maturin develop
```

Por fim, rode o programa em **Python**:
```bash
python main.py
```

### Aproveite o jogo!
Se tiver duvidas de como jogar, leia as [regras do jogo da onça](https://nova-escola-producao.s3.amazonaws.com/vYfWM25yYpjxM4rXaa6BaHXMsSSjNZDy5nabwEZfr6DPxfzBE5qjFwXJ2JJS/his7-09und05-regras-do-jogo-da-onca.pdf)!
## Criado por
- **Gustavo Cunha Kneip**
- **Antônio Araújo de Brum**
- **Matheus Renan Freitas de Freitas**
