#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    this->resize(1400, 800);
    const QFont font = QFont("Roboto", 15);
    this->setFont(font);
    create_layout();
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::create_layout() {
    QWidget * central_widget = new QWidget();
    this->top_level_layout = new QVBoxLayout();
    this->top_level_layout->setContentsMargins(20, 20, 20, 20);
    this->top_level_layout->setSpacing(10);
    central_widget->setLayout(this->top_level_layout);
    this->setCentralWidget(central_widget);
    this->create_navbar();
    this->create_action_bar();
    this->create_inventory();
    this->create_table();
    this->create_form();

    apply_dark_theme(this);
}

void MainWindow::create_navbar() {
    QHBoxLayout * navbar_layout = new QHBoxLayout();
    navbar_layout->setAlignment(Qt::AlignTop);
    QLabel * app_name = new QLabel("Bike App");
    QPushButton* home = new QPushButton("Home");
    QPushButton* store = new QPushButton("Store");
    QPushButton* info = new QPushButton("Info");
    home->setObjectName("navBtn");
    store->setObjectName("navBtn");
    info->setObjectName("navBtn");

    connect(info, SIGNAL(clicked()), this, SLOT(on_info_clicked()));

    QLineEdit* search_bar = new QLineEdit();
    search_bar->setPlaceholderText("Search");
    navbar_layout->addWidget(app_name);
    navbar_layout->addWidget(home);
    navbar_layout->addWidget(store);
    navbar_layout->addWidget(info);
    navbar_layout->addStretch();
    navbar_layout->addWidget(search_bar);
    this->top_level_layout->addLayout(navbar_layout);
}

void MainWindow::on_info_clicked() {
    this->info_window = new QDialog(this);
    info_window->resize(400, 400);
    QVBoxLayout* info_layout = new QVBoxLayout();
    info_layout->setAlignment(Qt::AlignTop | Qt::AlignCenter);
    info_layout->setContentsMargins(10, 10, 10, 10);
    info_window->setWindowTitle("Info");
    info_window->setObjectName("info_window");


    QLabel* info_header = new QLabel("Info for bike app");
    info_header->setAlignment(Qt::AlignCenter);
    info_layout->addWidget(info_header);

    QTextEdit* feedback = new QTextEdit();
    feedback->setPlaceholderText("Feedback");
    info_layout->addWidget(feedback);

    QPushButton* exit_btn = new QPushButton("Send");
    info_layout->addWidget(exit_btn);

    connect(exit_btn, SIGNAL(clicked()), this, SLOT(on_send_clicked()));

    info_window->setLayout(info_layout);
    info_window->exec();
}

void MainWindow::on_send_clicked() {
    info_window->done(0);
}

void MainWindow::create_action_bar() {
    QHBoxLayout * action_bar_layout = new QHBoxLayout();
    action_bar_layout->setAlignment(Qt::AlignTop);
    QPushButton* load_btn = new QPushButton("Load Data");
    load_btn->setObjectName("primaryBtn");

    QPushButton* save_btn = new QPushButton("Save Data");
    save_btn->setObjectName("primaryBtn");

    QPushButton* delete_btn = new QPushButton("Delete Selected");
    delete_btn->setObjectName("dangerBtn");

    action_bar_layout->addWidget(save_btn);
    action_bar_layout->addWidget(load_btn);
    action_bar_layout->addWidget(delete_btn);
    action_bar_layout->addStretch();
    this->top_level_layout->addLayout(action_bar_layout);
}

void MainWindow::create_inventory() {
    this->inventory = parse_bikes_csv("bikes.csv");
}

void MainWindow::create_table() {
    QFrame* table_container = new QFrame();
    table_container->setObjectName("formCard");

    QVBoxLayout* table_layout = new QVBoxLayout(table_container);

    std::vector<QString> headers = {"Brand", "Name", "Type", "Size", "Wheel Size", "Stock", "Price"};

    this->model = new QStandardItemModel(0, headers.size());

    for (size_t i = 0; i < headers.size(); i++) {
        this->model->setHeaderData(i, Qt::Horizontal, headers[i]);
    }

    QTableView* table = new QTableView();
    table->setModel(this->model);
    table->horizontalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    table->horizontalHeader()->setDefaultAlignment(Qt::AlignLeft | Qt::AlignVCenter);

    table->verticalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    table->verticalHeader()->setVisible(false);
    table->setEditTriggers(QAbstractItemView::NoEditTriggers);
    table->setSelectionBehavior(QAbstractItemView::SelectRows);
    table->setSelectionMode(QAbstractItemView::SingleSelection);

    connect(table, SIGNAL(clicked(QModelIndex)), this, SLOT(on_item_selected(QModelIndex)));

    table_layout->addWidget(table);

    for (auto bike: this->inventory) {
        model->appendRow(bike.to_row());
    }

    this->top_level_layout->addWidget(table_container, 1);
}

void MainWindow::on_item_selected(QModelIndex idx) {
    if (idx.isValid()) {
        int row = idx.row();
        for (int col = 0; col < model->columnCount(); ++col) {
            QStandardItem *item = model->item(row, col);
            if (item) {
                QString text = item->text();
                qDebug() << "Column" << col << ":" << text;
            }
        }
    }
}

void MainWindow::create_form() {
    QFrame* form_container = new QFrame();
    form_container->setObjectName("formCard");

    QGridLayout* form_layout = new QGridLayout(form_container);
    form_layout->setContentsMargins(20, 20, 20, 20);
    form_layout->setSpacing(15);

    QLabel* form_name = new QLabel("Add to inventory");
    form_name->setObjectName("formTitle");
    form_layout->addWidget(form_name, 0, 0);

    QLineEdit* brand_input = new QLineEdit();
    brand_input->setPlaceholderText("Brand");
    QLineEdit* model_input = new QLineEdit();
    model_input->setPlaceholderText("Model");
    form_layout->addWidget(brand_input, 1, 0);
    form_layout->addWidget(model_input, 1, 1);

    QComboBox* type_selection = new QComboBox();
    type_selection->addItems({"MTB", "Road", "Gravel"});
    form_layout->addWidget(type_selection, 1, 2);

    QComboBox* size_selection = new QComboBox();
    size_selection->addItems({"S", "M", "L"});
    form_layout->addWidget(size_selection, 1, 3);

    QTextEdit* description = new QTextEdit();
    description->setPlaceholderText("Description");
    form_layout->addWidget(description, 1, 4, 2, 4);

    QComboBox* wheel_size_selection = new QComboBox();
    wheel_size_selection->addItems({"26", "27.5", "29"});
    form_layout->addWidget(wheel_size_selection, 2, 0);

    QLineEdit* stock_input = new QLineEdit();
    stock_input->setPlaceholderText("Stock Quantity");
    QLineEdit* price_input = new QLineEdit();
    price_input->setPlaceholderText("Price");
    form_layout->addWidget(stock_input, 2, 1);
    form_layout->addWidget(price_input, 2, 2, 1, 2);

    QPushButton* clear_btn = new QPushButton("Clear form");
    clear_btn->setObjectName("secondaryBtn");
    form_layout->addWidget(clear_btn, 3, 5);


    QPushButton* add_btn = new QPushButton("Add to inventory");
    add_btn->setObjectName("primaryBtn");
    form_layout->addWidget(add_btn, 3, 6, 1, 2);

    this->top_level_layout->addWidget(form_container);
}

void apply_dark_theme(QWidget *window) {
    QString qss = R"(
        /* Global Background and Text */
        QMainWindow, QWidget, QDialog {
            background-color: #1e222d;
            color: #d1d5db;
            font-family: 'Segoe UI', Roboto, sans-serif;
        } 

        /* Table Styling */
        QTableView {
            background-color: #282c37;
            border: 1px solid #3f4451;
            gridline-color: #3f4451;
            selection-background-color: #3d4455;
            border-radius: 8px;
        }
        QHeaderView::section {
            background-color: #282c37;
            color: #9ca3af;
            padding: 10px;
            border: none;
            text-transform: uppercase;
            font-weight: bold;
            font-size: 14px;
        }

        /* Inputs & ComboBoxes */
        QLineEdit, QComboBox, QTextEdit {
            background-color: #282c37;
            border: 1px solid #3f4451;
            border-radius: 6px;
            padding: 8px;
            color: white;
        }

        /* Buttons - General */
        QPushButton {
            background-color: #3f4451;
            border-radius: 6px;
            padding: 8px 15px;
            font-weight: bold;
        }
        QPushButton:hover { background-color: #4b5263; }

        /* Custom Named Buttons */
        QPushButton#primaryBtn { background-color: #2563eb; color: white; }
        QPushButton#primaryBtn:hover { background-color: #3b82f6; }

        QPushButton#dangerBtn { background-color: #991b1b; color: white; opacity: 0.8; }
        QPushButton#dangerBtn:hover { background-color: #b91c1c; }

        QPushButton#secondaryBtn { background-color: #374151; color: white; }

        QFrame#formCard {
            background-color: #282c37;
            border: 1px solid #3f4451;
            border-radius: 12px;
            margin-top: 10px;
        }

        /* Make the Table headers look cleaner */
        QHeaderView::section {
            background-color: #1e222d; /* Match the main background */
            color: #e5e7eb; /* Dimmer text for headers */
            border-bottom: 1px solid #3f4451;
            text-transform: uppercase;
            font-size: 11px;
            letter-spacing: 1px;
        }

        /* Table cell styling */
        QTableView {
            border: none;
            gridline-color: transparent; /* Hide the grid for a cleaner look */
            alternate-background-color: #232732; /* Subtle zebra striping */
        }

        /* Input fields focus state */
        QLineEdit:focus, QComboBox:focus {
            border: 1px solid #3b82f6; /* Blue border when typing */
        }

        QLabel#formTitle {
            color: #e5e7eb; /* Bright, clean white/gray */
            font-size: 14px;
            font-weight: bold;
            margin-bottom: 5px;
            background-color: #282c37;
        }

        /* Vertical Scrollbar styling */
        QScrollBar:vertical {
            border: none;
            background-color: #1e222d;
            width: 12px;
            border-radius: 6px;
        }

        QScrollBar::handle:vertical {
            background-color: #3f4451;
            min-height: 30px;
            border-radius: 6px;
        }

        QScrollBar::handle:vertical:hover {
            background-color: #4b5263;
        }

        /* Hide the up/down arrows on the scrollbar */
        QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {
            border: none;
            background: none;
            height: 0px;
        }

        /* Make Nav buttons flat and blend in */
        QPushButton#navBtn {
            background-color: transparent;
            color: #9ca3af;
            border: none;
            font-size: 14px;
        }

        QPushButton#navBtn:hover {
            color: #ffffff;
            background-color: #282c37;
        }
    )";
    window->setStyleSheet(qss);
}
