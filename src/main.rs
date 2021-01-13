use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Token {
    Text(String),
    Underscore,
    Asterisk,
    Hash(u16)
}

fn compile(fname: &str) {
    
    let fpath = Path::new(fname);
    let f = File::open(&fpath)
                .expect("Couldn't open file");

    // this will be the output
    let mut html: Vec<String> = Vec::new();

    // we don't handle any multiline states,
    // so each line can be tokenized individually
    for line in BufReader::new(f).lines() {
        let tokens = tokenize(&line.unwrap());
        if tokens.len() > 0 {
            html.push(parse(tokens));
        }
    }

    for h in html {
        println!("{}", h)
    }

}


fn parse(tokens: Vec<Token>) -> String {
    

    let tags = match &tokens[0] {
        Token::Text(_) => ("<p>", "</p>"),
        Token::Hash(0) => ("<h1>", "</h1>"),
        Token::Hash(1) => ("<h2>", "</h2>"),
        Token::Hash(2) => ("<h3>", "</h3>"),
        Token::Asterisk => ("", ""),
        Token::Underscore => ("", ""),
        e => panic!("Error, bad start token, {:?}", e)
    };

    let mut output: String = tags.0.to_string();
    
    let mut em = false;
    let mut b = false;

    for (i, toke) in tokens.iter().enumerate() {
       let out = match (i, toke) {
           (_, Token::Text(t)) => t.to_string(),
           
           (_, Token::Asterisk) => {
               b = !b;
               if b {"<b>".to_string()} else {"</b>".to_string()}
           }

           (_, Token::Underscore) => {
               em = !em;
               if em {"<em>".to_string()} else {"</em>".to_string()}
           }
           
           (0, Token::Hash(_)) => "".to_string(),

           (_, e) => panic!("Bad token! {:?}", e)
       }; 

       output.push_str(&out);

    }

    output.push_str(tags.1);

    return output;
}


fn tokenize(line: &str) -> Vec<Token> {
    
    // to hold the output
    let mut tokens: Vec<Token> = Vec::new();

    // to act as a text buffer
    let mut text = String::new();

    let mut iter = line.chars().peekable();
    let mut hash_count = 0;

    while let Some(character) = iter.next() {
        
        if "_#*".contains(character) && (text.len() > 0) {
            // if we've hit a special char flush the text
            tokens.push(Token::Text(text));
            text = "".to_string();
        } 

        if (character.to_string() == "#") 
                         && (iter.peek().unwrap().to_string() == "#") {
            // if we hit a # and the next char is a hash, increase count
            hash_count += 1

        } else if (character.to_string() == "#") 
                         && (iter.peek().unwrap().to_string() != "#") {
            tokens.push(Token::Hash(hash_count));
            hash_count = 0;

        } else if character.to_string() == "_" {
            tokens.push(Token::Underscore);

        } else if character.to_string() == "*" {
            tokens.push(Token::Asterisk);

        } else {
            text.push_str(&character.to_string());
        
        }
    } 

    if text.len() > 0 {
        tokens.push(Token::Text(text));
    }
    return tokens;

}


fn usage() {
    let desc = env!("CARGO_PKG_DESCRIPTION");
    let name = env!("CARGO_PKG_NAME");
    println!("{}, {}", name, desc);
    println!("Usage: {} [file.md]", name);
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => compile(&args[1]),
        _ => usage(),
    }


}
