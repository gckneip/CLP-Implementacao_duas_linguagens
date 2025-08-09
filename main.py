import tkinter as tk
from tkinter import messagebox
from board_state import Tabuleiro  
# import pintada # O seu módulo Rust

# Classe Menu
class MenuFrame(tk.Frame):
    def __init__(self, master, switch_to_board_callback):
        super().__init__(master)

        self.switch_to_board_callback = switch_to_board_callback

        tk.Label(self, text="Bem vindo ao", font=("Arial", 16)).pack(pady=10)
        tk.Label(self, text="JOGO DA ONÇA", font=("Arial", 32, "bold"), fg="#D2691E").pack(pady=10)
        tk.Label(self, text="Deseja jogar como...").pack(pady=20)

        tk.Button(self, text="Onça", command=lambda: self.switch_to_board_callback("Onça")).pack(pady=5)
        tk.Button(self, text="Matilha", command=lambda: self.switch_to_board_callback("Matilha")).pack(pady=5)

# Classe Tabuleiro
class TabuleiroFrame(tk.Frame):
    def __init__(self, master, escolha_do_jogador):
        super().__init__(master)
        

        self.escolha_do_jogador = escolha_do_jogador
        self.modelo_tabuleiro = Tabuleiro()
        
        tk.Label(self, text=f"Sua vez, você joga como {self.escolha_do_jogador}").pack(pady=10)
        
    
        self.canvas = tk.Canvas(self, width=600, height=800, bg="white")
        self.canvas.pack(pady=10)

        # Cria as coordenadas das casas
        # Eventualmente será interessante definir as coordenadas com base em proporções do tamanho do canvas
        self.coordenadas_casas = self.gerar_coordenadas_5x5()
        self.coordenadas_casas[25] = (230, 600)
        self.coordenadas_casas[26] = (300, 600)
        self.coordenadas_casas[27] = (370, 600)
        self.coordenadas_casas[28] = (160, 700)
        self.coordenadas_casas[29] = (300, 700)
        self.coordenadas_casas[30] = (440, 700)
        
        # Desenha o tabuleiro
        self.desenhar_linhas()
        self.casas_gui = {} 
        self.desenhar_casas()

    def gerar_coordenadas_5x5(self): 
        coords = {}
        margin = 100
        spacing = 100
        rows = 5
        cols = 5
        
        for row in range(rows):
            for col in range(cols):
                house_id = row * cols + col
                x = margin + col * spacing
                y = margin + row * spacing
                coords[house_id] = (x, y)
        return coords

    def desenhar_linhas(self):
        for casa in self.modelo_tabuleiro.casas:
            x1, y1 = self.coordenadas_casas[casa.id]
            
            for vizinho_id in casa.vizinhos:
                if vizinho_id > casa.id:
                    x2, y2 = self.coordenadas_casas[vizinho_id]
                    
                    self.canvas.create_line(x1, y1, x2, y2, fill="black", width=2)

    def desenhar_casas(self):
        """
        Desenha os círculos das casas.
        """
        for id_casa, (x, y) in self.coordenadas_casas.items():
            casa_id = self.canvas.create_oval(
                x - 15, y - 15, x + 15, y + 15,
                fill="lightgray", outline="black"
            )
            self.casas_gui[id_casa] = casa_id



class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Jogo da Onça")
        self.geometry("800x800")

        self.menu_frame = MenuFrame(self, self.show_board_frame)
        self.tabuleiro_frame = None

        self.show_menu_frame()

    def show_menu_frame(self):
        if self.tabuleiro_frame:
            self.tabuleiro_frame.pack_forget()
        self.menu_frame.pack(expand=True, fill='both')

    def show_board_frame(self, escolha_do_jogador):
        self.menu_frame.pack_forget()
        
        self.tabuleiro_frame = TabuleiroFrame(self, escolha_do_jogador)
        self.tabuleiro_frame.pack(expand=True, fill='both')


if __name__ == "__main__":
    app = App()
    app.mainloop()