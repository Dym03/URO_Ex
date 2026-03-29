#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow) {
    this->resize(1400, 800);
    this->setFont(QFont("Roboto", 15));
    this->create_inventory();
    create_actions();
    create_menu();
    create_layout();
    apply_dark_theme(this);
}

void MainWindow::create_inventory() {
    this->inventory = parse_bikes_csv("bikes.csv");
}

void MainWindow::create_menu() {
    fileMenu = menuBar()->addMenu(tr("&File"));
    fileMenu->addAction(newAct);
    fileMenu->addAction(openAct);
    fileMenu->addAction(saveAct);

    editMenu = menuBar()->addMenu(tr("&Edit"));
    editMenu->addAction(newAct);
    editMenu->addAction(openAct);
    editMenu->addAction(saveAct);
}

void MainWindow::create_actions() {
    newAct = new QAction(tr("&New"), this);
    newAct->setShortcuts(QKeySequence::New);
    newAct->setStatusTip(tr("Create a new file"));
    openAct = new QAction(tr("&Open"), this);
    openAct->setShortcuts(QKeySequence::Open);
    openAct->setStatusTip(tr("Open a file"));
    saveAct = new QAction(tr("&Save"), this);
    saveAct->setShortcuts(QKeySequence::Save);
    saveAct->setStatusTip(tr("Save"));

    undoAct = new QAction(tr("&Undo"), this);
    undoAct->setShortcuts(QKeySequence::Undo);
    undoAct->setStatusTip(tr("Undo"));
    redoAct = new QAction(tr("&Redo"), this);
    redoAct->setShortcuts(QKeySequence::Redo);
    redoAct->setStatusTip(tr("Redo"));


}

void MainWindow::create_layout() {
    QWidget * central_widget = new QWidget();
    this->top_level_layout = new QVBoxLayout(central_widget);
    this->top_level_layout->setContentsMargins(20, 20, 20, 20);
    this->setCentralWidget(central_widget);

    this->create_navbar();

    this->view_stack = new QStackedWidget();

    HomeWidget* home_page = new HomeWidget(this, this->inventory);
    StoreWidget* store_page = new StoreWidget(this, this->inventory);

    this->view_stack->addWidget(home_page);  // Index 0
    this->view_stack->addWidget(store_page); // Index 1

    this->top_level_layout->addWidget(view_stack);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::create_navbar() {
    QHBoxLayout * navbar_layout = new QHBoxLayout();

    navbar_layout->setAlignment(Qt::AlignTop);

    QLabel * app_name = new QLabel("Bike App");
    app_name->setObjectName("appTitle");
    QPushButton* home = new QPushButton("Home");
    QPushButton* store = new QPushButton("Store");
    QPushButton* info = new QPushButton("Info");

    home->setObjectName("navBtn");
    store->setObjectName("navBtn");
    info->setObjectName("navBtn");
    connect(info, SIGNAL(clicked()), this, SLOT(on_info_clicked()));
    connect(home, SIGNAL(clicked()), this, SLOT(on_home_clicked()));
    connect(store, SIGNAL(clicked()), this, SLOT(on_store_clicked()));

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

void MainWindow::on_home_clicked() {
    if (this->view_stack->currentIndex() != 0) {
        this->view_stack->setCurrentIndex(0);
    }

}

void MainWindow::on_store_clicked() {
    if (this->view_stack->currentIndex() != 1) {
        this->view_stack->setCurrentIndex(1);
    }
}

void MainWindow::on_send_clicked() {
    info_window->done(0);
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

        QFrame#cardLabel {
            background-color: #282c37 !important;
            font-size: 12px
        }
        QFrame#formCard {
            background-color: #282c37 !important;
            border: 1px solid #3f4451 !important;
            border-radius: 12px;
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

        #appTitle {
            color: #e5e7eb;
            font-size: 22px;
            font-weight: bold;
            margin-bottom: 5px;
        }

        QLabel#formTitle {
            color: #e5e7eb; /* Bright, clean white/gray */
            font-size: 22px;
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
