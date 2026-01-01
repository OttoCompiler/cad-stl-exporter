use std::fs::File;
use std::io::Write;


#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}


#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: [Vec3; 3],
}


pub struct Part {
    pub name: String,
    pub faces: Vec<Face>,
}

impl Part {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            faces: Vec::new(),
        }
    }


    pub fn create_box(name: &str, width: f64, height: f64, depth: f64) -> Self {
        let mut part = Part::new(name);
        let v = [
            Vec3::new(0.0, 0.0, 0.0), Vec3::new(width, 0.0, 0.0),
            Vec3::new(width, height, 0.0), Vec3::new(0.0, height, 0.0),
            Vec3::new(0.0, 0.0, depth), Vec3::new(width, 0.0, depth),
            Vec3::new(width, height, depth), Vec3::new(0.0, height, depth),
        ];

        let mut add_quad = |i1: usize, i2: usize, i3: usize, i4: usize| {
            part.faces.push(Face { vertices: [v[i1], v[i2], v[i3]] });
            part.faces.push(Face { vertices: [v[i1], v[i3], v[i4]] });
        };

        // define cube
        add_quad(0, 1, 2, 3);
        add_quad(4, 5, 6, 7);
        add_quad(0, 1, 5, 4);
        add_quad(1, 2, 6, 5);
        add_quad(2, 3, 7, 6);
        add_quad(3, 0, 4, 7);

        part
    }

    pub fn translate(&mut self, offset: Vec3) {
        for face in &mut self.faces {
            for vertex in &mut face.vertices {
                *vertex = vertex.add(offset);
            }
        }
    }

    // export stl
    pub fn export_stl(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "solid {}", self.name)?;

        for face in &self.faces {
            writeln!(file, "  facet normal 0 0 0")?;
            writeln!(file, "    outer loop")?;
            for v in &face.vertices {
                writeln!(file, "      vertex {} {} {}", v.x, v.y, v.z)?;
            }
            writeln!(file, "    endloop")?;
            writeln!(file, "  endfacet")?;
        }

        writeln!(file, "endsolid {}", self.name)?;
        Ok(())
    }
}

fn main() {
    println!("Initializing Rust CAD Library...");
    let mut bracket = Part::create_box("AngleBracket", 50.0, 10.0, 50.0);
    bracket.translate(Vec3::new(10.0, 0.0, 10.0));
    match bracket.export_stl("output_part.stl") {
        Ok(_) => println!("Successfully exported output_part.stl"),
        Err(e) => eprintln!("Export failed: {}", e),
    }
}