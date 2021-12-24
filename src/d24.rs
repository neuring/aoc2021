use std::{collections::HashSet, rc::Rc};

use anyhow::{anyhow, bail, ensure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    X,
    Y,
    Z,
    W,
    Number(i64),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Div(Operand, Operand),
    Eql(Operand, Operand),
}

fn parse_operand(input: &str) -> anyhow::Result<Operand> {
    let operand = input.parse::<i64>();

    if let Ok(operand) = operand {
        return Ok(Operand::Number(operand));
    }

    let op = match input.chars().next() {
        Some('w') => Operand::W,
        Some('x') => Operand::X,
        Some('y') => Operand::Y,
        Some('z') => Operand::Z,
        _ => bail!("Invalid operand '{}'", input),
    };

    Ok(op)
}

fn parse_operand_with_two_args<'a>(
    op: fn(Operand, Operand) -> Instruction,
    mut remaining: impl Iterator<Item = &'a str>,
) -> anyhow::Result<Instruction> {
    let lhs = remaining.next().ok_or(anyhow!("Missing lhs operand"))?;
    let rhs = remaining.next().ok_or(anyhow!("Missing rhs operand"))?;

    let lhs = parse_operand(lhs)?;
    ensure!(
        !matches!(lhs, Operand::Number(_)),
        "left hand side of binary operation has to be variable."
    );
    let rhs = parse_operand(rhs)?;

    Ok(op(lhs, rhs))
}

fn parse(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let operand = parts.next().unwrap();

            match operand {
                "inp" => {
                    let op = parts.next().ok_or(anyhow!("Missing input operand"))?;
                    let op = parse_operand(op)?;
                    ensure!(
                        !matches!(op, Operand::Number(_)),
                        "Operand of input has to be variable."
                    );
                    Ok(Instruction::Inp(op))
                }
                "add" => parse_operand_with_two_args(Instruction::Add, parts),
                "mul" => parse_operand_with_two_args(Instruction::Mul, parts),
                "div" => parse_operand_with_two_args(Instruction::Div, parts),
                "mod" => parse_operand_with_two_args(Instruction::Mod, parts),
                "eql" => parse_operand_with_two_args(Instruction::Eql, parts),
                _ => bail!("Invalid instruction '{}'", operand),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NodeOp {
    Literal(i64),
    Add(Rc<Node>, Rc<Node>),
    Mul(Rc<Node>, Rc<Node>),
    Div(Rc<Node>, Rc<Node>),
    Mod(Rc<Node>, Rc<Node>),
    Eql(Rc<Node>, Rc<Node>),
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    id: u32,
    operation: NodeOp,
}

struct Constructor {
    next_id: u32,
}

impl Constructor {
    fn new_add(&mut self, lhs: Rc<Node>, rhs: Rc<Node>) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Add(lhs, rhs),
        }
    }

    fn new_mul(&mut self, lhs: Rc<Node>, rhs: Rc<Node>) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Mul(lhs, rhs),
        }
    }

    fn new_div(&mut self, lhs: Rc<Node>, rhs: Rc<Node>) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Div(lhs, rhs),
        }
    }

    fn new_mod(&mut self, lhs: Rc<Node>, rhs: Rc<Node>) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Mod(lhs, rhs),
        }
    }

    fn new_eql(&mut self, lhs: Rc<Node>, rhs: Rc<Node>) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Eql(lhs, rhs),
        }
    }
    fn new_input(&mut self, _op: Operand) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Input,
        }
    }

    fn new_literal(&mut self, val: i64) -> Node {
        Node {
            id: self.next_id(),
            operation: NodeOp::Literal(val),
        }
    }

    fn next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

fn build_graph_from_instructions(instructions: &[Instruction]) -> Node {
    let var_to_idx = |v: Operand| match v {
        Operand::X => 0,
        Operand::Y => 1,
        Operand::Z => 2,
        Operand::W => 3,
        Operand::Number(_) => panic!(),
    };

    let mut c = Constructor { next_id: 1 };

    let operand_to_graph =
        |op: Operand, vars: &[Node; 4], c: &mut Constructor| match op {
            Operand::X | Operand::Y | Operand::Z | Operand::W => {
                vars[var_to_idx(op)].clone()
            }
            Operand::Number(val) => c.new_literal(val),
        };
    let binary_instr_to_graph =
        |bin: fn(&mut Constructor, _, _) -> Node,
         lhs: Operand,
         rhs: Operand,
         vars: &mut [Node; 4],
         c: &mut Constructor| {
            let left = operand_to_graph(lhs, &vars, c);
            let right = operand_to_graph(rhs, &vars, c);
            vars[var_to_idx(lhs)] = bin(c, Rc::new(left), Rc::new(right))
        };

    let mut vars = [
        c.new_literal(0),
        c.new_literal(0),
        c.new_literal(0),
        c.new_literal(0),
    ];

    #[rustfmt::skip]
    let _ = for instr in instructions {
        match instr {
            Instruction::Inp(op) => vars[var_to_idx(*op)] = c.new_input(*op),
            Instruction::Add(lhs, rhs) => {
                binary_instr_to_graph(|c, l, r| c.new_add(l, r), *lhs, *rhs, &mut vars, &mut c);
            }
            Instruction::Mul(lhs, rhs) => {
                binary_instr_to_graph(|c, l, r| c.new_mul(l, r), *lhs, *rhs, &mut vars, &mut c);
            }
            Instruction::Div(lhs, rhs) => {
                binary_instr_to_graph(|c, l, r| c.new_div(l, r), *lhs, *rhs, &mut vars, &mut c);
            }
            Instruction::Mod(lhs, rhs) => {
                binary_instr_to_graph(|c, l, r| c.new_mod(l, r), *lhs, *rhs, &mut vars, &mut c);
            }
            Instruction::Eql(lhs, rhs) => {
                binary_instr_to_graph(|c, l, r| c.new_eql(l, r), *lhs, *rhs, &mut vars, &mut c);
            }
        };
    };

    vars[var_to_idx(Operand::Z)].clone()
}

fn dump_graph(graph: &Node) {
    fn dump_recursive(graph: &Node, visited: &mut HashSet<u32>) {
        if visited.contains(&graph.id) {
            return;
        } else {
            visited.insert(graph.id);
        }
        match &graph.operation {
            NodeOp::Literal(l) => {
                println!("{}[label={}]", graph.id, l);
            }
            NodeOp::Add(lhs, rhs) => {
                println!("{}[label=\"add\"]", graph.id);
                println!("{} -> {} [label=lhs]", graph.id, lhs.id);
                println!("{} -> {} [label=rhs]", graph.id, rhs.id);
                dump_recursive(&lhs, visited);
                dump_recursive(&rhs, visited);
            }
            NodeOp::Mul(lhs, rhs) => {
                println!("{}[label=\"mul\"]", graph.id);
                println!("{} -> {} [label=lhs]", graph.id, lhs.id);
                println!("{} -> {} [label=rhs]", graph.id, rhs.id);
                dump_recursive(&lhs, visited);
                dump_recursive(&rhs, visited);
            }
            NodeOp::Div(lhs, rhs) => {
                println!("{}[label=\"div\"]", graph.id);
                println!("{} -> {} [label=lhs]", graph.id, lhs.id);
                println!("{} -> {} [label=rhs]", graph.id, rhs.id);
                dump_recursive(&lhs, visited);
                dump_recursive(&rhs, visited);
            }
            NodeOp::Mod(lhs, rhs) => {
                println!("{}[label=\"mod\"]", graph.id);
                println!("{} -> {} [label=lhs]", graph.id, lhs.id);
                println!("{} -> {} [label=rhs]", graph.id, rhs.id);
                dump_recursive(&lhs, visited);
                dump_recursive(&rhs, visited);
            }
            NodeOp::Eql(lhs, rhs) => {
                println!("{}[label=\"eql\"]", graph.id);
                println!("{} -> {} [label=lhs]", graph.id, lhs.id);
                println!("{} -> {} [label=rhs]", graph.id, rhs.id);
                dump_recursive(&lhs, visited);
                dump_recursive(&rhs, visited);
            }
            NodeOp::Input => {
                println!("{}[label=\"input\"]", graph.id);
            }
        }
    }

    let mut visited = HashSet::new();
    println!("digraph {{");

    dump_recursive(graph, &mut visited);

    println!("}}");
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    let instructions = parse(&text)?;

    let mut current_vars = [0; 4];

    let op_idx = |v: Operand| match v {
        Operand::X => 0,
        Operand::Y => 1,
        Operand::Z => 2,
        Operand::W => 3,
        Operand::Number(_) => panic!(),
    };

    let var_str = |op: Operand, idx: u32| match op {
        Operand::X => format!("x{}", idx),
        Operand::Y => format!("y{}", idx),
        Operand::Z => format!("z{}", idx),
        Operand::W => format!("w{}", idx),
        Operand::Number(_) => panic!(),
    };

    let op_str = |op: Operand, vars: &[u32; 4]| match op {
        Operand::X | Operand::Y | Operand::Z | Operand::W => {
            var_str(op, vars[op_idx(op)])
        }
        Operand::Number(val) => format!("{}", val),
    };

    println!("(declare-const x0 Int)");
    println!("(declare-const y0 Int)");
    println!("(declare-const z0 Int)");
    println!("(declare-const w0 Int)");

    for (instr, t) in instructions.into_iter().zip(text.trim().lines()) {
        println!("; {}", t.trim());

        match instr {
            Instruction::Inp(o) => {
                let i = &mut current_vars[op_idx(o)];
                *i += 1;
                println!("(declare-const {} Int)", var_str(o, *i));
                println!("(assert (>= {} 1))", var_str(o, *i));
                println!("(assert (<= {} 9))", var_str(o, *i));
            }
            Instruction::Add(lhs, rhs) => {
                let old_lhs_idx = current_vars[op_idx(lhs)];
                let i = &mut current_vars[op_idx(lhs)];
                *i += 1;

                println!("(declare-const {} Int)", var_str(lhs, *i));
                println!(
                    "(assert (= {} (+ {} {})))",
                    var_str(lhs, *i),
                    var_str(lhs, old_lhs_idx),
                    op_str(rhs, &current_vars),
                );
            }
            Instruction::Mul(lhs, rhs) => {
                let old_lhs_idx = current_vars[op_idx(lhs)];
                let i = &mut current_vars[op_idx(lhs)];
                *i += 1;

                println!("(declare-const {} Int)", var_str(lhs, *i));
                println!(
                    "(assert (= {} (* {} {})))",
                    var_str(lhs, *i),
                    var_str(lhs, old_lhs_idx),
                    op_str(rhs, &current_vars),
                );
            }
            Instruction::Div(lhs, rhs) => {
                let old_lhs_idx = current_vars[op_idx(lhs)];
                let i = &mut current_vars[op_idx(lhs)];
                *i += 1;

                println!("(declare-const {} Int)", var_str(lhs, *i));
                println!(
                    "(assert (= {} (div {} {})))",
                    var_str(lhs, *i),
                    var_str(lhs, old_lhs_idx),
                    op_str(rhs, &current_vars),
                );
                println!("(assert (not (= {} 0)))", op_str(rhs, &current_vars));
            }
            Instruction::Mod(lhs, rhs) => {
                let old_lhs_idx = current_vars[op_idx(lhs)];
                let i = &mut current_vars[op_idx(lhs)];
                *i += 1;

                println!("(declare-const {} Int)", var_str(lhs, *i));
                println!(
                    "(assert (= {} (mod {} {})))",
                    var_str(lhs, *i),
                    var_str(lhs, old_lhs_idx),
                    op_str(rhs, &current_vars),
                );
                println!("(assert (> {} 0))", op_str(rhs, &current_vars));
                println!("(assert (>= {} 0))", var_str(lhs, old_lhs_idx));
            }
            Instruction::Eql(lhs, rhs) => {
                let old_lhs_idx = current_vars[op_idx(lhs)];
                let i = &mut current_vars[op_idx(lhs)];
                *i += 1;

                println!("(declare-const {} Int)", var_str(lhs, *i));
                println!(
                    "(assert (= {} (if (= {} {}) 1 0)))",
                    var_str(lhs, *i),
                    var_str(lhs, old_lhs_idx),
                    op_str(rhs, &current_vars),
                );
            }
        }
    }

    println!("(check-sat)");
    println!("(get-value ({}))", op_str(Operand::Z, &current_vars));
    println!("(exit)");

    Ok(0)
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    todo!()
}
