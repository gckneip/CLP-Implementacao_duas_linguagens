import tkinter as tk
from tkinter import messagebox
from PIL import Image, ImageTk
from board_state import Tabuleiro 
import pintada  # Módulo Rust

# Classe Menu
class MenuFrame(tk.Frame):
    def __init__(self, master, switch_to_board_callback):
        super().__init__(master)
        self.switch_to_board_callback = switch_to_board_callback

        tk.Label(self, text="Bem vindo ao", font=("Arial", 16)).pack(pady=10)
        tk.Label(self, text="JOGO DA ONÇA", font=("Arial", 32, "bold"), fg="#D2691E").pack(pady=10)

        pintada_img = Image.open("./assets/pintada.jpg")
        matilha_img = Image.open("./assets/matilha.png")
        self.pintada_img = ImageTk.PhotoImage(pintada_img)
        self.matilha_img = ImageTk.PhotoImage(matilha_img)

        tk.Label(self, text="Deseja jogar como...").pack(pady=20)
        tk.Button(self, text="Onça", command=lambda: self.switch_to_board_callback("Onça")).pack(pady=5)
        tk.Label(self, image=self.pintada_img).pack(pady=10)
        tk.Button(self, text="Matilha", command=lambda: self.switch_to_board_callback("Matilha")).pack(pady=5)
        tk.Label(self, image=self.matilha_img).pack(pady=10)

# Classe Tabuleiro
class TabuleiroFrame(tk.Frame):
    def __init__(self, master, escolha_do_jogador):
        super().__init__(master)

        self.escolha_do_jogador = escolha_do_jogador
        self.jogo_rust = pintada.Jogo("Matilha" if escolha_do_jogador == "Onça" else "Onça")  # Instância do jogo no Rust
        self.modelo_tabuleiro = Tabuleiro()

        tk.Label(self, text=f"Sua vez, você joga como {self.escolha_do_jogador}").pack(pady=10)
        self.canvas = tk.Canvas(self, width=600, height=800, bg="white")
        self.canvas.pack(pady=10)

        # Gera coordenadas das casas
        self.coordenadas_casas = self.gerar_coordenadas_5x5()
        self.coordenadas_casas[25] = (230, 600)
        self.coordenadas_casas[26] = (300, 600)
        self.coordenadas_casas[27] = (370, 600)
        self.coordenadas_casas[28] = (160, 700)
        self.coordenadas_casas[29] = (300, 700)
        self.coordenadas_casas[30] = (440, 700)

        self.selecao = []
        self.desenhar_linhas()
        self.casas_gui = {}
        self.desenhar_casas()
        self.atualizar_tabuleiro()

        self.canvas.bind("<Button-1>", self.on_click)
        self.verificar_turno_cpu()

    def verificar_turno_cpu(self):
        """Verifica se é a vez da CPU e, se for, executa a jogada."""
        jogador_da_vez = self.jogo_rust.vez()
        # O oponente do jogador humano é a CPU
        oponente = "Matilha" if self.escolha_do_jogador == "Onça" else "Onça"
                                                
        if jogador_da_vez == oponente:
            self.jogo_rust.jogada_cpu()
            self.atualizar_tabuleiro()
            # Adicione uma verificação de fim de jogo aqui se desejar

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
        pass

    def desenhar_casas(self):
        for id_casa, (x, y) in self.coordenadas_casas.items():
            casa_id = self.canvas.create_oval(
                x - 15, y - 15, x + 15, y + 15,
                fill="lightgray", outline="black", width=3
            )
            self.casas_gui[id_casa] = casa_id

    def atualizar_tabuleiro(self):
      estado = self.jogo_rust.estado()  # dict {pos: "Onça" ou "Matilha"}
      for pos, casa_id in self.casas_gui.items():
          cor = "lightgray"  # cor padrão
          if pos in estado:
              ocupante = estado[pos]
              if ocupante == "Onça":
                  cor = "orange"
              elif ocupante == "Matilha":
                  cor = "blue"
          self.canvas.itemconfig(casa_id, fill=cor, outline="black")

      if len(self.selecao) == 1:
        pos_destaque = self.selecao[0]
        casa_id = self.casas_gui.get(pos_destaque)
        if casa_id is not None:
            self.canvas.itemconfig(casa_id, outline="red", width=3)

    def on_click(self, event):
      x, y = event.x, event.y
      casa_encontrada = False

      for pos, (cx, cy) in self.coordenadas_casas.items():
          if (x - cx)**2 + (y - cy)**2 <= 15**2:
              casa_encontrada = True
              
              if not self.selecao:
                  self.selecao.append(pos)
              else:
                  from_pos, to_pos = self.selecao[0], pos
                  self.selecao.append(pos)
                  
                  try:
                      sucesso = self.jogo_rust.aplicar_jogada(from_pos, to_pos)
                  except Exception as e:
                      messagebox.showerror("Erro", f"Falha ao aplicar jogada: {e}")
                      sucesso = False

                  if sucesso:
                      print(f"Jogada de {self.escolha_do_jogador} de {from_pos} para {to_pos} bem-sucedida.")
                      self.atualizar_tabuleiro()
                      if self.jogo_rust.jogo_terminou():
                        messagebox.showinfo("Fim de Jogo", "O jogo terminou!")
                      else:
                      # Turno da CPU
                        self.after(100, self.verificar_turno_cpu)
                        self.jogo_rust.jogada_cpu()
                        self.atualizar_tabuleiro()
                        if self.jogo_rust.jogo_terminou():
                            messagebox.showinfo("Fim de Jogo", "O jogo terminou!")
                  else:
                      messagebox.showinfo("Erro", "Jogada inválida")

                  self.selecao = [] 
              
              break 
      
      if not casa_encontrada:
          if self.selecao:
              self.selecao = [] 
              self.atualizar_tabuleiro() 
              
      self.atualizar_tabuleiro() 


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
        self.menu_frame.pack(expand=True, fill="both")

    def show_board_frame(self, escolha_do_jogador):
        self.menu_frame.pack_forget()
        self.tabuleiro_frame = TabuleiroFrame(self, escolha_do_jogador)
        self.tabuleiro_frame.pack(expand=True, fill="both")

if __name__ == "__main__":
    app = App()
    app.mainloop()
