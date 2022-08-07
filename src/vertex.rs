
pub struct Vertex {
  x: f64,
  y: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64) -> Vertex {
      let vertex = Vertex {
          x,
          y,
      };
      vertex
    }
	pub fn distance_from(&self, other: &Vertex) -> f64 {
		let x = self.x - other.x;
		let y = self.y - other.y;
		let distance = (x.powf(2.0) + y.powf(2.0)).sqrt();
		distance
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_be_created() {
        let vertex = Vertex::new( 100.0, 100.0 );
        assert_eq!(vertex.x, 100.0);
        assert_eq!(vertex.y, 100.0);
    }

	#[test]
    fn y_dist_to_another_vertex() {
        let vertex = Vertex::new( 100.0, 100.0 );
        let another_vertex = Vertex::new( 100.0, 200.0 );
        assert_eq!(vertex.distance_from(&another_vertex), 100.0);
    }

	#[test] 
	fn x_dist_to_another_vertex() {
		let vertex = Vertex::new(100.0, 100.0);
		let another_vertex = Vertex::new(200.0, 100.0);
		assert_eq!(vertex.distance_from(&another_vertex), 100.0);
	}

	#[test]
	fn dist_to_another_vertex() {
		let vertex = Vertex::new(100.0, 100.0);
		let another_vertex = Vertex::new(200.0, 200.0);
		assert_eq!(vertex.distance_from(&another_vertex), 141.4213562373095);
	}
}