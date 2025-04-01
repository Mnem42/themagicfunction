import json
import random
import sys
import time
import tkinter as tk

class CodeRunner:
    def __init__(self,strings: list[str]):
        self.strings=strings

    def _select_random(self):
        return self.strings[random.randint(0,len(self.strings)-1)]

    def run_code_cli(self):
        print("")
        for _ in range(5):
            print("Processing.  \033[1A\033[K")
            time.sleep(0.5)
            print("Processing.. \033[1A\033[K")
            time.sleep(0.5)
            print("Processing...\033[1A\033[K")
            time.sleep(0.5)

        print("\033[K", self._select_random(), "\n")

    def run_code_tk(self):
        root = tk.Tk()

        root.title("Magic function")
        root.attributes('-fullscreen', True)

        text = tk.Label(root, text=self._select_random(), font=("Comic Sans MS", 50, 'bold'), anchor='center')
        text.pack(expand=True)

        root.mainloop()

if __name__ == "__main__":
    strings = json.load(open("strings.json"))["strings"]

    while True:
        print("1. Run CLI version\n"
              "2. Run GUI version\n"
              "3. Show license\n"
              "4. Exit")
        option = input("Run option:")
        
        match option:
            case "1":
                CodeRunner(strings).run_code_cli()
            case "2":
                CodeRunner(strings).run_code_tk()
                print("")
            case "3":
                print(open("LICENSE.txt").read(),'\n')
            case "4":
                exit(0)
            case _:
                pass