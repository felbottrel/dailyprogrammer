use std::fmt;

#[derive(Debug)]
enum Op {
    Clear,
    Origin,
    Beginning,
    Down,
    Up,
    Left,
    Right,
    Erase,
    Insert,
    Overwrite,
    Move(u8, u8),
    Write(char),
}

#[derive(Debug)]
struct Cursor(u8, u8);

#[derive(Debug)]
struct Terminal {
    terminal: [[char; 10]; 10],
    mode: u8,
    cursor: Cursor,
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
        let mut display = String::new();

        for arrays in self.terminal.iter() {
            for chars in arrays.iter(){
                display.push(*chars);
            }
            display.push('\n');
        }

        write!(f, "{}", display)
    }
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            terminal: [[' '; 10]; 10],
            mode: 0,
            cursor: Cursor(0, 0),
        }
    }

    fn clear(&mut self) {
        self.terminal = [[' '; 10]; 10];
    }

    fn origin(&mut self) {
        self.cursor.0 = 0;
        self.cursor.1 = 0;
    }

    fn beginning(&mut self) {
        self.cursor.1 = 0;
    }

    fn down(&mut self) {        
        if self.cursor.0 != 9 {
             self.cursor.0 = self.cursor.0 + 1;
        }
    }

    fn up(&mut self) {
        if self.cursor.0 != 0 {
            self.cursor.0 = self.cursor.0 - 1;
        }    
    }

    fn left(&mut self) {
        if self.cursor.1 != 0 {
            self.cursor.1 = self.cursor.1 - 1;
        }      
    }

    fn right(&mut self) {
        if self.cursor.1 != 9 {
            self.cursor.1 = self.cursor.1 + 1;
        }
    }

    fn erase(&mut self) {
        for i in (self.cursor.1)..10 {
            self.terminal[self.cursor.0 as usize][i as usize] = ' ';
        }
    }

    fn insert(&mut self) {
       self.mode = 1;
    }

    fn overwrite(&mut self) {
       self.mode = 2;
    }

    fn moves(&mut self, x: u8, y: u8) {
        if x > 9 || y > 9 {
            panic!("Index out of bounds. x: {}, y: {}", x, y);
        }

        self.cursor.0 = x;
        self.cursor.1 = y;
    }

    fn write(&mut self, c: char) {
        self.terminal[self.cursor.0 as usize][self.cursor.1 as usize] = c;
        if self.mode == 1 && self.cursor.1 != 9{
            self.right();
        }
    }

    fn do_cmd(&mut self, cmd: Op){
        //println!("Doing {:?}", cmd);
        match cmd {
            Op::Clear => self.clear(),
            Op::Origin => self.origin(),
            Op::Beginning => self.beginning(),
            Op::Down => self.down(),
            Op::Up => self.up(),
            Op::Left => self.left(),
            Op::Right => self.right(),
            Op::Erase => self.erase(),
            Op::Insert => self.insert(),
            Op::Overwrite => self.overwrite(),
            Op::Move(x, y) => self.moves(x, y),
            Op::Write(c) => self.write(c),
        }
    }
}

fn parse_inputs(input: &str, terminal: &mut Terminal) {
    let mut state: u8 = 0;

    let mut iter = input.chars();
    'iter: while let Some(chars) = iter.next(){

        loop{
            //println!("Starting loop with state = {}, cursor is = {:?}", state, terminal.cursor);
            match state {
                0 => {
                    //println!("State 0, char = {}", chars);
                    if chars == '^' {
                        state = 1;
                        continue 'iter;
                    }else{
                        terminal.do_cmd(Op::Write(chars));
                        break;
                    }
                },
                1 => {                    
                    if chars == '^' {
                        terminal.do_cmd(Op::Write('^'));

                        state = 0;
                        break;
                    } else if "0123456789".find(chars).is_some() {
                        let x: u8 = chars.to_digit(10).unwrap() as u8;
                        
                        let y: u8 = match iter.next(){
                            Some(c) => c.to_digit(10).unwrap() as u8,
                            None => panic!("Input is invalid"),
                        };

                        terminal.do_cmd(Op::Move(x, y));
                        
                        state = 0;
                        break;
                    } else if let Some(x) = "chbdulreio".find(chars) {
                        let cmd = match x {
                            0 => Op::Clear,
                            1 => Op::Origin,
                            2 => Op::Beginning,
                            3 => Op::Down,
                            4 => Op::Up,
                            5 => Op::Left,
                            6 => Op::Right,
                            7 => Op::Erase,
                            8 => Op::Insert,
                            9 => Op::Overwrite,
                            _ => panic!("Impossible!"),
                        };
                        terminal.do_cmd(cmd);

                        state = 0;
                        break;
                    }
                },
                _ => panic!("State shouldn't happen"),
            }
        }

    }
}


fn main() {
    let mut terminal = Terminal::new();

    let input = "^h^c^iDDD^r^rPPPP^d^bD^r^rD^rP^19P^d^bD^r^rD^rPPPP^d^bD^r^rD^rP^d^bDDD^r^rP";
    
    parse_inputs(input, &mut terminal);

    println!("{}", terminal);
}