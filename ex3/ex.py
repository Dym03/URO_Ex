import random
import customtkinter as ctk
from CTkTable import *


BRAND_MODEL = {"Giant" : ["Trance", "Propel", "Roadster"],
               "Author" : ["Pavel", "Kolo"],
               "Bianchi" : ["Stradale", "Puppisimo"]}

SIZE = ["S", "M", "L", "XL"]
WHEEL_SIZES = ["26", "27.5", "29"]
BIKE_TYPE = ["City", "Road", "Gravel", "MTB", "Fixed"]

SAMPLE_DATA = [
            ["Brand", "Name", "Type", "Size", "Wheel Size", "Stock", "Price"],
            ["Trek", "Marlin 7", "Gravel", "M", "29", "12", "$850.00"],
            ["Specialized", "Rockhopper", "Mountain", "L", "29", "5", "$700.00"],
        ]

ctk.set_window_scaling(1.3)  # Scales the window size
ctk.set_widget_scaling(1.3)  # Scales the widgets and fonts
ctk.set_appearance_mode("dark")

class Bike:
    id = 0
    def __init__(self, brand, model, size, _type="", wheel_size="", quantity=0, price=0, description=""):
        self.bike_id = Bike.id
        Bike.id += 1
        self.brand = brand
        self.model = model
        self.size = size
        self.type = _type
        self.wheel_size = wheel_size
        self.quantity = quantity
        self.price = price
        self.description = description

    def new():
        brand = random.choice(list(BRAND_MODEL.keys()))
        model = random.choice(BRAND_MODEL[brand])
        size = random.choice(SIZE)
        return Bike(brand, model, size)

    def __str__(self):
        return f"{self.brand} - {self.model} - {self.size}"

    def __repr__(self):
        return str(self)

    def to_csv(self):
        return f"{self.bike_id}, {self.brand}, {self.model}, {self.size}"

class CustomDropDown(ctk.CTkOptionMenu):
    def __init__(self, master, fg_color="gray23", hover_color="gray21", **kwargs):
        super().__init__(master, fg_color=fg_color, dropdown_fg_color=fg_color, button_color=fg_color, button_hover_color=hover_color, **kwargs)

class BikeApp(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.geometry("1200x800")
        self.main_color = "#181B22"
        self.configure(fg_color=self.main_color)
        self.font = ("Noto",15)
        self.bike_path = "bikes.csv"
        self.create_layout()
        self.create_navbar()
        self.create_action_bar()
        self.create_bike_list()
        self.create_form()
        self.mainloop()

    def create_layout(self):
        self.navbar = ctk.CTkFrame(self, fg_color=self.main_color)
        self.navbar.pack(fill=ctk.X, ipady=10)
        self.action_bar = ctk.CTkFrame(self, corner_radius=0)
        self.action_bar.pack(fill=ctk.X, ipady=10)
        self.bike_list = ctk.CTkScrollableFrame(self)
        self.bike_list.pack(fill=ctk.BOTH, expand=True, padx=20, pady=20)
        self.input_form = ctk.CTkFrame(self)
        self.input_form.pack(fill=ctk.X, padx=20, pady=20)

    def create_navbar(self):
        master = self.navbar
        self.logo = ctk.CTkLabel(master, text="🚲 Bike Manager Pro", font=self.font)
        self.logo.pack(side=ctk.LEFT, padx=10)
        self.search_bar = ctk.CTkEntry(master, placeholder_text="Search")
        self.search_bar.pack(side=ctk.RIGHT, padx=10)

    def create_action_bar(self):
        master = self.action_bar
        self.buttons = []
        for (text, color) in [("📁 Save Data", "Blue"), ("💾 Load Data", "Blue"), ("🗑 Delete Selected", "Red")]:
            btn = ctk.CTkButton(master, text=text, fg_color=color, font=self.font)
            btn.pack(side=ctk.LEFT, padx=5)
            self.buttons.append(btn)

    def create_bike_list(self):
        master = self.bike_list
        self.table = CTkTable(master=master, column=7, values=SAMPLE_DATA)
        self.table.pack(expand=True, fill="both")

    def create_form(self):
        master = self.input_form
        padx = 5
        pady = 5
        master.rowconfigure((0, 1, 2, 3), weight=1)
        master.columnconfigure((0, 1, 2, 3, 4, 5, 6), weight=1)
        self.input_form_label = ctk.CTkLabel(master, text="ADD NEW INVENTORY")
        self.input_form_label.grid(row=0, column=0, columnspan=2, sticky="W", padx=(10, padx), pady=pady)
        self.brand_entry = ctk.CTkEntry(master, placeholder_text="Brand")
        self.brand_entry.grid(row=1, column=0, sticky="NSEW", padx=(10, padx), pady=pady)
        self.model_entry = ctk.CTkEntry(master, placeholder_text="Model")
        self.model_entry.grid(row=1, column=1, sticky="NSEW", padx=padx, pady=pady)
        self.type_entry = CustomDropDown(master, values=BIKE_TYPE)
        self.type_entry.grid(row=1, column=2, sticky="NSEW", padx=padx, pady=pady)
        self.size_entry = CustomDropDown(master, values=SIZE)
        self.size_entry.grid(row=1, column=3, sticky="NSEW",padx=padx, pady=pady)
        self.description = ctk.CTkTextbox(master, height=80)
        self.description.grid(row=1, column=4, sticky="NSEW", rowspan=2, columnspan=3, padx=padx, pady=pady)
        self.wheel_size_entry = CustomDropDown(master, values=WHEEL_SIZES)
        self.wheel_size_entry.grid(row=2, column=0, sticky="NSEW", padx=(10, padx), pady=pady)
        self.stock_quantiy_entry = ctk.CTkEntry(master, placeholder_text="Quantity")
        self.stock_quantiy_entry.grid(row=2, column=1, sticky="NSEW", padx=padx, pady=pady)
        self.price_entry = ctk.CTkEntry(master, placeholder_text="Price")
        self.price_entry.grid(row=2, column=2, columnspan=2, sticky="NSEW", padx=padx, pady=pady)
        self.clear_form_btn = ctk.CTkButton(master, text="Clear Form")
        self.clear_form_btn.grid(row=3, column=5, sticky="NSEW",  padx=padx, pady=pady)
        self.add_btn = ctk.CTkButton(master, text="Add to Inventory")
        self.add_btn.grid(row=3, column=6, sticky="NSEW", padx=padx, pady=pady)

    def add_bike(self):
        brand = self.brand_input.get()
        model = self.model_input.get()
        size = self.size_input.get()
        if brand and model and size:
            with open(self.bike_path, "a+") as file:
                bike = Bike(brand, model, size)
                line = bike.to_csv() + "\n"
                file.write(line)
        else:
            print("Fill all the inputs")

if __name__ == "__main__":
    app = BikeApp()