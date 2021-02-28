#[derive(Debug)]
struct General_structure<'a> {
  abc_array: &'a Vec<[usize; 3]>,
  x: i32,
  matrix: Vec<[usize; 3]>,
  current: i32,
}
fn create_new_abc_array() -> Vec<[usize; 3]> {
  let mut new_abc_array: Vec<[usize; 3]> = Vec::new();

  for a in 0..25 {
    for b in 0..25 {
      for c in 0..25 {
        new_abc_array.push([a, b, c])
      }
    }
  }

  new_abc_array
}

fn comparitor(vector_a: [usize; 3], vector_b: [usize; 3]) -> bool {
  let mut return_bool: bool = false;

  for k in vector_a.iter() {
    if k == &vector_b[0] {
      return_bool = true
    }
    if k == &vector_b[1] {
      return_bool = true
    }
    if k == &vector_b[2] {
      return_bool = true
    }
  }

  return_bool
}

fn munge_matrix(new_gs: &mut General_structure) {
  match new_gs.matrix.len() {
    0 => new_gs
      .matrix
      .push(new_gs.abc_array[new_gs.current as usize]),
    1 if new_gs.x < new_gs.abc_array.len() as i32
      && !comparitor(new_gs.matrix[0], new_gs.abc_array[new_gs.x as usize]) =>
    {
      new_gs.matrix.push(new_gs.abc_array[new_gs.x as usize])
    }
    2 if new_gs.x < new_gs.abc_array.len() as i32
      && !comparitor(new_gs.matrix[0], new_gs.abc_array[new_gs.x as usize])
      && !comparitor(new_gs.matrix[1], new_gs.abc_array[new_gs.x as usize]) =>
    {
      new_gs.matrix.push(new_gs.abc_array[new_gs.x as usize])
    }
    3 => {
      new_gs.x = 0;
      new_gs.current += 1;
      println!("{:?}", new_gs.matrix);
      new_gs.matrix.clear();
    }
    _ => (),
  }
}

fn recursize_matrix_builder(new_gs: &mut General_structure) {
  new_gs.x += 1;
  new_gs.current += 1;
  if new_gs.current > (new_gs.abc_array.len() as i32) {
    return;
  }
  munge_matrix(new_gs);
  recursize_matrix_builder(new_gs);
}

fn matrix_builder(mut new_gs: General_structure) {
  loop {
    new_gs.x += 1;
    new_gs.current += 1;
    if new_gs.current > (new_gs.abc_array.len() as i32) {
      break;
    }
    munge_matrix(&mut new_gs);
  }
}

fn main() {
  let abc_array = &create_new_abc_array();
  let compare = comparitor([0, 0, 0], [1, 1, 1]);
  println!("COMPARE TEST : {:?}", compare);

  println!("\nRunning non-recursive version:");
  let iter_gs = General_structure {
    abc_array,
    x: 0,
    matrix: Vec::new(),
    current: 0,
  };
  matrix_builder(iter_gs);
  println!("\nRunning recursive version:");
  let mut recur_gs = General_structure {
    abc_array,
    x: 0,
    matrix: Vec::new(),
    current: 0,
  };
  recursize_matrix_builder(&mut recur_gs);
}