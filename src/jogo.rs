use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct Jogo {
    posicoes: HashMap<usize, String>,
    vez: String,
    vizinhos: HashMap<usize, Vec<usize>>,
    saltos: HashMap<usize, Vec<(usize, usize)>>,
}

#[pymethods]
impl Jogo {
    #[new]
    pub fn new() -> Self {
        let mut posicoes = HashMap::new();

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
            posicoes,
            vez: "Onça".to_string(),
            vizinhos,
            saltos,
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
                    return Ok(false);
                }
                self.mover(from, to);
                return Ok(true);
            }

            // Movimento de captura (somente Onça)
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
        let jogador = self.vez.clone();
        for (&from, dono) in &self.posicoes {
            if *dono == jogador {
                for &dest in self.vizinhos.get(&from).unwrap_or(&vec![]) {
                    if !self.posicoes.contains_key(&dest) {
                        self.mover(from, dest);
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }
}

impl Jogo {
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
            (20, vec![15, 16, 21, 25]),
            (21, vec![16, 20, 22, 25, 26, 27]),
            (22, vec![16, 17, 18, 21, 23, 26]),
            (23, vec![18, 22, 24, 26, 27, 30]),
            (24, vec![18, 19, 23, 27]),
            (25, vec![20, 21, 26, 28]),
            (26, vec![21, 22, 23, 25, 27, 28, 29, 30]),
            (27, vec![21, 24, 23, 26, 30]),
            (28, vec![25, 26, 29]),
            (29, vec![26, 28, 30]),
            (30, vec![26, 27, 29]),
        ];

        for (k, ns) in adj {
            v.insert(k, ns);
        }
        v
    }

    fn mapa_saltos() -> HashMap<usize, Vec<(usize, usize)>> {
        let mut s = HashMap::new();

        let jumps = vec![
            (0, vec![(1, 2), (5, 10), (6, 12)]),
            (1, vec![(6, 11)]),
            (2, vec![(1, 0), (3, 4), (6, 10), (7, 12), (8, 14)]),
            (3, vec![(2, 1), (8, 13)]),
            (4, vec![(3, 2), (8, 12), (9, 14)]),
            (5, vec![(6, 7), (10, 15)]),
            (6, vec![(0, 1), (2, 3), (10, 20), (12, 18)]),
            (7, vec![(2, 1), (12, 17)]),
            (8, vec![(2, 0), (4, 9), (12, 16), (14, 19)]),
            (9, vec![(4, 3), (8, 13)]),
            (10, vec![(5, 0), (6, 2), (15, 20), (16, 22)]),
            (11, vec![(6, 1), (12, 13)]),
            (12, vec![(6, 0), (7, 2), (8, 4), (16, 20), (17, 22), (18, 24)]),
            (13, vec![(8, 3), (12, 11)]),
            (14, vec![(8, 2), (9, 4), (18, 23), (19, 24)]),
            (15, vec![(10, 5), (16, 17)]),
            (16, vec![(10, 6), (12, 8), (20, 25), (22, 27)]),
            (17, vec![(12, 7), (18, 19)]),
            (18, vec![(12, 6), (13, 8), (14, 9), (22, 26), (23, 27), (24, 30)]),
            (19, vec![(14, 8), (18, 17)]),
            (20, vec![(15, 10), (16, 12), (25, 28)]),
            (21, vec![(16, 11), (22, 23), (25, 29), (26, 30)]),
            (22, vec![(16, 10), (17, 12), (18, 14), (26, 28)]),
            (23, vec![(18, 12), (26, 29)]),
            (24, vec![(18, 13), (27, 30)]),
            (25, vec![(20, 15), (21, 16), (28, 29)]),
            (26, vec![(21, 20), (22, 18), (23, 19), (28, 30)]),
            (27, vec![(21, 16), (24, 19), (30, 29)]),
            (28, vec![(25, 20), (26, 22)]),
            (29, vec![(25, 21), (26, 23)]),
            (30, vec![(26, 22), (27, 24)]),
        ];

        for (k, js) in jumps {
            s.insert(k, js);
        }
        s
    }
}
