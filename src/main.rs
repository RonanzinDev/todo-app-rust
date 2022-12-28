use ncurses::{addstr, endwin, getch, initscr, mv, refresh, start_color, init_pair, COLOR_BLACK, COLOR_WHITE, attron, COLOR_PAIR, attroff};
use std::cmp::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGTH_PAIR: i16 = 1;


fn main() {
    // Iniciar a tela
    initscr();

    start_color();

    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGTH_PAIR, COLOR_BLACK, COLOR_WHITE);

    let todos = vec!["Write the todo app", "Buy a bread", "Make a cup of tea"];
    // Currente Todo no cursor
    let mut todo_cur: usize = 0;

    let mut quit = false;
    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_cur == index {
                // Renderizer em um estilo diferente
                HIGHLIGTH_PAIR
            } else {
                REGULAR_PAIR
            }
        };
            attron(COLOR_PAIR(pair));
            mv(index as i32, 1);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }
        // Atualizando a tela
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' | 'A' => if todo_cur > 0 {
                todo_cur -= 1
            },
            's' | 'B'=> todo_cur = min(todo_cur + 1, todos.len() - 1), // para não ultrapassar
            _ => {}
        }

    }
    // Aguardando por uma tecla

    // Finalizando ncurses
    endwin();
}

