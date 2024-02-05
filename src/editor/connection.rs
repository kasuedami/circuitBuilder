use std::collections::HashMap;

use itertools::Itertools;

#[derive(Default, Debug)]
pub struct Connections {
    lines: HashMap<LinePointIndex, Connection>,
}

impl Connections {
    pub fn insert_connection(&mut self, line_point_index: LinePointIndex, connection: Connection) {
        match connection {
            Connection {
                element: Element::Line(_),
                index: line_index,
            } if line_index == line_point_index.index => (),
            _ => {
                self.lines.insert(line_point_index, connection);
            }
        }
    }

    pub fn remove_for(&mut self, line_point_index: LinePointIndex) {
        self.lines.remove(&line_point_index);

        // TODO: check for lines that connect to this LinePointIndex
    }

    pub fn connections_for_connected(&self, connected: Connection) -> Vec<LinePointIndex> {
        self.lines
            .iter()
            .filter_map(|(line_point_index, connection)| {
                if connected == *connection {
                    Some(*line_point_index)
                } else {
                    None
                }
            })
            .unique()
            .collect()
    }

    pub fn connection_for_line_point_index(
        &self,
        line_point_index: LinePointIndex,
    ) -> Option<&Connection> {
        self.lines.get(&line_point_index)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub element: Element,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Element {
    Input,
    Output,
    Component(ComponentConnection),
    Line(LinePoint),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LinePointIndex {
    pub index: usize,
    pub point: LinePoint,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LinePoint {
    Start,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentConnection {
    pub input: bool,
    pub index: usize,
}

// TODO: probably a better way: save all connections in EditorCircuit,
// only apply the ones for moved elements
// everytime an element is released check if a new connections is formed
// everytime a line is clicked all its connections get removed
// every connection is between a line start or end and an input, output, component or other line, not itself
//
// create a connections data structure containing all connections for every line
