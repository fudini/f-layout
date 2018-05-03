extern crate layout;
extern crate euclid;

use euclid::{rect};
use layout::{Component, Composite, Cell, Layout, Size, resolve};
use layout::Size::{Flex, Fixed};

fn main() {

  let main_cell = Composite {
    layout: Layout::Vertical,
    items: vec!(
      Component { id: 2, size: Fixed(1), cell: Cell::Leaf },
      Component { id: 3, size: Flex(3.0), cell: Cell::Leaf },
    )
  };

  let component = Component {
    id: 1,
    size: Size::Flex(1.0),
    cell: Cell::Composite(Composite {
      layout: Layout::Horizontal,
      items: vec!(
        Component { id: 0, size: Flex(1.0), cell: Cell::Composite(main_cell) },
        Component { id: 1, size: Fixed(1), cell: Cell::Leaf },
      )
    }),
  };

  let resolved = resolve(&component, rect(0.0, 0.0, 50.0, 50.0));

  println!("{:?}", resolved);
}
