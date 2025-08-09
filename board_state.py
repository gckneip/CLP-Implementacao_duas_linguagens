class Casa:
    def __init__(self, id, vizinhos, ocupante=None):
        self.id = id
        self.vizinhos = vizinhos
        self.ocupante = ocupante

class Tabuleiro:
    def __init__(self):
        self.casas = []
        
        self.casas.append(Casa(0, [1, 5, 6]))
        self.casas.append(Casa(1, [0,2,6]))
        self.casas.append(Casa(2, [1,3,6,7,8]))
        self.casas.append(Casa(3, [2,4,8]))
        self.casas.append(Casa(4, [3,8,9]))
        self.casas.append(Casa(5, [0,6,10]))
        self.casas.append(Casa(6, [0,1,2,5,7,10,11,12]))
        self.casas.append(Casa(7, [2,6,8,12]))
        self.casas.append(Casa(8, [2,3,4,7,9,12,13,14]))
        self.casas.append(Casa(9, [4,8,14]))
        self.casas.append(Casa(10, [5,6,11,15,16]))
        self.casas.append(Casa(11, [6,10,12,16]))
        self.casas.append(Casa(12, [6,7,8,11,13,16,17,18]))
        self.casas.append(Casa(13, [8,12,14,18]))
        self.casas.append(Casa(14, [8,9,13,18,19]))
        self.casas.append(Casa(15, [10,16,20]))
        self.casas.append(Casa(16, [10,11,12,15,17,20,21,22]))
        self.casas.append(Casa(17, [12,16,18,22]))
        self.casas.append(Casa(18, [12,13,14,17,19,22,23,24]))
        self.casas.append(Casa(19, [14,18,24]))
        self.casas.append(Casa(20, [15,16,21]))
        self.casas.append(Casa(21, [16,20,22]))
        self.casas.append(Casa(22, [16,17,18,21,23,25,26,27]))
        self.casas.append(Casa(23, [18,22,24]))
        self.casas.append(Casa(24, [18,19,23]))
        self.casas.append(Casa(25, [22,26,28]))
        self.casas.append(Casa(26, [22,25,27,29]))
        self.casas.append(Casa(27, [22,26,30]))
        self.casas.append(Casa(28, [25,29]))
        self.casas.append(Casa(29, [28,26,30]))
        self.casas.append(Casa(30, [27,29]))

        # Onça começa na casa 12
        self.casas[12].ocupante = "Onça"
        
        # 14 cães ocupam as primeiras casas, exceto a 12
        for i in range(14):
            if i!= 12: self.casas[i].ocupante = "Matilha"

    def get_estado(self):
        # Estado do tabuleiro que deverá ser passado ao Rust
        estado = [(casa.id, casa.ocupante) for casa in self.casas]
        return estado