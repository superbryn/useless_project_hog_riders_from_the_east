use logos::Logos;
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs;
use std::io::{Cursor, Write};
use std::process::{Command, Stdio};

// ==========================================
// 🔊 SOUND EFFECTS (EMBEDDED INSIDE BINARY)
// ==========================================
// The audio files are baked directly into the compiled executable.
// No loose .wav files need to travel with your binary.
static CHENDA_MELAM: &[u8] = include_bytes!("../assets/chenda.mp3");
static AANA_ALARCHA: &[u8] = include_bytes!("../assets/elephant.mp3");

fn play_sfx(audio_data: &'static [u8]) {
    // Run playback in a dedicated thread so it doesn't block the terminal
    let handle = std::thread::spawn(move || {
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(_) => return, // Silent fallback if no audio device is connected
        };
        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(_) => return,
        };

        let cursor = Cursor::new(audio_data);
        if let Ok(source) = Decoder::new(cursor) {
            sink.append(source);
            sink.sleep_until_end();
        }
    });

    // Let the audio play for up to 3 seconds before exiting the program
    let _ = handle.join();
}

// ==========================================
// 🧠 LEXER & TOKEN DEFINITIONS
// ==========================================
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
        // Types
        "pindam" => Some("void"),
        "poornam" => Some("int"),
        "chillara" => Some("float"),
        "rand_chillara" => Some("double"),
        "charad" | "charadu" => Some("std::string"),
        "aksharam" => Some("char"),
        "bool" => Some("bool"),

        // Values
        "athe" => Some("true"),
        "thallu" => Some("false"),

        // Entry Point
        "pradhanam" => Some("main"),

        // I/O streams
        "paray_shavi" | "paray_gedi" => Some("std::cout"),
        "choikk_shavi" | "choikk_gedi" => Some("std::cin"),
        "theernnu" => Some("std::endl"),

        // Control Flow
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
    let mut out = String::from("#include <iostream>\n#include <string>\n\n");

    let mut pending_word: Option<String> = None;
    let mut pending_ws: Option<String> = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(RawToken::Word(w)) => {
                if let Some(prev) = pending_word.take() {
                    // Multi-token mapping: "igane allegil" -> "else if"
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

// ==========================================
// 💥 GEDI ERROR PARSER
// ==========================================
fn gedi_diagnostic(raw_err: &str) -> String {
    let mut transformed = Vec::new();

    for line in raw_err.lines() {
        let clean = line.replace("<stdin>:", "Line ");

        if clean.contains("expected ';'") {
            transformed.push("💀 [SCENE] Semicolon evideyaadey? Veettil ninnu idaan marannu poyo?".to_string());
        } else if clean.contains("use of undeclared identifier") {
            transformed.push("🤨 [AARA ITH] Angane oru saadhanathe njan jeevithathil kandittilla.".to_string());
        } else if clean.contains("expected '}'") || clean.contains("expected ')'") {
            transformed.push("🚪 [KOODARAM] Bracket thurannittu engotta poyi? Moothett varumo ath?".to_string());
        } else if clean.contains("reference to overloaded function") {
            transformed.push("💥 [KILI POYI] Built-in function-inte peril variable create cheyyalle Aliya.".to_string());
        } else if clean.contains("no matching function for call") {
            transformed.push("🚫 [MISMATCH] Set aavathilla... types thammil oru charcha nadakkunnilla.".to_string());
        } else if clean.trim().starts_with('^') || clean.contains("error:") {
            transformed.push(format!("   ↳ {}", clean.trim()));
        }
    }

    if transformed.is_empty() {
        "🔥 [AALU MAARI] Compiler aake confuse aayi koodaram ketti.".to_string()
    } else {
        transformed.join("\n")
    }
}

// ==========================================
// 🚀 COMPILER ENTRY POINT
// ==========================================
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

    let source = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("File vaayikkan pattunnilla mwone: {}", input_path);
        std::process::exit(1);
    });

    let cpp_stream = transpile(&source);

    // Pipe in-memory C++ directly into clang++
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
        play_sfx(CHENDA_MELAM);
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("\n================== GEDI RUNTIME ERROR ==================");
        eprintln!("{}", gedi_diagnostic(&stderr_str));
        eprintln!("========================================================\n");
        play_sfx(AANA_ALARCHA);
    }
}