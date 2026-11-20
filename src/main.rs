//! HyperRogue CLI Game Runner and Visualizer

use hyperrogue_rs::engine::*;
use hyperrogue_rs::render::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut seed = 42;
    let mut steps = 10;
    let mut svg_path = None;
    let mut render_terminal = true;
    let mut run_nil = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                if i + 1 < args.len() {
                    seed = args[i + 1].parse().unwrap_or(42);
                    i += 1;
                }
            }
            "--steps" => {
                if i + 1 < args.len() {
                    steps = args[i + 1].parse().unwrap_or(10);
                    i += 1;
                }
            }
            "--svg" => {
                if i + 1 < args.len() {
                    svg_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--no-ascii" => {
                render_terminal = false;
            }
            "--nil" => {
                run_nil = true;
            }
            _ => {}
        }
        i += 1;
    }
    if run_nil {
        use hyperrogue_rs::nil::*;
        println!("=== HyperRogue: Nil Geometry (Heisenberg Group) ===");
        let mut nil_map = NilMap::new(NilStructure::ns6());
        nil_map.expand(3);
        println!("Nil Honeycomb: {} cells generated (Structure: {})", nil_map.cell_count(), nil_map.structure.name);
        println!("Nil Origin at (0, 0, 0)");

        let a = MVec::new(1, 2, 0);
        let b = MVec::new(0, 1, 0);
        let ab = a * b;
        let ba = b * a;
        println!("Non-commutative multiplication in Heisenberg group:");
        println!("  (1, 2, 0) * (0, 1, 0) = ({}, {}, {})", ab.x, ab.y, ab.z);
        println!("  (0, 1, 0) * (1, 2, 0) = ({}, {}, {})", ba.x, ba.y, ba.z);

        let v = hyperrogue_rs::math::HyperPoint::new(0.5, 0.3, 0.8);
        let exp_p = formula_exp(v, NilModel::Heisenberg);
        println!("Nil Geodesic Exponential Map exp(0.5, 0.3, 0.8):");
        println!("  Endpoint: ({:.4}, {:.4}, {:.4})", exp_p.x(), exp_p.y(), exp_p.z());
        return;
    }
    println!("Starting HyperRogue (Rust Recreation) with seed {}...", seed);
    let mut engine = GameEngine::new(seed);

    if render_terminal {
        let frame = Renderer::render_ascii(&mut engine, 70, 30);
        println!("{}", frame);
    }

    // Run simulated steps
    for step in 1..=steps {
        if engine.player.is_dead || engine.player.won {
            break;
        }

        // Choose valid action: try stepping in a passable direction
        let current_c = engine.player.walker.at;
        let deg = engine.world.tiling.cells[current_c].degree();
        let mut acted = false;

        for d in 0..deg {
            let next_c = engine.world.tiling.create_mov(current_c, d);
            if engine.is_passable(next_c, false) {
                acted = engine.process_player_action(PlayerAction::Step(d));
                if acted {
                    break;
                }
            }
        }

        if !acted {
            engine.process_player_action(PlayerAction::Wait);
        }

        if render_terminal && (step == steps || engine.player.is_dead || engine.player.won) {
            println!("--- After Step {} ---", step);
            let frame = Renderer::render_ascii(&mut engine, 70, 30);
            println!("{}", frame);
        }
    }

    if let Some(path) = svg_path {
        let svg = Renderer::render_svg(&mut engine, 600);
        fs::write(&path, svg).expect("Failed to write SVG");
        println!("Rendered hyperbolic disk visualization saved to {}", path);
    }

    println!("Finished game run at turn {}.", engine.turn_count);
    println!("Treasures collected: {}", engine.player.total_treasures());
    println!("Total kills: {}", engine.player.total_kills());
}
