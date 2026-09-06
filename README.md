<img width="1280" height="640" alt="git (1)" src="https://github.com/user-attachments/assets/8920b256-2ba8-4988-b824-5351134eb4bd" />


# Gedi-Cpp 🎯


## Basic Details
### Team Name: Hog Riders from the east


### Team Members
- Member 1: Madhav Manoj - SCMS School of Engineering and Technology
- Member 2: Neeraj Praleep - SCMS School of Engineering and Technology

### Project Description
Gedi C++ (gedic) is a lightweight, esoteric C++ transpiler written in Rust that swaps standard syntax with authentic Malayalam and Thrissur slang. It pipes transpiled code directly into Clang via memory without creating messy temporary files, delivers custom Naadan compiler diagnostics on syntax failures, and features baked-in audio that blasts an elephant trumpet on compile errors and chenda melam on a successful build.

### The Problem (that doesn't exist)
Modern systems programming in C++ is plagued by clinical, emotionless keywords and foreign compiler diagnostics that fail to convey the sheer existential crisis of a missing semicolon. Gedi C++ solves the chronic shortage of authentic regional drama in low-level engineering by replacing sterile syntax with pure Naadan expressions, providing localized emotional closure through real-time Thrissur slang diagnostics, and holding developers audibly accountable via elephant screams on failure and celebratory chenda melam on success.  

### The Solution (that nobody asked for)
Pure Naadan Lexicon: Stripping away corporate std:: jargon and replacing it with battlefield-tested slang—pindam for void, chillara for floats, and paray_shavi for printing to console.  

In-Memory Pipe Sorcery: Built entirely in Rust to stream transpiled C++ code straight through Clang's stdin via process memory—because writing intermediate .cpp files to your disk is lame behavior.  

Aggressive Local Diagnostics: Intercepting cryptic compiler errors and translating them into unapologetic callouts, from "Semicolon evideda gediye?" to full-blown confusion warnings.  

Bespoke Binaural Reinforcement: Baking raw MP3 byte streams straight into the compiled binary so every syntax blunder triggers a traumatizing elephant trumpet, while a clean build drops an instant, glorious burst of chenda melam. 


## Technical Details
### Technologies/Components Used
For Software:
- Languages used: Rust, C++, Gedi C++
- None
- logos (high-performance lexer and token scanning), rodio (cross-platform audio playback engine with MP3 decoding)
- Tools used: Cargo, Clang++

### Implementation
# Installation
```zsh
# Clone the repository
git clone https://github.com/your-username/gedic.git
cd gedic

# Build the release binary with embedded audio assets
cargo build --release

# Install globally to your system PATH
sudo cp target/release/gedic /usr/local/bin/
```

# Run
```zsh
# Compile and name your executable output
gedic sample.gedi -o my_gedi_app

# Run the generated native binary
./my_gedi_app
```

| Note: Make sure your speaker volume is turned up to hear the build sound effects.

### Project Documentation
For Software:

# Screenshots
<img width="2556" height="372" alt="Successful compilation in terminal showing 'Sambhavam set aayi kdaave!' and execution output" src="https://media.discordapp.net/attachments/1428067931660484782/1545940206757224569/Screenshot_2026-09-06_at_4.38.40_AM.png?ex=6a9df883&is=6a9ca703&hm=6566e84fd9147f6f2ae8d8a3836472590d38fb3a7dafcfa8447673909f4b10c8&=&format=webp&quality=lossless&width=2556&height=372" /> Successful compilation in terminal showing "Sambhavam set aayi kdaave!" and execution output

<img width="2528" height="412" alt="Intentional missing semicolon triggering the Thrissur slang error '[SCENE] Semicolon evideda gediye?'" src="https://media.discordapp.net/attachments/1428067931660484782/1545940207377973349/Screenshot_2026-09-06_at_4.40.51_AM.png?ex=6a9df883&is=6a9ca703&hm=195070c95e76a435ad5b21523f8e72c5c94838d3a462a4c900736fd80a95f88f&=&format=webp&quality=lossless&width=2528&height=412" />Intentional missing semicolon triggering the Thrissur slang error '[SCENE] Semicolon evideda gediye?

<img width="2720" height="896" alt="Side-by-side view of a .gedi source file next to standard C++ to show syntax comparison" src="https://media.discordapp.net/attachments/1428067931660484782/1545940207751405679/Screenshot_2026-09-06_at_4.48.22_AM.png?ex=6a9df883&is=6a9ca703&hm=dce47c275c4557d1f45e0c24f1ae2939492f4331998d558cd13340803ddce7be&=&format=webp&quality=lossless&width=2720&height=896" />ide-by-side view of a .gedi source file next to standard C++ to show syntax comparison

# Diagrams
```
+---------------------+
|     source.gedi     |  (Gedi C++ source code)
+---------------------+
           |
           v
+---------------------+
|     gedic (Rust)    |  - Tokenize via Logos
|                     |  - Map slang tokens to C++ syntax
+---------------------+
           |
           v (stdin pipe: in-memory, no temp files on disk)
+---------------------+
|       clang++       |
+---------------------+
      /         \
     /           \
[Success]      [Failure]
   |                |
   v                v
Play Chenda       Play Elephant Scream
Melam SFX         + Naadan Error Diagnostics
   |
   v
Native Executable (./a.out)*
```

### Project Demo
# Video
[![Watch Gedi C++ Demo](./assets/thumbnail.png)](https://github.com/user-attachments/assets/8640b05c-ea51-41ce-88a7-a679983970fa)

# Additional Demos
[Add any extra demo materials/links]

## Team Contributions
- [Name 1]: [Specific contributions]
- [Name 2]: [Specific contributions]
- [Name 3]: [Specific contributions]

---
Made with ❤️ at TinkerHub Useless Projects 

![Static Badge](https://img.shields.io/badge/TinkerHub-24?color=%23000000&link=https%3A%2F%2Fwww.tinkerhub.org%2F)
![Static Badge](https://img.shields.io/badge/UselessProjects--26-26?link=https%3A%2F%2Ftinkerhub.org%2Fevents%2F1M8ORET9A1%2Fuseless-projects-3.0)
