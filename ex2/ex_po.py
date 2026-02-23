import random
import customtkinter as ctk
from PIL import Image
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
    def __init__(self, master, color="grey23", hover_color="grey21", **kwargs):
        super().__init__(master, fg_color=color, dropdown_fg_color=color, button_color=color, button_hover_color=hover_color, **kwargs)

class BikeApp(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.geometry("1200x800")
        self.main_color = "#181B22"
        
        self.configure(fg_color=self.main_color)
        self.font = ("Roboto",25)
        self.bike_path = "bikes.csv"
        self.create_layout()
        self.create_form()
        self.create_navbar()
        self.create_action_bar()
        self.create_bike_list()

        self.mainloop()

    def create_layout(self):
        self.navbar = ctk.CTkFrame(self, fg_color=self.main_color)
        self.navbar.pack(fill=ctk.X, ipady=10)
        self.action_bar = ctk.CTkFrame(self, corner_radius=0)
        self.action_bar.pack(fill=ctk.X, ipady=10)
        self.bike_list = ctk.CTkScrollableFrame(self)
        self.bike_list.pack(fill=ctk.BOTH, expand=True, padx=10, pady=10)
        self.input_form = ctk.CTkFrame(self)
        self.input_form.pack(fill=ctk.X, padx=10, pady=10)
    
    def create_navbar(self):
        master = self.navbar
        self.app_logo = ctk.CTkLabel(master, font=self.font)
        self.app_logo.pack(side=ctk.LEFT, padx=15)
        self.search_input = ctk.CTkEntry(master, placeholder_text="Search", font=self.font)
        self.search_input.pack(side=ctk.RIGHT, padx=10)
    
    def create_action_bar(self):
        master = self.action_bar
        self.buttons = []
        for (text, color, file_path) in [("Load Data", "Blue", "file.png"), ("Save Data", "Blue", "save.png"), ("Delete Selected", "Red", "trash.png")]:
            if file_path != "":
                image = Image.open(f"icons/{file_path}")
                ctk_image = ctk.CTkImage(light_image=image)
                btn = ctk.CTkButton(master, text=text, font=self.font, fg_color=color, image=ctk_image)
                btn.pack(side=ctk.LEFT, padx=10)
            else:
                btn = ctk.CTkButton(master, text=text, font=self.font, fg_color=color)
                btn.pack(side=ctk.LEFT, padx=10)
        
    def create_bike_list(self):
        master = self.bike_list
        self.bike_table = CTkTable(master, column=7, values=SAMPLE_DATA)
        self.bike_table.pack(fill=ctk.X)

    def create_form(self):
        master = self.input_form
        master.columnconfigure((0, 1, 2, 3, 4), weight=1)
        master.rowconfigure((0, 1, 2, 3), weight=1)
        self.input_form_label = ctk.CTkLabel(master, text="ADD NEW INVENTORY")
        self.input_form_label.grid(row=0, column=0, columnspan=2, sticky="W", padx=(10, 5), pady=5)
        self.brand_input = ctk.CTkEntry(master, placeholder_text="Brand")
        self.brand_input.grid(row=1, column=0, sticky="NSEW", padx=(10, 5), pady=5)
        self.model_input = ctk.CTkEntry(master, placeholder_text="Model")
        self.model_input.grid(row=1, column=1, sticky="NSEW", pady=5, padx=5)
        self.type_input = CustomDropDown(master, values=BIKE_TYPE)
        self.type_input.grid(row=1, column=2, sticky="NSEW", pady=5, padx=5)
        self.size_input = CustomDropDown(master, values=SIZE)
        self.size_input.grid(row=1, column=3, sticky="NSEW", pady=5, padx=5)
        self.description = ctk.CTkTextbox(master, height=80)
        self.description.grid(row=1, column=4, rowspan=2, columnspan=3, sticky="NSEW", pady=5, padx=5)
        self.wheel_size_input = CustomDropDown(master, values=WHEEL_SIZES)
        self.wheel_size_input.grid(row=2, column=0, sticky="NSEW", padx=(10, 5), pady=5)
        self.stock_quntity_input = ctk.CTkEntry(master, placeholder_text="Stock Quantity")
        self.stock_quntity_input.grid(row=2, column=1, sticky="NSEW", pady=5, padx=5)
        self.price_input = ctk.CTkEntry(master, placeholder_text="Price")
        self.price_input.grid(row=2, column=2, columnspan=2, sticky="NSEW", pady=5, padx=5)
        self.clear_form_btn = ctk.CTkButton(master, text="Clear Form", fg_color="Gray")
        self.clear_form_btn.grid(row=3, column=5, sticky="NSEW", pady=5, padx=5)
        image = Image.open("icons/add_icon.jpg")
        ctk_image = ctk.CTkImage(light_image=image)
        self.clear_form_btn = ctk.CTkButton(master, text="Add to Inventory", fg_color="Blue", image=ctk_image)
        self.clear_form_btn.grid(row=3, column=6, sticky="NSEW", pady=5, padx=5)


    
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