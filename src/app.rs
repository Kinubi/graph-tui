use crossterm::event::{ KeyCode, KeyEvent };

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Graph,
    GraphEditor,
    NodeEditor,
    EdgeEditor,
    Exiting,
}

#[derive(Debug)]
pub enum NodeEditorMode {
    Label,
}

#[derive(Debug)]
pub enum EdgeEditorMode {
    Label,
    InOuts(InOut),
}

#[derive(Debug)]
pub enum InOut {
    From,
    To,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Node(NodeEditorMode),
    Edge(EdgeEditorMode),
}

use crate::graph::Graph;
#[derive(Debug)]
pub struct App {
    pub graph: Graph,
    pub exit: bool,
    pub label: String,
    pub in_outs: [u64; 2],
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self {
            graph: Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            exit: false,
            label: String::new(),
            in_outs: [0, 0],
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn update(&mut self) {
        todo!();
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        // Handle screen navigation and business logic
        match self.current_screen {
            CurrentScreen::Main => {
                match key.code {
                    KeyCode::Char('g') | KeyCode::Char('G') => {
                        self.current_screen = CurrentScreen::Graph;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }
            }
            CurrentScreen::Graph => {
                match key.code {
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
            }
            CurrentScreen::GraphEditor => {
                match key.code {
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.label.clear();
                        self.currently_editing = Some(
                            CurrentlyEditing::Node(NodeEditorMode::Label)
                        );
                        self.current_screen = CurrentScreen::NodeEditor;
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.label.clear();
                        self.in_outs = [0, 0];
                        self.currently_editing = Some(
                            CurrentlyEditing::Edge(EdgeEditorMode::Label)
                        );
                        self.current_screen = CurrentScreen::EdgeEditor;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.current_screen = CurrentScreen::Graph;
                    }
                    _ => {}
                }
            }
            CurrentScreen::NodeEditor => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.label.clear();
                        self.currently_editing = None;
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Enter => {
                        self.add_node();
                        self.label.clear();
                        self.current_screen = CurrentScreen::GraphEditor;
                        self.currently_editing = None;
                    }
                    KeyCode::Backspace => {
                        if let Some(CurrentlyEditing::Node(mode)) = &self.currently_editing {
                            match mode {
                                NodeEditorMode::Label => {
                                    self.label.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        self.currently_editing = None;
                        self.label.clear();
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Char(value) => {
                        if let Some(CurrentlyEditing::Node(mode)) = &self.currently_editing {
                            match mode {
                                NodeEditorMode::Label => {
                                    self.label.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            CurrentScreen::EdgeEditor => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.label.clear();
                        self.in_outs = [0, 0];
                        self.currently_editing = None;
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Enter => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::To))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.add_edge();
                                    self.label.clear();
                                    self.in_outs = [0, 0];
                                    self.currently_editing = None;
                                    self.current_screen = CurrentScreen::GraphEditor;
                                }
                            }
                        }
                    }
                    KeyCode::Tab => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::To))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::Label)
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.label.pop();
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.in_outs[0] = 0;
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::Label)
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.in_outs[1] = 0;
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        self.currently_editing = None;
                        self.label.clear();
                        self.in_outs = [0, 0];
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Char(value) => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.label.push(value);
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.in_outs[0] = value.to_digit(10).unwrap_or(0) as u64;
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.in_outs[1] = value.to_digit(10).unwrap_or(0) as u64;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            CurrentScreen::Exiting => {
                match key.code {
                    KeyCode::Char('y') => {
                        self.exit = true;
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn add_node(&mut self) {
        let id = (self.graph.nodes.len() as u64) + 1;
        self.graph.nodes.push(crate::graph::Node {
            id,
            label: self.label.clone(),
        });
    }

    pub fn add_edge(&mut self) {
        let from = self.in_outs[0];
        let to = self.in_outs[1];
        let id = (self.graph.edges.len() as u64) + 1;
        self.graph.edges.push(crate::graph::Edge {
            id,
            from,
            to,
            label: self.label.clone(),
        });
    }

    pub fn on_tick(&mut self) {
        self.update();
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn print_nodes(&self) {
        for node in &self.graph.nodes {
            println!("Node {}: {}", node.id, node.label);
        }
    }
}
