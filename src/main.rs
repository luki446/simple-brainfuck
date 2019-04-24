use std::fs::File;
use std::env;
use std::io::Read;

#[derive(PartialEq)]
enum Token
{
    Leftarrow,
    Rightarrow,
    Plus,
    Minus,
    Dot,
    Comma,
    Leftbracket(Option<usize>),
    Rightbracket(Option<usize>)
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
        let mut contest: String = String::new();
        
        {
            let mut file = File::open(&args[1]).expect("Can't open file");
            file.read_to_string(&mut contest).expect("Cannot read file to string");
        }

        let mut memo: [u8; 1024] = [0; 1024];
        let mut pointer = 0;

        let mut done: bool = false;
        let mut ite: usize= 0;

        let mut tokens = Vec::new();
        {
            let mut adresses: Vec<usize> = Vec::new();
            for (i, c) in contest.chars().enumerate()
            {
                match c
                {
                    '<' => tokens.push(Token::Leftarrow),
                    '>' => tokens.push(Token::Rightarrow),
                    '+' => tokens.push(Token::Plus),
                    '-' => tokens.push(Token::Minus),
                    '.' => tokens.push(Token::Dot),
                    ',' => tokens.push(Token::Comma),
                    '[' => 
                    {
                        adresses.push(i);
                        tokens.push(Token::Leftbracket(None));
                    },
                    ']' => {
                        match adresses.pop()
                        {
                            Some(x) => 
                            {
                                tokens[x] = Token::Leftbracket(Some(i));
                                tokens.push(Token::Rightbracket(Some(x)))
                            },
                            None =>
                            {
                                panic!("Missing opening bracket");
                            }
                        }
                    },
                    _ => continue,
                }
            }
            if !adresses.is_empty()
            {
                panic!("Something is fucked up with brackets, too much openings");
            }
        }

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
                Token::Plus =>
                    {
                        match memo[pointer].checked_add(1)
                            {
                                Some(x) => memo[pointer] = x,
                                None => memo[pointer] = u8::min_value(),
                            }
                    },
                Token::Minus =>
                    {
                        match memo[pointer].checked_sub(1)
                            {
                                Some(x) => memo[pointer] = x,
                                None => memo[pointer] = u8::max_value(),
                            }
                    },
                Token::Dot => print!("{}", memo[pointer] as char),
                Token::Comma =>
                {
                    let mut foo: [u8; 1] = [0; 1];
                    std::io::stdin().read_exact(&mut foo).expect("Cannot read from console");
                    memo[pointer] = foo[0];
                },
                Token::Leftbracket(x) =>
                {
                    if memo[pointer] == 0
                    {
                        match x {
                            Some(a) => ite = *a,
                            None => panic!("Brackets error"),
                        }
                    }
                },
                Token::Rightbracket(x) =>
                {
                    if memo[pointer] != 0
                    {
                        match x {
                            Some(a) => ite = *a,
                            None => panic!("Brackets error"),
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
