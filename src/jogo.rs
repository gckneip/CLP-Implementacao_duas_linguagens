use pyo3::prelude::*;
use std::thread;
use std::collections::HashMap;

#[pyclass]
pub struct Jogo {
    posicoes: HashMap<usize, String>,
    vez: String,
    vizinhos: HashMap<usize, Vec<usize>>,
    saltos: HashMap<usize, Vec<(usize, usize)>>,
    cpu: String,
    ultimas_jogadas: HashMap<String, (usize, usize)>,
    contagem_repeticoes: HashMap<String, u8>,
}

#[pymethods]
impl Jogo {
    #[new]
    pub fn new(cpu: String) -> Self {
        let mut posicoes = HashMap::new();
        let cpu = cpu.to_string();

        // Estado inicial: Onça na 12
        posicoes.insert(12, "Onça".to_string());

        // Matilha nas casas 0..14 exceto 12
        for i in 0..15 {
            if i != 12 {
                posicoes.insert(i, "Matilha".to_string());
            }
        }

        let vizinhos = Self::mapa_vizinhos();
        let saltos = Self::mapa_saltos();

        Jogo {
            cpu,
            posicoes,
            vez: "Onça".to_string(),
            vizinhos,
            saltos,
            ultimas_jogadas: HashMap::new(),
            contagem_repeticoes: HashMap::new(),
        }
    }

    fn estado(&self) -> HashMap<usize, String> {
        self.posicoes.clone()
    }

    fn vez(&self) -> String {
        self.vez.clone()
    }

    fn aplicar_jogada(&mut self, from: usize, to: usize) -> PyResult<bool> {
        if let Some(jogador) = self.posicoes.get(&from) {
            if *jogador != self.vez {
                return Ok(false);
            }

            // Movimento normal
            if self.vizinhos.get(&from).unwrap_or(&vec![]).contains(&to) {
                if self.posicoes.contains_key(&to) {
                    println!("Jogada inválida: posição de destino {} já ocupada", to);
                    return Ok(false);
                }
                self.mover(from, to);
                let jogador_da_vez = self.vez.clone();
                let jogada_atual = (from, to);

                let mut contador = self.contagem_repeticoes.entry(jogador_da_vez.clone()).or_insert(0);

                if let Some(ultima_jogada) = self.ultimas_jogadas.get(&jogador_da_vez) {
                    if *ultima_jogada == (to, from) {
                        *contador += 1;
                    } else {
                        *contador = 1;
                    }
                } else {
                    *contador = 1;
                }

                self.ultimas_jogadas.insert(jogador_da_vez.clone(), jogada_atual);
    
                return Ok(true);
            }

            // Movimento de captura
            if *jogador == "Onça" {
                if let Some(saltos) = self.saltos.get(&from) {
                    for (inter, destino) in saltos {
                        if *destino == to {
                            if self.posicoes.get(inter) == Some(&"Matilha".to_string())
                                && !self.posicoes.contains_key(&to)
                            {
                                self.posicoes.remove(inter);
                                self.mover(from, to);
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }
    fn jogada_cpu(&mut self) -> PyResult<()> {
        let jogador_da_vez = self.vez.clone();
        let profundidade = 5;
        let jogadas_possiveis = self.obter_jogadas_possiveis(&jogador_da_vez);

        if jogadas_possiveis.is_empty() {
            return Ok(());
        }

        let mut handles = Vec::new();

        for (from, to) in jogadas_possiveis.clone() {
            let mut jogo_clone = self.clone();
            let jogador_da_vez_clone = jogador_da_vez.clone();

            let handle = thread::spawn(move || {
                let (captura, peca_movida, estado_anterior_jogada, estado_anterior_repeticoes) = jogo_clone.simular_jogada(from, to);

                let (_, pontuacao) = jogo_clone.minimax(
                    profundidade - 1,
                    false, 
                    if jogador_da_vez_clone == "Matilha" { "Onça" } else { "Matilha" },
                );

                jogo_clone.desfazer_jogada(from, to, captura, peca_movida, estado_anterior_jogada, estado_anterior_repeticoes);

                (from, to, pontuacao)
            });
            handles.push(handle);
        }
        let mut melhor_jogada = Some(jogadas_possiveis[0]);
        let mut melhor_pontuacao = i32::min_value();
        for (from, to) in jogadas_possiveis.clone() {
            let mut jogo_clone = self.clone();
            let jogador_da_vez_clone = jogador_da_vez.clone();

            let handle = thread::spawn(move || {
                let (captura, peca_movida, estado_anterior_jogada, estado_anterior_repeticoes) = jogo_clone.simular_jogada(from, to);

                let (_, pontuacao) = jogo_clone.minimax(
                    profundidade - 1,
                    false, 
                    if jogador_da_vez_clone == "Matilha" { "Onça" } else { "Matilha" },
                );

                jogo_clone.desfazer_jogada(from, to, captura, peca_movida, estado_anterior_jogada, estado_anterior_repeticoes);

                (from, to, pontuacao)
            });
            handles.push(handle);
        }

        let mut melhor_pontuacao = i32::min_value();
        let mut melhor_jogada = None;

        for handle in handles {
            let (from, to, pontuacao) = handle.join().unwrap();
            if pontuacao > melhor_pontuacao {
                melhor_pontuacao = pontuacao;
                melhor_jogada = Some((from, to));
            }
        }

        if let Some((from, to)) = melhor_jogada {
            self.aplicar_jogada(from, to)?;
        }
      Ok(())
    } 

    pub fn jogo_terminou(&self) -> bool {
        if self.obter_jogadas_possiveis("Onça").is_empty() {
            return true;
        }
        let pecas_matilha = self.posicoes.values().filter(|p| p.as_str() == "Matilha").count();
        if pecas_matilha < 9 {
            return true;
        }
        if let Some(contador) = self.contagem_repeticoes.get(&self.vez) {
        if *contador >= 3 {
            return true;
        }
    } 
    false
  }
}

impl Jogo {
    fn obter_jogadas_possiveis(&self, jogador: &str) -> Vec<(usize, usize)> {
        let mut jogadas = Vec::new();
        for (&from_pos, dono) in &self.posicoes {
            if dono == jogador {
                if let Some(vizinhos) = self.vizinhos.get(&from_pos) {
                    for &to_pos in vizinhos {
                        if !self.posicoes.contains_key(&to_pos) {
                            jogadas.push((from_pos, to_pos));
                        }
                    }
                }
                if dono == "Onça" {
                    if let Some(saltos) = self.saltos.get(&from_pos) {
                        for (inter, destino) in saltos {
                            if self.posicoes.get(inter) == Some(&"Matilha".to_string()) {
                                if !self.posicoes.contains_key(&destino) {
                                    jogadas.push((from_pos, *destino));
                                }
                            }
                        }
                    }
                }
            }
        }
        jogadas
    }

fn simular_jogada(&mut self, from: usize, to: usize) -> (Option<(usize, String)>, String, HashMap<String, (usize, usize)>, HashMap<String, u8>) {
    let mut captura = None;
    let peca_movida = self.posicoes.remove(&from).unwrap();

    if peca_movida == "Onça" {
        if let Some(saltos) = self.saltos.get(&from) {
            for (inter, destino) in saltos {
                if *destino == to {
                    let peca_capturada = self.posicoes.remove(inter).unwrap();
                    captura = Some((*inter, peca_capturada));
                    break;
                }
            }
        }
    }
    
    let ultimas_jogadas_anterior = self.ultimas_jogadas.clone();
    let contagem_repeticoes_anterior = self.contagem_repeticoes.clone();

    let jogador_da_vez = self.vez.clone();
    let jogada_atual = (from, to);
    let mut contador = self.contagem_repeticoes.entry(jogador_da_vez.clone()).or_insert(0);

    if let Some(ultima_jogada) = self.ultimas_jogadas.get(&jogador_da_vez) {
        if *ultima_jogada == jogada_atual {
            *contador += 1;
        } else {
            *contador = 1;
        }
    } else {
        *contador = 1;
    }
    self.ultimas_jogadas.insert(jogador_da_vez.clone(), jogada_atual);

    self.posicoes.insert(to, peca_movida.clone());

    (captura, peca_movida, ultimas_jogadas_anterior, contagem_repeticoes_anterior)
}

fn desfazer_jogada(&mut self, from: usize, to: usize, captura: Option<(usize, String)>, peca_movida: String, ultimas_jogadas_anterior: HashMap<String, (usize, usize)>, contagem_repeticoes_anterior: HashMap<String, u8>) {
    self.posicoes.remove(&to);
    self.posicoes.insert(from, peca_movida);
    
    if let Some((pos_captura, peca_capturada)) = captura {
        self.posicoes.insert(pos_captura, peca_capturada);
    }
    
    self.ultimas_jogadas = ultimas_jogadas_anterior;
    self.contagem_repeticoes = contagem_repeticoes_anterior;
}
    fn avaliar_tabuleiro_matilha(&self) -> i32 {
        if self.jogo_terminou() {
            if self.obter_jogadas_possiveis("Onça").is_empty() {
                return i32::max_value(); 
            }
            let pecas_matilha = self.posicoes.values().filter(|p| p.as_str() == "Matilha").count();
            if pecas_matilha < 9 {
                return i32::min_value(); 
            }
        }
       if let Some(contador) = self.contagem_repeticoes.get(&self.vez) {
            if *contador >= 3 {
                if self.vez == "Matilha" {
                    return i32::min_value();
                } else {
                    return i32::max_value();
                }
            }
        }

        let peso_pecas = 100;
        let peso_mobilidade_onca = -50; 
        let peso_posicao_central_onca = -20; 

        let mut pontuacao = 0;

        let pecas_matilha = self.posicoes.values().filter(|p| *p == "Matilha").count();
        pontuacao += pecas_matilha as i32 * peso_pecas;

        let mut mobilidade_onca = 0;
        let pos_onca = self.posicoes.iter().find(|(_, &ref p)| p == "Onça").map(|(&k, _)| k);

        if let Some(pos) = pos_onca {
            if let Some(vizinhos) = self.vizinhos.get(&pos) {
                for &vizinho_pos in vizinhos {
                    if !self.posicoes.contains_key(&vizinho_pos) {
                        mobilidade_onca += 1;
                    }
                }
            }
        }
        pontuacao += mobilidade_onca * peso_mobilidade_onca;

        if let Some(pos) = pos_onca {
            if pos == 12 {
                pontuacao += peso_posicao_central_onca;
            }
        }

        if mobilidade_onca == 0 && pos_onca.is_some() {
            pontuacao = i32::max_value();
        }

        pontuacao
    }

    fn avaliar_tabuleiro_onca(&self) -> i32 {
      let peso_pecas = -100;
      let peso_mobilidade_onca = 50;
      let peso_posicao_central_onca = 20;

      let mut pontuacao = 0;

      let pecas_matilha = self.posicoes.values().filter(|p| *p == "Matilha").count();
      pontuacao += pecas_matilha as i32 * peso_pecas;

      let mut mobilidade_onca = 0;
      let pos_onca = self
          .posicoes
          .iter()
          .find(|(_, &ref p)| p == "Onça")
          .map(|(&k, _)| k);

      if let Some(pos) = pos_onca {
          if let Some(vizinhos) = self.vizinhos.get(&pos) {
              for &vizinho_pos in vizinhos {
                  if !self.posicoes.contains_key(&vizinho_pos) {
                      mobilidade_onca += 1;
                  }
              }
          }
      }
      pontuacao += mobilidade_onca * peso_mobilidade_onca;

      if let Some(pos) = pos_onca {
          if pos == 12 {
              pontuacao += peso_posicao_central_onca;
          }
      }

      if mobilidade_onca == 0 && pos_onca.is_some() {
          // caught in the undertow just caught in the undertow
          pontuacao = i32::MIN;
      }

      pontuacao
  }

  fn minimax(
    &mut self,
    profundidade: u8,
    is_max_player: bool,
    jogador_da_vez: &str, 
  ) -> (Option<(usize, usize)>, i32) {
    if profundidade == 0 || self.jogo_terminou() { 
        let mut pontuacao = if self.cpu == "Matilha" { self.avaliar_tabuleiro_matilha()}  else {self.avaliar_tabuleiro_onca()};
        if !is_max_player {
            pontuacao *= -1;
        }
        return (None, pontuacao);
    }
    

    let mut melhor_pontuacao = if is_max_player { i32::min_value() } else { i32::max_value() };
    let mut melhor_jogada = None;

    let jogadas_possiveis = self.obter_jogadas_possiveis(jogador_da_vez);

    for (from, to) in jogadas_possiveis {
        let (captura, peca_movida, estado_anterior_jogada, estado_anterior_repeticoes) = self.simular_jogada(from, to);

        let (_, pontuacao) = self.minimax(
            profundidade - 1, 
            !is_max_player, 
            if jogador_da_vez == "Matilha" { "Onça" } else { "Matilha" }
        );

        self.desfazer_jogada(from, to, captura, peca_movida, estado_anterior_jogada, estado_anterior_repeticoes);

        if is_max_player {
            if pontuacao > melhor_pontuacao {
                melhor_pontuacao = pontuacao;
                melhor_jogada = Some((from, to));
            }
        } else {
            if pontuacao < melhor_pontuacao {
                melhor_pontuacao = pontuacao;
                melhor_jogada = Some((from, to));
            }
        }
    }

    (melhor_jogada, melhor_pontuacao)
}
    fn mover(&mut self, from: usize, to: usize) {
        let peca = self.posicoes.remove(&from).unwrap();
        self.posicoes.insert(to, peca);
        self.vez = if self.vez == "Onça" {
            "Matilha".to_string()
        } else {
            "Onça".to_string()
        };
    }

    fn mapa_vizinhos() -> HashMap<usize, Vec<usize>> {
        let mut v = HashMap::new();

        let adj = vec![
            (0, vec![1, 5, 6]),
            (1, vec![0, 2, 6]),
            (2, vec![1, 3, 6, 7, 8]),
            (3, vec![2, 4, 8]),
            (4, vec![3, 8, 9]),
            (5, vec![0, 6, 10]),
            (6, vec![0, 1, 2, 5, 7, 10, 11, 12]),
            (7, vec![2, 6, 8, 12]),
            (8, vec![2, 3, 4, 7, 9, 12, 13, 14]),
            (9, vec![4, 8, 14]),
            (10, vec![5, 6, 11, 15, 16]),
            (11, vec![6, 10, 12, 16]),
            (12, vec![6, 7, 8, 11, 13, 16, 17, 18]),
            (13, vec![8, 12, 14, 18]),
            (14, vec![8, 9, 13, 18, 19]),
            (15, vec![10, 16, 20]),
            (16, vec![10, 11, 12, 15, 17, 20, 21, 22]),
            (17, vec![12, 16, 18, 22]),
            (18, vec![12, 13, 14, 17, 19, 22, 23, 24]),
            (19, vec![14, 18, 24]),
            (20, vec![15, 16, 21]),
            (21, vec![16, 20, 22]),
            (22, vec![16, 17, 18, 21, 23, 25, 26, 27]),
            (23, vec![18, 22, 24]),
            (24, vec![18, 19, 23]),
            (25, vec![22, 26, 28]),
            (26, vec![22, 25, 27, 29]),
            (27, vec![22, 26, 30]),
            (28, vec![25, 29]),
            (29, vec![26, 28, 30]),
            (30, vec![27, 29]),
        ];

        for (k, ns) in adj {
            v.insert(k, ns);
        }
        v
    }

    fn mapa_saltos() -> HashMap<usize, Vec<(usize, usize)>> {
        let mut s = HashMap::new();

        // Saltos diagonais do quadrado e saltos completos no triângulo
        let jumps = vec![
            (0, vec![(6, 12)]),
            (2, vec![(6, 10), (8, 14)]),
            (4, vec![(8, 12)]),
            (6, vec![(12, 18)]),
            (8, vec![(12, 16)]),
            (10, vec![(6, 2), (16, 22)]),
            (12, vec![(6, 0), (8, 4), (16, 20), (18, 24)]),
            (14, vec![(8, 2), (18, 22)]),
            (16, vec![(12, 8), (22, 27)]),
            (17, vec![(22, 26)]),
            (18, vec![(12, 6), (22, 25)]),
            (20, vec![(16, 12)]),
            (22, vec![(16, 10), (18, 14), (25, 28), (26,29), (27, 30)]),
            (24, vec![(18, 12)]),
            (25, vec![(22, 18), (26, 27)]),
            (26, vec![(22, 17)]),
            (27, vec![(22, 16), (26, 25)]),
            (28, vec![(25, 22), (29, 30)]),
            (29, vec![(26, 22)]),
            (30, vec![(27, 22), (29, 28)]),
        ];
        for (k, js) in jumps {
            s.insert(k, js);
        }
        // Saltos horizontais para o grid 5x5
        for row in 0..5 {
            for col in 0..3 {
                let from = row * 5 + col;
                let inter = from + 1;
                let to = from + 2;

                s.entry(from).or_insert_with(Vec::new).push((inter, to));
                s.entry(to).or_insert_with(Vec::new).push((inter, from));
            }
        }

        // Saltos verticais para o grid 5x5
        for row in 0..3 {
            for col in 0..5 {
                let from = row * 5 + col;
                let inter = from + 5;
                let to = from + 10;

                s.entry(from).or_insert_with(Vec::new).push((inter, to));
                s.entry(to).or_insert_with(Vec::new).push((inter, from));
            }
        }

        s
    }
}

impl Clone for Jogo {
    fn clone(&self) -> Self {
        Jogo {
            posicoes: self.posicoes.clone(),
            vez: self.vez.clone(),
            vizinhos: self.vizinhos.clone(),
            saltos: self.saltos.clone(),
            cpu: self.cpu.clone(),
            ultimas_jogadas: self.ultimas_jogadas.clone(),
            contagem_repeticoes: self.contagem_repeticoes.clone(),
        }
    }
}

