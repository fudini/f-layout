use euclid::{TypedPoint2D, TypedRect, TypedSize2D};
use super::*;
use super::Layout::{Horizontal, Vertical};
use super::Size::{Fixed, Flex};

fn get_rect(x: f64, y: f64, w: f64, h: f64) -> TypedRect<f64> {
    TypedRect::new(TypedPoint2D::new(x, y), TypedSize2D::new(w, h))
}

pub fn size_to_rect((w, h): (u16, u16)) -> TypedRect<f64> {
    get_rect(0., 0., w as f64, h as f64)
}

pub fn get_layout() -> Component {
    let content_cell = Composite {
        layout: Horizontal,
        items: vec![
            Component::leaf(2, Fixed(20)),
            Component::leaf(3, Flex(3.)),
            Component::leaf(4, Flex(1.)),
        ],
    };

    let component = Component {
        id: 0,
        size: Flex(1.),
        cell: Cell::Composite(Composite {
            layout: Vertical,
            items: vec![
                Component::comp(0, Flex(1.), content_cell),
                Component::leaf(1, Fixed(1)),
            ],
        }),
    };

    component
}

#[test]
fn resolve_test() {
    let main_component = get_layout();
    let rect = size_to_rect((100, 50));
    let resolved = resolve(&main_component, rect);
    let result = vec![
        ResolvedComponent {
            id: 2,
            bounds: get_rect(0., 0., 20., 49.),
        },
        ResolvedComponent {
            id: 3,
            bounds: get_rect(20., 0., 60., 49.),
        },
        ResolvedComponent {
            id: 4,
            bounds: get_rect(80., 0., 20., 49.),
        },
        ResolvedComponent {
            id: 1,
            bounds: get_rect(0., 49., 100., 1.),
        },
    ];

    assert_eq!(resolved, result);
}
