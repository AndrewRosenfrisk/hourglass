use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::{thread_rng, Rng};
use std::{collections::HashSet, io::stdout, thread, time::Duration};

const _SCREEN_WIDTH: usize = 79;
const _SCREEN_HEIGHT: usize = 25;

const WALL_CHAR: char = '█';
const SAND_CHAR: char = '░';
const SPACE: char = ' ';
fn main() {
    let mut hourglass = HashSet::new();

    for i in 18..37 {
        hourglass.insert((i, 1));
        hourglass.insert((i, 23));
    }
    for i in 1..5 {
        hourglass.insert((18, i));
        hourglass.insert((36, i));
        hourglass.insert((18, i + 19));
        hourglass.insert((36, i + 19));
    }
    for i in 0..8 {
        hourglass.insert((19 + i, 5 + i));
        hourglass.insert((35 - i, 5 + i));
        hourglass.insert((25 - i, 13 + i));
        hourglass.insert((29 + i, 13 + i));
    }
    let mut initial_sand = HashSet::new();

    for y in 0..8 {
        for x in (19 + y)..(36 - y) {
            initial_sand.insert((x, y + 4));
        }
    }
    execute!(
        stdout(),
        Clear(All),
        Clear(Purge),
        Hide,
        DisableLineWrap,
        MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print("Ctrl-C to quit.")
    )
    .unwrap();

    print_initial_object(&hourglass, WALL_CHAR);

    for _ in 0..10 {
        //arbitrary 10 loop end for re-simulating
        print_initial_object(&initial_sand, SAND_CHAR);
        run_simulation(&mut initial_sand.clone(), hourglass.clone());
    }
}

fn run_simulation(sand: &mut HashSet<(u16, u16)>, hourglass: HashSet<(u16, u16)>) {
    loop {
        let mut frame_did_update = false;
        for (_i, (x, y)) in sand.clone().iter().enumerate() {
            if can_move_to_cell((*x, y + 1), &sand, &hourglass) {
                move_sand_sprite(sand, (*x, *y), (*x, *y + 1));
                frame_did_update = true;
            } else {
                let can_fall_below_left = can_move_to_cell((*x - 1, *y), &sand, &hourglass)
                    && can_move_to_cell((*x - 1, y + 1), &sand, &hourglass);

                let can_fall_below_right = can_move_to_cell((*x + 1, *y), &sand, &hourglass)
                    && can_move_to_cell((*x + 1, y + 1), &sand, &hourglass);

                let delta_x;
                if thread_rng().gen_bool(0.5) {
                    let can_fall_far_left = can_fall_below_left
                        && can_move_to_cell((*x - 2, *y + 1), &sand, &hourglass);
                    let can_fall_far_right = can_fall_below_right
                        && can_move_to_cell((*x + 2, *y + 1), &sand, &hourglass);

                    delta_x = set_delta_x(can_fall_far_left, can_fall_far_right, 2)
                } else {
                    delta_x = set_delta_x(can_fall_below_left, can_fall_below_right, 1);
                }

                if delta_x != 0 {
                    let new_x = x.checked_add_signed(delta_x).unwrap();
                    move_sand_sprite(sand, (*x, *y), (new_x, *y + 1));
                    frame_did_update = true;
                }
            }
        }
        thread::sleep(Duration::from_secs_f32(0.2));
        if !frame_did_update {
            for (x, y) in sand.iter() {
                execute!(stdout(), MoveTo(*x, *y), Print(SPACE)).unwrap();
            }
            break;
        }
    }
}

fn can_move_to_cell(
    cell: (u16, u16),
    sand: &HashSet<(u16, u16)>,
    hourglass: &HashSet<(u16, u16)>,
) -> bool {
    !sand.contains(&cell) && !hourglass.contains(&cell)
}

fn set_delta_x(left: bool, right: bool, delta: i16) -> i16 {
    if left || (left && thread_rng().gen_bool(0.5)) {
        -delta
    } else if right {
        delta
    } else {
        0
    }
}

fn move_sand_sprite(sand: &mut HashSet<(u16, u16)>, old: (u16, u16), new: (u16, u16)) {
    execute!(
        stdout(),
        MoveTo(old.0, old.1),
        Print(SPACE),
        MoveTo(new.0, new.1),
        Print(SAND_CHAR)
    )
    .unwrap();
    sand.remove(&(old.0, old.1));
    sand.insert((new.0, new.1));
}

fn print_initial_object(obj: &HashSet<(u16, u16)>, char: char) {
    for cell in obj {
        execute!(stdout(), MoveTo(cell.0, cell.1), Print(char)).unwrap();
    }
}
