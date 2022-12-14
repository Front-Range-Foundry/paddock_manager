use crate::vertex::Vertex;
#[derive(Debug)]

pub struct Perimeter {
  pub vertices: Vec<Vertex>,
}

impl Perimeter {
  pub fn new(vertices: Vec<Vertex>) -> Perimeter {
    let perimeter = Perimeter {
      vertices,
    };
    perimeter
  }

  pub fn area(&self) -> f64 {
      let mut area = 0.0;
      let mut i = 0;
      let mut j = self.vertices.len() - 1;
      while i < self.vertices.len() {
        area += (self.vertices[j].x + self.vertices[i].x) * (self.vertices[j].y - self.vertices[i].y);
        j = i;
        i += 1;
      }
      area = area.abs() / 2.0;
      area
  }

  pub fn perimeter(&self) -> f64 {
	let mut perimeter = 0.0;
	let mut i = 0;
	let mut j = self.vertices.len() - 1;
	while i < self.vertices.len() {
	  perimeter += self.vertices[i].distance_from(&self.vertices[j]);
	  j = i;
	  i += 1;
	}
	perimeter
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_can_be_created() {
    let perimeter = Perimeter {
      vertices: Vec::new(),
    };
    assert_eq!(perimeter.vertices.len(), 0);
  }

  #[test]
  fn it_can_be_created_with_vertices() {
    let vertices = vec![
      Vertex::new(100.0, 100.0),
      Vertex::new(200.0, 100.0),
      Vertex::new(200.0, 200.0),
      Vertex::new(100.0, 200.0),
    ];
    let perimeter = Perimeter::new(vertices);
    assert_eq!(perimeter.vertices.len(), 4);
  }

  #[test]
  fn it_can_calculate_its_area_as_a_square() {
    let vertices = vec![
      Vertex::new(100.0, 100.0),
      Vertex::new(200.0, 100.0),
      Vertex::new(200.0, 200.0),
      Vertex::new(100.0, 200.0),
    ];
    let perimeter = Perimeter::new(vertices);
    assert_eq!(perimeter.area(), 10000.0);
  }

  #[test]
  fn it_can_calculate_its_area_as_a_rectangle() {
    let vertices = vec![
      Vertex::new(100.0, 100.0),
      Vertex::new(200.0, 100.0),
      Vertex::new(200.0, 300.0),
      Vertex::new(100.0, 300.0),
    ];
    let perimeter = Perimeter::new(vertices);
    assert_eq!(perimeter.area(), 20000.0);
  }

  #[test]
  fn it_can_calculate_its_area_as_a_triangle() {
    let vertices = vec![
      Vertex::new(100.0, 100.0),
      Vertex::new(200.0, 100.0),
      Vertex::new(200.0, 200.0),
    ];
    let perimeter = Perimeter::new(vertices);
    assert_eq!(perimeter.area(), 5000.0);
  }

  #[test] 
  fn it_can_calculate_its_area_as_a_polygon() {
    let vertices = vec![
      Vertex::new(100.0, 100.0),
      Vertex::new(200.0, 100.0),
      Vertex::new(200.0, 200.0),
      Vertex::new(150.0, 250.0),
      Vertex::new(100.0, 200.0),
    ];
    let perimeter = Perimeter::new(vertices);
    assert_eq!(perimeter.area(), 12500.0);
  }

  #[test]
  fn it_can_calculate_its_perimeter() {
	let vertices = vec![
	  Vertex::new(100.0, 100.0),
	  Vertex::new(200.0, 100.0),
	  Vertex::new(200.0, 200.0),
	  Vertex::new(100.0, 200.0),
	];
	let perimeter = Perimeter::new(vertices);
	assert_eq!(perimeter.perimeter(), 400.0);
  }

  #[test]
  fn it_can_calculate_its_perimeter_as_a_polygon() {
	let vertices = vec![
	  Vertex::new(100.0, 100.0),
	  Vertex::new(200.0, 100.0),
	  Vertex::new(200.0, 200.0),
	  Vertex::new(150.0, 250.0),
	  Vertex::new(100.0, 200.0),
    ];

	let perimeter = Perimeter::new(vertices);
	assert_eq!(perimeter.perimeter(), 441.4213562373095);
  }
}