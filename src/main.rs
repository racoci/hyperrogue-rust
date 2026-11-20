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
            "--nil-penrose" => {
                let path = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    i += 1;
                    args[i].clone()
                } else {
                    "nil_penrose.svg".to_string()
                };
                let tribar = hyperrogue_rs::nil::PenroseTribar::new(4.0, 0.35);
                fs::write(&path, tribar.to_svg(600)).expect("Failed to write Penrose SVG");
                println!("Nil Penrose Impossible Triangle saved to {}", path);
                return;
            }
            "--nil-staircase" => {
                let path = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    i += 1;
                    args[i].clone()
                } else {
                    "nil_staircase.svg".to_string()
                };
                let stair = hyperrogue_rs::nil::NilStaircase::new(4.0, 4);
                fs::write(&path, stair.to_svg(600)).expect("Failed to write Staircase SVG");
                println!("Nil Ascending Staircase saved to {}", path);
                return;
            }
            "--gui" => {
                use hyperrogue_rs::gui::*;
                let mut app = InteractiveApp::new(GuiMode::NilWorld).expect("Failed to create GUI window");
                app.run().expect("GUI error");
                return;
            }
            "--gui-penrose" => {
                use hyperrogue_rs::gui::*;
                let mut app = InteractiveApp::new(GuiMode::PenroseTribar).expect("Failed to create GUI window");
                app.run().expect("GUI error");
                return;
            }
            "--gui-staircase" => {
                use hyperrogue_rs::gui::*;
                let mut app = InteractiveApp::new(GuiMode::NilStaircase).expect("Failed to create GUI window");
                app.run().expect("GUI error");
                return;
            }
            "--web" => {
                let port = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    i += 1;
                    args[i].parse().unwrap_or(8080)
                } else {
                    8080
                };
                hyperrogue_rs::web::WebServer::serve(port).expect("Web server failed");
                return;
            }
            "--html" => {
                let path = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    i += 1;
                    args[i].clone()
                } else {
                    "nil_interactive.html".to_string()
                };
                fs::write(&path, hyperrogue_rs::web::WebServer::html_app()).expect("Failed to write HTML");
                println!("Interactive 60 FPS HTML5 Canvas app saved to {}", path);
                return;
            }
            _ => {}
        }
        i += 1;
    }
    if run_nil {
        use hyperrogue_rs::nil::*;
        println!("=== HyperRogue: Nil Geometry (Heisenberg Group H_3) ===");
        let mut game = NilGame::new(NilStructure::ns6(), seed);
        println!("Nil Honeycomb: {} cells generated (Structure: {})", game.map.cell_count(), game.map.structure.name);
        println!("Player starts at Nil origin (0, 0, 0)");

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

        // Demonstrate 5-step commutator loop: -X -> -Y -> +X -> +Y -> -Z
        println!("\nTesting Heisenberg 5-step loop closure (-X -> -Y -> +X -> +Y -> -Z):");
        let p0 = game.player_mvec();
        println!("  Initial: ({}, {}, {})", p0.x, p0.y, p0.z);
        game.step_free(0); // -X
        println!("  After -X: ({}, {}, {})", game.player_mvec().x, game.player_mvec().y, game.player_mvec().z);
        game.step_free(1); // -Y
        println!("  After -Y: ({}, {}, {})", game.player_mvec().x, game.player_mvec().y, game.player_mvec().z);
        game.step_free(3); // +X
        println!("  After +X: ({}, {}, {})", game.player_mvec().x, game.player_mvec().y, game.player_mvec().z);
        game.step_free(4); // +Y
        println!("  After +Y: ({}, {}, {}) [Climbed vertically by +1 due to [X, Y]=Z]", game.player_mvec().x, game.player_mvec().y, game.player_mvec().z);
        game.step_free(2); // -Z
        println!("  After -Z: ({}, {}, {}) [Closed back to initial cell!]", game.player_mvec().x, game.player_mvec().y, game.player_mvec().z);
        assert_eq!(game.player_mvec(), p0);

        if render_terminal {
            println!("\nNil 3D Space View (current layer and adjacent Z levels):");
            println!("{}", game.render_ascii(50, 15));
        }
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
