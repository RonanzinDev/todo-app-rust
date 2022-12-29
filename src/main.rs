use ncurses::{
    addstr, attroff, attron, curs_set, endwin, getch, init_pair, initscr, mv, noecho, refresh,
    start_color, COLOR_BLACK, COLOR_PAIR, COLOR_WHITE,
};
use std::cmp::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGTH_PAIR: i16 = 1;

type Id = usize;
#[derive(Default)]
struct Ui {
    list_cur: Option<Id>,
}

impl Ui {
    fn begin(&mut self) {
        todo!();
    }

    fn label(&mut self, a: &str) {
        todo!()
    }
    fn begin_list(&mut self, id: Id) {
        todo!()
    }
    fn list_element(&mut self, label: &str, id: Id) {
        // let pair = {
        //     if todo_cur == index {
        //         // Renderizer em um estilo diferente
        //         HIGHLIGTH_PAIR
        //     } else {
        //         REGULAR_PAIR
        //     }
        // };
        // attron(COLOR_PAIR(pair));
        // mv(index as i32, 1);
        // addstr(*todo);
        // attroff(COLOR_PAIR(pair));
    }
    fn end_list(&mut self) {
        todo!()
    }

    fn end(&mut self) {
        todo!()
    }
}
fn main() {
    // Iniciar a tela
    initscr();

    start_color();

    noecho(); // tirando o echo(letras que ficam aparecendo quando apertamos alguma)

    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Tirando o cursor

    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);

    init_pair(HIGHLIGTH_PAIR, COLOR_BLACK, COLOR_WHITE);

    let dones = Vec::<String>::new();

    let todos: Vec<String> = vec![
        "Write the todo app".to_string(),
        "Buy a bread".to_string(),
        "Make a cup of tea".to_string(),
    ];
    // Currente Todo no cursor
    let mut ui = Ui::default();

    let mut todo_cur: usize = 0;
    let done_curr: usize = 0;

    let mut quit = false;

    while !quit {
        ui.begin();
        {
            ui.begin_list(todo_cur);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(&*todo.as_str(), index)
            }

            ui.end_list();

            ui.label("-----------------------------------------------------");

            ui.begin_list(done_curr);
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(done.as_str(), index)
            }

            ui.end_list();
        }
        ui.end();

        // Atualizando a tela
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' | 'A' => {
                if todo_cur > 0 {
                    todo_cur -= 1
                }
            }
            's' | 'B' => todo_cur = min(todo_cur + 1, todos.len() - 1), // para não ultrapassar
            _ => {}
        }
    }

    // Finalizando ncurses
    endwin();
}
