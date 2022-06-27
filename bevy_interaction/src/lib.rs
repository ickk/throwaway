use bevy::prelude::*;

#[derive(Debug, Clone)]
struct Rect {
  start: Vec2,
  end: Vec2,
}

#[derive(Debug, Clone)]
struct Circle {
  centre: Vec2,
  radius: f32,
}

#[derive(Debug, Clone)]
enum Primitive {
  Rect(Rect),
  Circle(Circle),
}

impl Primitive {
  fn contains(&self, point: Vec2) -> bool {
    match self {
      Primitive::Rect(rect) => {
        point.x >= rect.start.x
          && point.x <= rect.end.x
          && point.y >= rect.start.y
          && point.y <= rect.end.y
      }
      Primitive::Circle(circle) => (point - circle.centre).length() <= circle.radius,
    }
  }
}

#[derive(Debug, Clone)]
enum Operation {
  // Index into array of primitives
  Primitive(usize),
  // Transformation Operations
  Rotation {
    index: usize, // Index into array of operations
    rotation: f32,
  },
  // Boolean Operations
  Union {
    left: usize, // Indices into array of operations
    right: usize,
  },
  Intersection {
    left: usize,
    right: usize,
  },
  Difference {
    left: usize,
    right: usize,
  },
  // TODO: Add complement & Translation operators.
}

#[derive(Component, Debug, Clone)]
pub struct Region {
  ops: Vec<Operation>,
  primitives: Vec<Primitive>,
}

impl Region {
  pub fn contains(&self, point: Vec2) -> bool {
    // From the root node
    self._contains(&point, self.ops.last().unwrap())
  }

  pub fn from_rect(start: Vec2, end: Vec2) -> Self {
    Self {
      ops: vec![Operation::Primitive(0)],
      primitives: vec![Primitive::Rect(Rect { start, end })],
    }
  }

  pub fn from_circle(centre: Vec2, radius: f32) -> Self {
    Self {
      ops: vec![Operation::Primitive(0)],
      primitives: vec![Primitive::Circle(Circle { centre, radius })],
    }
  }

  pub fn rotate(&mut self, rotation: f32) {
    self.ops.push(Operation::Rotation {
      rotation,
      index: self.ops.len() -1,
    })
  }

  pub fn union(&mut self, other: Self) {
    let ops_len = ready_boolean(self, other);
    self.ops.push(Operation::Union {
      left: ops_len,
      right: self.ops.len() -1,
    });
  }

  pub fn intersection(&mut self, other: Self) {
    let ops_len = ready_boolean(self, other);
    self.ops.push(Operation::Intersection {
      left: ops_len,
      right: self.ops.len() -1,
    });
  }

  pub fn difference(&mut self, other: Self) {
    let ops_len = ready_boolean(self, other);
    self.ops.push(Operation::Difference {
      left: ops_len,
      right: self.ops.len() -1,
    });
  }

  fn _contains(&self, point: &Vec2, node: &Operation) -> bool {
    match node {
      Operation::Primitive(index) => {
        self.primitives.get(*index).unwrap().contains(*point)
      },
      Operation::Rotation{index, rotation} => {
        // Project points onto local coordinate system rotated by `self.1` radians.
        let local_x = Vec2::new(rotation.cos(), rotation.sin());
        let local_y = Vec2::new(-rotation.sin(), rotation.cos());
        let projected_point = Vec2::new(point.dot(local_x), point.dot(local_y));
        self._contains(&projected_point, self.ops.get(*index).unwrap())
      }
      Operation::Union{left, right} => {
        self._contains(point, self.ops.get(*left).unwrap())
          || self._contains(point, self.ops.get(*right).unwrap())
      },
      Operation::Intersection{left, right} => {
        self._contains(point, self.ops.get(*left).unwrap())
          && self._contains(point, self.ops.get(*right).unwrap())
      },
      Operation::Difference{left, right} => {
        self._contains(point, self.ops.get(*left).unwrap())
          && !self._contains(point, self.ops.get(*right).unwrap())
      },
    }
  }
}

// appends right onto left, being sure to adjust indices.
// returns the original length of left before the merge.
#[inline]
fn ready_boolean(left: &mut Region, mut right: Region) -> usize {
  let ops_len = left.ops.len()-1;
  let prim_len = left.primitives.len();
  for mut op in right.ops.iter_mut() {
    adjust_indices(&mut op, ops_len, prim_len);
  }
  left.ops.append(&mut right.ops);
  left.primitives.append(&mut right.primitives);

  return ops_len;
}

#[inline]
fn adjust_indices(op: &mut Operation, op_offset: usize, prim_offset: usize) {
  match op {
    Operation::Primitive(index) => *index += prim_offset,
    Operation::Rotation {index, ..} => *index += op_offset,
    Operation::Union {left, right}
    | Operation::Intersection {left, right}
    | Operation::Difference {left, right} => {
      *left += op_offset;
      *right += op_offset;
    }
  }
}
