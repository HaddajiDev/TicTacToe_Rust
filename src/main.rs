use eframe::egui;
use egui::{Color32, FontId, TextStyle, Vec2};

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    X,
    O,
}

struct TicTacToe {
    board: [[CellState; 3]; 3],
    current_player: CellState,
    game_over: bool,
    winner: Option<CellState>,
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self {
            board: [[CellState::Empty; 3]; 3],
            current_player: CellState::X,
            game_over: false,
            winner: None,
        }
    }
}

impl TicTacToe {
    fn check_win(&self) -> Option<CellState> {
        for row in 0..3 {
            if self.board[row][0] != CellState::Empty &&
                self.board[row][0] == self.board[row][1] &&
                self.board[row][0] == self.board[row][2] {
                return Some(self.board[row][0]);
            }
        }

        for col in 0..3 {
            if self.board[0][col] != CellState::Empty &&
                self.board[0][col] == self.board[1][col] &&
                self.board[0][col] == self.board[2][col] {
                return Some(self.board[0][col]);
            }
        }

        if self.board[0][0] != CellState::Empty &&
            self.board[0][0] == self.board[1][1] &&
            self.board[0][0] == self.board[2][2] {
            return Some(self.board[0][0]);
        }

        if self.board[0][2] != CellState::Empty &&
            self.board[0][2] == self.board[1][1] &&
            self.board[0][2] == self.board[2][0] {
            return Some(self.board[0][2]);
        }

        None
    }

    fn check_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != CellState::Empty))
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl eframe::App for TicTacToe {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tic Tac Toe");
            
            let button_font = FontId::proportional(24.0);
            
            for row in 0..3 {
                ui.horizontal(|ui| {
                    for col in 0..3 {
                        let cell = &mut self.board[row][col];
                        let text = match cell {
                            CellState::X => "X",
                            CellState::O => "O",
                            CellState::Empty => "",
                        };
                        
                        let button = egui::Button::new(
                            egui::RichText::new(text)
                                .font(button_font.clone())
                                .size(32.0)
                        )
                        .min_size(Vec2::new(64.0, 64.0));
                        
                        if ui.add(button).clicked() && *cell == CellState::Empty && !self.game_over {
                            *cell = self.current_player;
                            
                            if let Some(winner) = self.check_win() {
                                self.game_over = true;
                                self.winner = Some(winner);
                            } else if self.check_draw() {
                                self.game_over = true;
                            } else {
                                self.current_player = match self.current_player {
                                    CellState::X => CellState::O,
                                    _ => CellState::X,
                                };
                            }
                        }
                    }
                });
            }

            if self.game_over {
                if let Some(winner) = self.winner {
                    ui.label(
                        egui::RichText::new(format!("Player {} wins!", match winner {
                            CellState::X => "X",
                            CellState::O => "O",
                            _ => unreachable!(),
                        }))
                        .color(Color32::GREEN)
                        .font(FontId::proportional(24.0))
                    );
                } else {
                    ui.label(
                        egui::RichText::new("It's a draw!")
                            .color(Color32::YELLOW)
                            .font(FontId::proportional(24.0))
                    );
                }
            } else {
                ui.label(
                    egui::RichText::new(format!("Current Player: {}", match self.current_player {
                        CellState::X => "X",
                        CellState::O => "O",
                        _ => unreachable!(),
                    }))
                    .font(FontId::proportional(24.0))
                );
            }

            if ui.button("Reset Game").clicked() {
                self.reset();
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 400.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Box::new(TicTacToe::default())),
    );
}