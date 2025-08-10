import tkinter as tk
from tkinter import messagebox
from PIL import Image, ImageTk
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

        self.jogo_rust = pintada.Jogo()  # Instância do jogo no Rust
        self.escolha_do_jogador = escolha_do_jogador

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
        # Aqui você teria que definir as conexões manualmente
        # ou pegar de um modelo como "board_state.py"
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
      for pos, (cx, cy) in self.coordenadas_casas.items():
          if (x - cx)**2 + (y - cy)**2 <= 15**2:
              self.selecao.append(pos)
              self.atualizar_tabuleiro()
              if len(self.selecao) == 2:
                  from_pos, to_pos = self.selecao
                  try:
                      sucesso = self.jogo_rust.aplicar_jogada(from_pos, to_pos)
                  except Exception as e:
                      messagebox.showerror("Erro", f"Falha ao aplicar jogada: {e}")
                      sucesso = False

                  if sucesso:
                      self.atualizar_tabuleiro()

                      # Turno da CPU
                      self.jogo_rust.jogada_cpu()
                      self.atualizar_tabuleiro()
                  else:
                      messagebox.showinfo("Erro", "Jogada inválida")

                  self.selecao = []
              break


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
