import tkinter as tk
import random
import customtkinter as ctk

def pack_sides():
    root = ctk.CTk()
    root.geometry('1250x550')
    button1 = ctk.CTkButton(root, text="Left")
    button1.pack(side = tk.LEFT)
    button2 = ctk.CTkButton(root, text="Top")
    button2.pack(side = tk.TOP)
    button3 = ctk.CTkButton(root, text="Right")
    button3.pack(side = tk.RIGHT)
    button4 = ctk.CTkButton(root, text="Bottom")
    button4.pack(side = tk.BOTTOM)
    root.mainloop()

def pack_sides_with_anchor():
    root = tk.Tk()
    root.geometry('1250x550')
    button1 = tk.Button(text="Left")
    button1.pack(side = tk.LEFT, anchor="n")
    button2 = tk.Button(text="Top")
    button2.pack(side = tk.TOP, anchor="w")
    button3 = tk.Button(text="Right")
    button3.pack(side = tk.RIGHT)
    button4 = tk.Button(text="Bottom")
    button4.pack(side = tk.BOTTOM)
    root.mainloop()

def main():
    root = tk.Tk()
    root.geometry('1200x900')
    bg = "white"
    root.configure(bg=bg)

    name_label = tk.Label(root, text="Enter Name", bg=bg)
    name_label.pack()
    name_entry = tk.Entry(root)
    name_entry.pack()

    email_label = tk.Label(root, text="Enter email", bg=bg)
    email_label.pack()
    email_entry = tk.Entry(root)
    email_entry.pack()

    phone_label = tk.Label(root, text="Enter phone", bg=bg)
    phone_label.pack()
    phone_entry = tk.Entry(root)
    phone_entry.pack()

    submit_btn = tk.Button(root, text="Submit", bg="blue", fg="white")
    submit_btn.pack()

    root.mainloop()

def example_1():

    def f():
        sum = 0
        for var in vars:
            num = int(var.get())
            sum += num
        print(sum)
        result_var.set(str(sum))

    root = tk.Tk()
    root.geometry("800x600")

    left_frame = tk.Frame(root, highlightbackground="black", highlightthickness=1)
    left_frame.pack(side="left", pady=50, padx=20, fill="both")

    variables = ["x", "y"]
    vars = []
    for var in variables:
        frame = tk.Frame(left_frame, pady=10)
        label = tk.Label(frame, text=f"Label {var}")
        label.pack(side="left", anchor="n", padx=10)
        input = tk.Entry(frame)
        input.pack(side="left", anchor="n", padx=10)
        vars.append(input)
        frame.pack()

    right_frame = tk.Frame(root, highlightbackground="black", highlightthickness=1)
    right_frame.pack(side="top", pady=50, padx=20, anchor="n")

    result_var = tk.StringVar(value="0")
    result = tk.Label(right_frame, padx=100, pady=100, textvariable=result_var)
    result.pack(side="top", anchor="n")


    calc_button = tk.Button(root, text="Calc", padx=20, command=f)
    calc_button.pack(side="left", padx=20)
    quit_button = tk.Button(root, text="Quit", padx=20)
    quit_button.pack(side="left", padx=20)


    root.mainloop()
    

if __name__ == "__main__":
    # pack_sides()
    # pack_sides_with_anchor()
    example_1()