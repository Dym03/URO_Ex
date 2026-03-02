import random
import os
import tkinter as tk
from tkinter import ttk
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
        ]

def get_tree_data(bike_list):
    tree_data = {}
    for brand, model, bike_type in bike_list:
        if brand not in tree_data:
            tree_data[brand] = {}
        if bike_type not in tree_data[brand]:
            tree_data[brand][bike_type] = []
            
        tree_data[brand][bike_type].append(model)
    return tree_data

ctk.set_window_scaling(1.5)  # Scales the window size
ctk.set_widget_scaling(1.5)  # Scales the widgets and fonts
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

class TreeFrame(ctk.CTkFrame):
    def __init__(self, master, **kwargs):
        super().__init__(master, **kwargs)
        self.sorted_columns = {}

        self.style = ttk.Style(self)
        self.style.theme_use("clam")

        bg_color = "#2b2b2b"       
        heading_bg = "#333333"     
        text_color = "#dce4ee"    
        selected_color = "#1f538d" 

        self.style.configure(
            "Treeview",
            background=bg_color,
            foreground=text_color,
            fieldbackground=bg_color,
            borderwidth=0,
            rowheight=40,
            font=("Noto Sans", 13) 
        )

        self.style.map(
            'Treeview',
            background=[('selected', selected_color)],
            foreground=[('selected', "white")]
        )

        self.style.configure(
            "Treeview.Heading",
            background=heading_bg,
            foreground=text_color,
            borderwidth=0,
            font=("Noto Sans", 14, "bold")
        )

        self.style.map(
            "Treeview.Heading",
            background=[('active', selected_color)] 
        )

        self.columns = ["Brand", "Name", "Type", "Size", "Wheel Size", "Price"]
        self.tree = ttk.Treeview(
            self,
            style="Treeview",
            columns=self.columns,
            show="headings",
        )

        for col in self.columns:
            self.tree.heading(col, text=col, anchor="center")

        self.tree.column("Brand", minwidth=150, anchor="center", stretch=True)
        self.tree.column("Name", minwidth=200, anchor="center", stretch=True)
        self.tree.column("Type", minwidth=100, anchor="center", stretch=True)
        self.tree.column("Size", minwidth=50, anchor="center", stretch=True)
        self.tree.column("Wheel Size", minwidth=100, anchor="center", stretch=True)
        self.tree.column("Price", minwidth=100, anchor="center", stretch=True)
 
        scrollbar = ctk.CTkScrollbar(self, orientation="vertical", command=self.tree.yview)
        self.tree.configure(yscrollcommand=scrollbar.set)
        
        self.tree.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

        for col in self.columns:
            self.tree.heading(col, text=col, command=lambda c=col: self.sort_column(c))
            self.sorted_columns[col] = False

        self.tree.bind("<Double-Button-1>", self.item_selected)

    def sort_column(self, column):
        items = [(self.tree.set(item, column), item) for item in self.tree.get_children("")]
        items.sort()
        if self.sorted_columns[column]:
            items.reverse()
            self.sorted_columns[column] = False
        else:
            self.sorted_columns[column] = True
            
        for index, (val, item) in enumerate(items):
            self.tree.move(item, "", index)

        self.update_column_header_appearance(column)

    def update_column_header_appearance(self, column):
        """Update the appearance of the column headers to indicate the sorted column."""
        for col in self.tree["columns"]:
            if col == column:
                self.tree.heading(
                    col, text=col + ("  ▲" if not self.sorted_columns[col] else "  ▼")
                )
            else:
                self.tree.heading(col, text=col)
                self.sorted_columns[col] = False

    def item_selected(self, event):
        # Changed tk.Yes_No to standard variable naming to avoid overwriting tkinter module attributes
        is_yes = tk.messagebox.askyesno(
            "Delete", "Do you want to delete the selected row?"
        )
        if is_yes:
            for item in event.widget.selection():
                event.widget.delete(item)

class InfoWindow(ctk.CTkToplevel):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.geometry("400x300")

        self.label = ctk.CTkLabel(self, text="Feedback")
        self.label.pack(padx=20, pady=20)
        self.info_text = ctk.CTkTextbox(self)
        self.info_text.pack()
        self.send_button = ctk.CTkButton(self, text="Submit Feedback", command=self.submit_feedback)
        self.send_button.pack()

    def submit_feedback(self):
        self.destroy()

class BikeApp(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.geometry("1200x800")
        self.bikes = []
        self.unique_bikes = set()
        self.main_color = "#181B22"
        self.configure(fg_color=self.main_color)
        self.font = ("Noto Serif",25)
        self.bike_path = "bikes.csv"
        self.create_layout()
        self.create_home_scene()
        self.create_store_scene()
        self.create_navbar()
        self.create_action_bar()
        self.create_bike_list()
        self.create_form()
        self.mainloop()

    def create_layout(self):
        self.navbar = ctk.CTkFrame(self, fg_color=self.main_color)
        self.navbar.pack(fill=ctk.X, ipady=10)

        self.main_container = ctk.CTkFrame(self, fg_color="transparent")
        self.main_container.pack(fill=ctk.BOTH, expand=True)

        self.home_scene = ctk.CTkFrame(self.main_container, fg_color="transparent")
        self.home_scene.pack(fill=ctk.BOTH, expand=True)
        self.store_scene = ctk.CTkFrame(self.main_container, fg_color="transparent")
        self.toplevel_window = None

    def create_home_scene(self):
        self.action_bar = ctk.CTkFrame(self.home_scene, corner_radius=0)
        self.action_bar.pack(fill=ctk.X, ipady=10)
        self.bike_list = ctk.CTkScrollableFrame(self.home_scene)
        self.bike_list.pack(fill=ctk.BOTH, expand=True, padx=20, pady=20)
        self.input_form = ctk.CTkFrame(self.home_scene)
        self.input_form.pack(fill=ctk.X, padx=20, pady=20)

    def create_store_scene(self):
        self.store_list_columns = ["Brand", "Model", "Type", "Size", "Wheel Size", "Price"]

        bg_color = self._apply_appearance_mode(ctk.ThemeManager.theme["CTkFrame"]["fg_color"])
        text_color = self._apply_appearance_mode(ctk.ThemeManager.theme["CTkLabel"]["text_color"])
        selected_color = self._apply_appearance_mode(ctk.ThemeManager.theme["CTkButton"]["fg_color"])

        treestyle = ttk.Style()
        treestyle.theme_use('default')
        treestyle.configure("Treeview", background=bg_color, foreground=text_color, fieldbackground=bg_color, borderwidth=0)
        treestyle.map('Treeview', background=[('selected', bg_color)], foreground=[('selected', selected_color)])
        self.store_scene.bind("<<TreeviewSelect>>", lambda event: self.store_scene.focus_set())
    
        ##Treeview widget data
        self.treeview = ttk.Treeview(self.store_scene, height=6, show="tree")
        self.treeview.pack(fill=ctk.BOTH, expand=True, padx=20, pady=20)
        # self.fill_tree()        

        self.bike_list_shop = TreeFrame(self.store_scene)
        self.bike_list_shop.pack(fill=ctk.BOTH, expand=True, padx=15, pady=15)

    def fill_tree(self, data):
        grouped_data = get_tree_data(data)

        for brand, types in grouped_data.items(): 
            brand_node = self.treeview.insert("", "end", text=brand)
            
            for bike_type, models in types.items():
                type_node = self.treeview.insert(brand_node, "end", text=bike_type)
                
                for model in models:                    
                    self.treeview.insert(type_node, "end", text=model)

    def create_navbar(self):
        master = self.navbar
        self.logo = ctk.CTkLabel(master, text="🚲 Bike Manager Pro", font=self.font)
        self.logo.pack(side=ctk.LEFT, padx=10, fill=ctk.X)        
        self.nav_navigation = ctk.CTkSegmentedButton(
            master, 
            values=["Home", "Store", "Info"], 
            command=self.change_scene
        )
        self.nav_navigation.pack(side=ctk.LEFT, padx=20, ipady=10, ipadx=10)
        self.nav_navigation.set("Home")
        
        self.search_bar = ctk.CTkEntry(master, placeholder_text="Search")
        self.search_bar.pack(side=ctk.RIGHT, padx=10)

    def change_scene(self, scene_name):
        self.home_scene.pack_forget()
        self.store_scene.pack_forget()

        if scene_name == "Home":
            self.home_scene.pack(fill=ctk.BOTH, expand=True)
        elif scene_name == "Store":
            self.store_scene.pack(fill=ctk.BOTH, expand=True)
        elif scene_name == "Info":
            if self.toplevel_window is None or not self.toplevel_window.winfo_exists():
                self.toplevel_window = InfoWindow(self)
            else:
                self.toplevel_window.focus() 

    def create_action_bar(self):
        master = self.action_bar
        self.buttons = []
        for (text, color, command) in [("📁 Save Data", "Blue", None), ("💾 Load Data", "Blue", self.load_bikes), ("🗑 Delete Selected", "Red", None)]:
            btn = ctk.CTkButton(master, text=text, fg_color=color, font=self.font, command=command)
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
    
    def load_bikes(self):
        filename = ctk.filedialog.askopenfilename()
        if os.path.isfile(filename):
            with open(filename, "r") as file:
                for (idx, line) in enumerate(file.readlines()[1:]):
                    tokens = line.split(",")
                    brand = tokens[0]
                    model = tokens[1]
                    type = tokens[2]
                    size = tokens[3]
                    wheel_size = tokens[4]
                    stock = int(tokens[5])  
                    price = int(tokens[6][1:])
                    print(brand, model, type, size, wheel_size, stock, price)   
                    self.bikes.append(Bike(brand, model, size, type, wheel_size, stock, price))
                    self.table.add_row(values=[brand, model, type, size, wheel_size, stock, price])
                    self.unique_bikes.add((brand, model, type))
                    self.bike_list_shop.tree.insert("", ctk.END, values=[brand, model, type, size, wheel_size, price])

        self.fill_tree(list(self.unique_bikes))
        return 
    

if __name__ == "__main__":
    app = BikeApp()