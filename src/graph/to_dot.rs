use std::{fmt::Display, fs::File, io::Write};

use super::Graph;

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Red,
    Blue,
    Black,
    RGB(u8, u8, u8),
}
// TODO add more options

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Red => write!(f, "red"),
            Color::Blue => write!(f, "blue"),
            Color::Black => write!(f, "black"),
            Color::RGB(_, _, _) => todo!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum NodeShape {
    Ellipse,
    Box,
    Circle,
    Doublecircle,
    Diamond,
    Plaintext,
    Record,
    Polygon,
}

impl Display for NodeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeShape::Ellipse => write!(f, "ellipse"),
            NodeShape::Box => write!(f, "box"),
            NodeShape::Circle => write!(f, "circle"),
            NodeShape::Doublecircle => write!(f, "doublecircle"),
            NodeShape::Diamond => write!(f, "diamond"),
            NodeShape::Plaintext => write!(f, "plaintext"),
            NodeShape::Record => write!(f, "record"),
            NodeShape::Polygon => write!(f, "polygon"),
        }
    }
}
#[derive(Clone, Copy)]
pub struct NodeRule {
    pub color: Color,
    pub fill_color: Color,
    pub label: bool,
    pub shape: NodeShape,
}

impl Default for NodeRule {
    fn default() -> Self {
        Self {
            color: Color::Black,
            fill_color: Color::White,
            label: true,
            shape: NodeShape::Ellipse,
        }
    }
}
#[derive(Clone, Copy)]
pub enum VersicleStyle {
    Box,
    Crow,
    Curve,
    Diamond,
    Dot,
    Icurve,
    Inv,
    None,
    Normal,
    Tee,
    Vee,
}

impl Display for VersicleStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersicleStyle::Box => write!(f, "box"),
            VersicleStyle::Crow => write!(f, "crow"),
            VersicleStyle::Curve => write!(f, "curve"),
            VersicleStyle::Diamond => write!(f, "diamond"),
            VersicleStyle::Dot => write!(f, "dot"),
            VersicleStyle::Icurve => write!(f, "icurve"),
            VersicleStyle::Inv => write!(f, "inv"),
            VersicleStyle::None => write!(f, "none"),
            VersicleStyle::Normal => write!(f, "normal"),
            VersicleStyle::Tee => write!(f, "tee"),
            VersicleStyle::Vee => write!(f, "vee"),
        }
    }
}
#[derive(Clone, Copy)]
pub struct VersicleRule {
    pub color: Color,
    pub label: bool,
    pub style: VersicleStyle,
}

impl Default for VersicleRule {
    fn default() -> Self {
        Self {
            color: Color::Black,
            label: true,
            style: VersicleStyle::Normal,
        }
    }
}
///! takes ownership of path
pub fn color_path_node(path: Vec<usize>, node_rule: NodeRule) -> impl Fn(usize) -> NodeRule {
    move |n: usize| {
        if path.contains(&n) {
            node_rule
        } else {
            default_node_rule(n)
        }
    }
}

pub fn color_path_versicle(
    path: Vec<(usize, usize)>,
    versicle_rule: VersicleRule,
) -> impl Fn(usize, usize) -> VersicleRule {
    move |n1: usize, n2: usize| {
        if path.contains(&(n1, n2)) {
            versicle_rule
        } else {
            default_versicle_rule(n1, n2)
        }
    }
}

pub fn default_node_rule(_node: usize) -> NodeRule {
    NodeRule {
        color: Color::Black,
        fill_color: Color::White,
        label: true,
        shape: NodeShape::Ellipse,
    }
}

pub fn default_versicle_rule(_n1: usize, _n2: usize) -> VersicleRule {
    VersicleRule {
        color: Color::Black,
        label: true,
        style: VersicleStyle::Normal,
    }
}

impl<V: Display + Clone, N: Display> Graph<V, N> {
    pub(crate) fn to_dot(&self, file_name: &str) -> std::io::Result<()> {
        self.to_dot_with_rules(file_name, default_node_rule, default_versicle_rule)
    }
    // expected fn pointer `fn(usize) -> NodeRule`
    // found opaque type `impl Fn<(usize,)>`
    pub(crate) fn to_dot_with_rules(
        &self,
        file_name: &str,
        node_rule: impl Fn(usize) -> NodeRule,
        versicle_rule: impl Fn(usize, usize) -> VersicleRule,
    ) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(b"digraph g{ \n")?;

        self.format_node(node_rule, &mut file)?;

        for node in 0..self.get_nodes_amount() {
            for n in self.get_neighbors(node).unwrap() {
                let rule = versicle_rule(node, n);
                file.write_all(format!("{} -> {} [", node, n,).as_bytes())?;
                if rule.label {
                    file.write_all(
                        format!("label = {} ", self.get_ver_value(node, n).as_ref().unwrap())
                            .as_bytes(),
                    )?;
                }
                file.write_all(format!("color = {} ", rule.color).as_bytes())?;
                file.write_all(format!("arrowhead = {} ]\n", rule.style).as_bytes())?;
            }
        }
        file.write_all(b"}\n")?;
        Ok(())
    }

    fn format_node(
        &self,
        node_rule: impl Fn(usize) -> NodeRule,
        file: &mut File,
    ) -> Result<(), std::io::Error> {
        for node in 0..self.get_nodes_amount() {
            let rule = node_rule(node);
            file.write_all(format!("{} [", node).as_bytes())?;
            file.write_all(format!("color = {} ,", rule.color).as_bytes())?;
            if rule.label {
                file.write_all(
                    format!("label = {} ,", self.get_node_value(node).as_ref().unwrap()).as_bytes(),
                )?;
            }
            file.write_all(format!("shape = {},", rule.shape).as_bytes())?;
            file.write_all(format!("fillcolor = {}, style = filled", rule.fill_color).as_bytes())?;
            file.write_all("]\n".to_string().as_bytes())?;
        };
        Ok(())
    }
}
