#[macro_use]
extern crate serde_derive;
pub extern crate euclid;

use std::fmt;
use std::collections::HashMap;
use euclid::{rect, TypedRect};

#[derive(Debug, Serialize, Deserialize)]
pub enum Size {
    Fixed(i16),
    Flex(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Layout {
    Vertical,
    Horizontal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Composite {
    pub layout: Layout,
    pub items: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Cell {
    Composite(Composite),
    Leaf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub id: u8,
    pub size: Size,
    pub cell: Cell,
}

impl Component {
    pub fn leaf(id: u8, size: Size) -> Component {
        Component {
            id,
            size,
            cell: Cell::Leaf,
        }
    }

    pub fn comp(id: u8, size: Size, composite: Composite) -> Component {
        Component {
            id,
            size,
            cell: Cell::Composite(composite),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedComponent {
    pub id: u8,
    pub bounds: TypedRect<f64>,
}

impl fmt::Display for ResolvedComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bounds = self.bounds;
        write!(f,
            "(id: {} ({}, {}, {}, {}))",
            self.id,
            bounds.origin.x,
            bounds.origin.y,
            bounds.size.width,
            bounds.size.height
        )
    }
}

fn calculate_totals(composite: &Composite) -> (f64, f64) {
    composite.items.iter().fold(
        (0.0, 0.0),
        |(total_flex, total_fixed), component| match component.size {
            Size::Flex(flex) => (total_flex + flex, total_fixed),
            Size::Fixed(fixed) => (total_flex, total_fixed + fixed as f64),
        },
    )
}

fn horizontal(composite: &Composite, rectangle: TypedRect<f64>) -> Vec<ResolvedComponent> {
    let (flex_total, fixed_total) = calculate_totals(composite);
    // calculate what is left when all fixed sizes are added up
    let remaining = rectangle.size.width - fixed_total;
    let mut x = rectangle.origin.x;

    composite
        .items
        .iter()
        .flat_map(|component| {
            let width = match component.size {
                Size::Fixed(fixed) => fixed as f64,
                Size::Flex(flex) => flex / flex_total * remaining,
            };

            let new_rectangle = rect(x, rectangle.origin.y, width, rectangle.size.height);
            x += width;

            resolve(component, new_rectangle)
        })
        .collect()
}

fn vertical(composite: &Composite, rectangle: TypedRect<f64>) -> Vec<ResolvedComponent> {
    let (flex_total, fixed_total) = calculate_totals(composite);
    // calculate what is left when all fixed sizes are added up
    let remaining = rectangle.size.height - fixed_total;
    let mut y = rectangle.origin.y;

    composite
        .items
        .iter()
        .flat_map(|component| {
            let height = match component.size {
                Size::Fixed(fixed) => fixed as f64,
                Size::Flex(flex) => flex / flex_total * remaining,
            };

            let new_rectangle = rect(rectangle.origin.x, y, rectangle.size.width, height);
            y += height;

            resolve(component, new_rectangle)
        })
        .collect()
}

// Rect - width and height of the viewport for layout
pub fn resolve(component: &Component, rectangle: TypedRect<f64>) -> Vec<ResolvedComponent> {
    match component.cell {
        Cell::Leaf => {
            return vec![
                ResolvedComponent {
                    id: component.id,
                    bounds: rectangle.clone(),
                },
            ]
        }
        Cell::Composite(ref composite) => {
            return match composite.layout {
                Layout::Horizontal => horizontal(composite, rectangle.clone()),
                Layout::Vertical => vertical(composite, rectangle.clone()),
            }
        }
    };
}

pub fn to_hashmap(resolved: Vec<ResolvedComponent>) -> HashMap<u8, ResolvedComponent> {
    resolved.into_iter()
        .fold(HashMap::new(), |mut map, i| {
            let key = i.id.clone();
            map.insert(key, i);
            map
        })
}

#[cfg(test)]
mod test;
