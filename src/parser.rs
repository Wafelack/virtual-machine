use crate::{Instruction, Instruction::*, Register::*};
use std::collections::BTreeMap;
use std::fs;

fn error(line: usize, whr: &str, message: &str) {
  eprintln!("{} | {}", line, whr);
  eprintln!("^^^^^^^^^^^^^^^^^^^^");
  eprintln!("{}\n", message);
}

pub fn parse_code(code: &str, quit: bool) -> (Vec<Instruction>, BTreeMap<String, i32>) {
  let mut instrs: Vec<Instruction> = vec![];
  let mut labels: BTreeMap<String, i32> = BTreeMap::new();
  let mut had_error = false;

  let lines = code.split('\n').collect::<Vec<&str>>();
  let mut ln = 0usize;

  for line in &lines {
    ln += 1;
    let splited = line.split(' ').collect::<Vec<&str>>();
    if line.starts_with(":") {
      // Labels
      labels.insert(splited[0].to_owned(), (ln) as i32); // Minus two because of human notation and Instruction launching (see main.rs)
      continue;
    }
  }

  ln = 0;

  for line in lines {
    ln += 1;
    let splited = line.split(' ').collect::<Vec<&str>>();

    if line.starts_with(";") {
      continue;
    }
    if line.starts_with(":") {
      continue;
    }

    if line.is_empty() {
      continue;
    }

    match splited[0] {
      "dmp" => instrs.push(Dmp),
      "gto" => {
        if splited.len() < 2 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `gto <label|instruction>``",
          );
          had_error = true;
          continue;
        }

        let raw = splited[1];

        let num = match raw.parse::<i32>() {
          Ok(n) => n,
          Err(_) => {
            println!("{:?}", labels);
            if labels.contains_key(raw) {
              labels[raw]
            } else {
              error(
                ln,
                line,
                "Type error: gto has to take a valid integer or label !",
              );
              had_error = true;
              continue;
            }
          }
        };

        instrs.push(Gto(num));
      }
      "prt" => {
        if splited.len() < 2 {
          error(ln, line, "Syntax error: valid syntax: `prt <register>`");
          had_error = true;
          continue;
        }

        let raw = splited[1];

        let reg = if raw == "a" {
          A
        } else if raw == "b" {
          B
        } else if raw == "c" {
          C
        } else if raw == "e" {
          D
        } else if raw == "d" {
          E
        } else if raw == "f" {
          F
        } else if raw == "ip" {
          Ip
        } else if raw == "sp" {
          Sp
        } else if raw == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw),
          );
          had_error = true;
          continue;
        };

        instrs.push(Prt(reg))
      }
      "tee" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `tee <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Tee(reg_a, reg_b));
      }
      "tne" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `tne <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Tne(reg_a, reg_b));
      }
      "tll" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `tll <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Tll(reg_a, reg_b));
      }
      "tmm" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `tmm <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Tmm(reg_a, reg_b));
      }
      "tel" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `tel <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Tel(reg_a, reg_b));
      }
      "tem" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `tem <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Tem(reg_a, reg_b));
      }
      "jmp" => {
        if splited.len() < 2 {
          error(ln, line, "Syntax error: valid syntax: `jmp <instruction>`");
          had_error = true;
          continue;
        }

        let instruction = match splited[1].parse::<i32>() {
          Ok(i) => i,
          Err(_e) => {
            println!("{:?}", labels);
            if labels.contains_key(splited[1]) {
              labels[splited[1]]
            } else {
              error(
                ln,
                line,
                "Type error: gto has to take a valid integer or label !",
              );
              had_error = true;
              continue;
            }
          }
        };

        instrs.push(Jmp(instruction));
      }
      "psh" => {
        if splited.len() < 2 {
          error(ln, line, "Syntax error: valid syntax: `psh <integer>`");
          had_error = true;
          continue;
        }

        let to_psh = match splited[1].parse::<i32>() {
          Ok(i) => i,
          Err(_e) => {
            error(
              ln,
              line,
              &format!("Type error : {} is not a valid integer", splited[1]),
            );
            had_error = true;
            continue;
          }
        };

        instrs.push(Psh(to_psh));
      }
      "mov" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `mov <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Mov(reg_a, reg_b));
      }

      "add" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `mov <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Add(reg_a, reg_b));
      }
      "sub" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `mov <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Sub(reg_a, reg_b));
      }
      "mul" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `mov <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Mul(reg_a, reg_b));
      }
      "div" => {
        if splited.len() < 3 {
          error(
            ln,
            line,
            "Syntax error: valid syntax: `mov <register_a> <register_b>`",
          );
          had_error = true;
          continue;
        }

        let raw_a = splited[1];
        let raw_b = splited[2];

        let reg_a = if raw_a == "a" {
          A
        } else if raw_a == "b" {
          B
        } else if raw_a == "c" {
          C
        } else if raw_a == "e" {
          D
        } else if raw_a == "d" {
          E
        } else if raw_a == "f" {
          F
        } else if raw_a == "ip" {
          Ip
        } else if raw_a == "sp" {
          Sp
        } else if raw_a == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_a),
          );
          had_error = true;
          continue;
        };

        let reg_b = if raw_b == "a" {
          A
        } else if raw_b == "b" {
          B
        } else if raw_b == "c" {
          C
        } else if raw_b == "e" {
          D
        } else if raw_b == "d" {
          E
        } else if raw_b == "f" {
          F
        } else if raw_b == "ip" {
          Ip
        } else if raw_b == "sp" {
          Sp
        } else if raw_b == "st" {
          St
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw_b),
          );
          had_error = true;
          continue;
        };

        instrs.push(Div(reg_a, reg_b));
      }
      "pop" => instrs.push(Pop),
      "drg" => {
        if splited.len() < 2 {
          error(ln, line, "Syntax error: valid syntax: `drg <register>`");
          had_error = true;
          continue;
        }

        let raw = splited[1];

        let reg = if raw == "a" {
          A
        } else if raw == "b" {
          B
        } else if raw == "c" {
          C
        } else if raw == "e" {
          D
        } else if raw == "d" {
          E
        } else if raw == "f" {
          F
        } else if raw == "ip" {
          Ip
        } else if raw == "sp" {
          Sp
        } else if raw == "st" {
          St
        } else if raw == "eq" {
          Eq
        } else {
          error(
            ln,
            line,
            &format!("Type error : {} is not a valid register", raw),
          );
          had_error = true;
          continue;
        };

        instrs.push(Drg(reg));
      }
      "hlt" => {
        instrs.push(Hlt);
        return (instrs, labels);
      }

      x => {
        error(ln, line, &format!("Error: Unexpected token: {}", x));
        had_error = true;
        continue;
      }
    }
  }
  if had_error {
    eprintln!("Aborting due to previous errors");
    if quit {
      std::process::exit(-7);
    }
  }
  instrs.push(Hlt);
  (instrs, labels)
}
