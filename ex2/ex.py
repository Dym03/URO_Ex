import random
import customtkinter as ctk
from enum import Enum

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
    
class BikeApp(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.geometry("1200x800")
        self.font = ("Roboto",30)
        self.bike_path = "bikes.csv"
        self.create_form()
        self.mainloop()

    def create_form(self):
        self.brand_label = ctk.CTkLabel(self, pady = 10, text="Brand Name", width=200, font=self.font)
        self.brand_label.pack(ipady = 10)
        self.brand_input = ctk.CTkEntry(self, placeholder_text="Brand Name", width=200, font=self.font)
        self.brand_input.pack(ipady = 10)
        self.model_label = ctk.CTkLabel(self, text="Model Name", font=self.font, pady = 10)
        self.model_label.pack(ipady = 10)
        self.model_input = ctk.CTkEntry(self, placeholder_text="Model Name", width=200, font=self.font)
        self.model_input.pack(ipady = 10)
        self.size_label = ctk.CTkLabel(self, text="Brand Name", font=self.font, pady = 10)
        self.size_label.pack(ipady = 10)
        self.size_input = ctk.CTkOptionMenu(self, values=SIZE, font=self.font, dropdown_font=self.font)
        self.size_input.pack()
        self.add_button = ctk.CTkButton(self, text="Add Bike", font=self.font, command=self.add_bike)
        self.add_button.pack(ipady = 15, pady = 20)
    
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