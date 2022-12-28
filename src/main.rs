use ncurses::{addstr, endwin, getch, initscr, mv, refresh, start_color, init_pair, COLOR_BLACK, COLOR_WHITE, attron};

const REGULAR_PAIR: i16 = 0;
const HIGHLIGTH_PAIR: i16 = 1;


fn main() {
    // Iniciar a tela
    initscr();

    start_color();

    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGTH_PAIR, COLOR_BLACK, COLOR_WHITE);

    let todos = vec!["Write the todo app", "Buy a bread", "Make a cup of tea"];

    let todo_cur: usize = 0;

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
            attron(pair);
            mv(index as i32, 1);
            addstr(*todo);
        }
        // Atualizando a tela
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }
    // Aguardando por uma tecla

    // Finalizando ncurses
    endwin();
}

