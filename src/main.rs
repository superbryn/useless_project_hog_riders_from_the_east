use logos::Logos;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Logos, Debug, PartialEq)]
enum RawToken<'a> {
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral(&'a str),

    #[regex(r"//[^\n]*")]
    LineComment(&'a str),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Word(&'a str),

    #[regex(r"[ \t\n\r]+")]
    Whitespace(&'a str),

    #[regex(r#"[{}()\[\];,.<>?:=+\-*/%!&|^~]"#)]
    Symbol(&'a str),
}

fn map_keyword(word: &str) -> Option<&'static str> {
    match word {
        "pindam" => Some("void"),
        "poornam" => Some("int"),
        "chillara" => Some("float"),
        "rand_chillara" => Some("double"),
        "charadu" => Some("std::string"),
        "aksharam" => Some("char"),
        "bool" => Some("bool"),
        "athe" => Some("true"),
        "thallu" => Some("false"),
        "pradhanam" => Some("main"),
        "paray_gedi" => Some("std::cout"),
        "choikk_gedi" => Some("std::cin"),
        "theernnu" => Some("std::endl"),
        "igane" => Some("if"),
        "allegil" => Some("else"),
        "cheyy_gedi" => Some("do"),
        "eppol" => Some("while"),
        "inna_pidicho" => Some("return"),
        _ => None,
    }
}

fn transpile(source: &str) -> String {
    let mut lexer = RawToken::lexer(source);
    // Line offset directive so compiler errors match the original .gedi file lines
    let mut out = String::from("#include <iostream>\n#include <string>\n");

    let mut pending_word: Option<String> = None;
    let mut pending_ws: Option<String> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(RawToken::Word(w)) => {
                if let Some(prev) = pending_word.take() {
                    if prev == "igane" && w == "allegil" {
                        out.push_str("else if");
                        pending_ws = None;
                        continue;
                    } else {
                        let mapped = map_keyword(&prev).unwrap_or(&prev);
                        out.push_str(mapped);
                        if let Some(ws) = pending_ws.take() {
                            out.push_str(&ws);
                        }
                    }
                }
                pending_word = Some(w.to_string());
            }
            Ok(RawToken::Whitespace(ws)) => {
                if pending_word.is_some() {
                    pending_ws = Some(ws.to_string());
                } else {
                    out.push_str(ws);
                }
            }
            _ => {
                if let Some(prev) = pending_word.take() {
                    let mapped = map_keyword(&prev).unwrap_or(&prev);
                    out.push_str(mapped);
                    if let Some(ws) = pending_ws.take() {
                        out.push_str(&ws);
                    }
                }
                out.push_str(lexer.slice());
            }
        }
    }

    if let Some(prev) = pending_word.take() {
        let mapped = map_keyword(&prev).unwrap_or(&prev);
        out.push_str(mapped);
        if let Some(ws) = pending_ws.take() {
            out.push_str(&ws);
        }
    }

    out
}

fn gedi_diagnostic(raw_err: &str) -> String {
    let mut transformed = Vec::new();

    for line in raw_err.lines() {
        // Strip compiler internals mentioning <stdin>
        let clean = line.replace("<stdin>:", "Line ");

        if clean.contains("expected ';'") {
            transformed.push("[SCENE CONDRA] Semicolon evideyaadey? Veettil ninnu idaan marannu poyoda shavi?".to_string());
        } else if clean.contains("use of undeclared identifier") {
            transformed.push("[AARA ITH] Angane oru saadhanathe njan jeevithathil kandittilla.".to_string());
        } else if clean.contains("expected '}'") || clean.contains("expected ')'") {
            transformed.push("[KODARAM] Bracket thurannittu engotta poyada shavi? Moothett varumo ath?".to_string());
        } else if clean.contains("reference to overloaded function") {
            transformed.push("[KILI POYI] Built-in functions-nte peru keri thallalle mwone, clash aayi.".to_string());
        } else if clean.contains("no matching function for call") {
            transformed.push("[MISMATCH] Set aavathilla... type thammil oru link-um illaloda kdavee👶🏿.".to_string());
        } else if clean.trim().starts_with('^') || clean.contains("error:") {
            transformed.push(format!(" Enthada pundachi ith:  ↳ {}", clean.trim()));
        }
    }

    if transformed.is_empty() {
        "[AALU MAARI] Compiler aake confuse aayi koodaram ketti.".to_string()
    } else {
        transformed.join("\n")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: gedic <source.gedi> [-o output_binary]");
        return;
    }

    let input_path = &args[1];
    let binary_name = if args.len() >= 4 && args[2] == "-o" {
        &args[3]
    } else {
        "a.out"
    };

    let source = fs::read_to_string(input_path).expect("File vaayikkan pattunnilla mwone!");
    let cpp_stream = transpile(&source);

    // Pass C++ via stdin directly into clang++ (-x c++ tells clang to treat stdin as C++)
    let mut child = Command::new("clang++")
        .args(["-x", "c++", "-", "-o", binary_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("clang++ launch cheyyan pattiyilla");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(cpp_stream.as_bytes())
            .expect("Stdin-ilot stream cheyyan pattiyilla");
    }

    let output = child.wait_with_output().expect("Compiler hang aayi");

    if output.status.success() {
        println!("✨ Sambhavam set aayi! Binary ready: ./{}", binary_name);
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("\n================== GEDI RUNTIME ERROR ==================");
        eprintln!("{}", gedi_diagnostic(&stderr_str));
        eprintln!("========================================================\n");
    }
}