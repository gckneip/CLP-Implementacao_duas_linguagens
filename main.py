import tkinter as tk
from tkinter import messagebox
import pintada

def on_button_click():
    """Esta função é chamada quando o botão é clicado."""

    button_onca.config(state=tk.DISABLED)
    
    status_label.config(text="Aguarde, a tarefa em Rust está em execução...")

    try:

        result = pintada.do_long_task()
        messagebox.showinfo("Sucesso", f"O Rust retornou: {result}")
    except Exception as e:
        messagebox.showerror("Erro", f"Ocorreu um erro: {e}")
    finally:
        
        button_onca.config(state=tk.NORMAL)
        status_label.config(text="Pronto para uma nova tarefa.")


root = tk.Tk()
root.title("Integração Python + Rust")

pre_title_label = tk.Label(
    root,
    text="Bem vindo ao",
    font=("Arial", 16),
    pady=20,
    )
pre_title_label.grid(row=0, column=0)

title_label = tk.Label(
    root,
    text="JOGO DA ONÇA",
    font=("Arial", 32, "bold"),
    fg="#D2691E",
    #bg="#FFF8DC",
    pady=20,
    padx=20
)

title_label.grid(row=0, column=1, sticky="ew")

status_label = tk.Label(root, text="Deseja jogar como...")

status_label.grid(row=0, pady=20, padx=20, column=20)


button_onca = tk.Button(root, text="Executar Tarefa em Rust", command=on_button_click)

button_onca.grid(row=1, padx=20, pady=5, column=0, columnspan=89, sticky="ew")

root.mainloop()