#[macro_use] extern crate text_io;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::Read;

enum Token
{
    Leftarrow,
    Rightarrow,
    Plus,
    Minus,
    Dot,
    Comma,
    Leftbracket,
    Rightbracket
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()< 2
    {
        println!("Please choose file ");
        println!("Ex: ./simple-brainfuck example.bf");
    }
    else
    {
        let mut file = File::open(&args[1]).expect("Can't open file");
        let mut contest: String = String::new();
        file.read_to_string(&mut contest).expect("Cannot read file to string");

        let mut memo: [u8; 1024] = [0; 1024];
        let mut pointer = 0;

        let mut done: bool = false;
        let mut ite: usize;

        let mut tokens = Vec::new();
        for c in contest.chars()
        {
            match c
            {
                '<' => tokens.push(Token::Leftarrow),
                '>' => tokens.push(Token::Rightarrow),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '.' => tokens.push(Token::Dot),
                ',' => tokens.push(Token::Comma),
                '[' => tokens.push(Token::Leftbracket),
                ']' => tokens.push(Token::Rightbracket),
                _ => continue,
            }
        }

        ite = 0;

        while !done
        {
            let i = &tokens[ite];
            match i
            {
                Token::Leftarrow =>
                {
                    if pointer > 0
                    {
                    pointer -= 1;
                    }
                },
                Token::Rightarrow => pointer += 1,
                Token::Plus => memo[pointer] += 1,
                Token::Minus => memo[pointer] -= 1,
                Token::Dot => print!("{}", memo[pointer] as char),
                Token::Comma =>
                {
                    let mut foo: [u8; 1] = [0; 1];
                    std::io::stdin().read_exact(&mut foo).expect("Cannot read from console");
                    memo[pointer] = foo[0];
                },
                Token::Leftbracket =>
                {
                    if memo[pointer] == 0
                    {
                        let mut muv = ite;
                        loop {
                            match tokens[muv]
                            {
                                Token::Rightbracket =>
                                {
                                    ite = muv;
                                    break;
                                },
                                _ => {
                                    muv += 1;
                                },
                            }
                        }

                    }
                },
                Token::Rightbracket =>
                {
                    if memo[pointer] != 0
                    {
                        let mut muv = ite;
                        loop {
                            match tokens[muv]
                            {
                                Token::Leftbracket =>
                                {
                                    ite = muv;
                                    break;
                                },
                                _ => {
                                    muv -= 1;
                                },
                            }
                        }
                    }
                },
            }
            ite += 1;
            if ite == tokens.len()
            {
                done = true;
            }
        }
    }

}
