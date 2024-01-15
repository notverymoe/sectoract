// Copyright 2023 Natalie Baker // AGPLv3 //

#![allow(dead_code)]

use std::{fmt::Write, path::Path as FSPath};

use sectoract_level::{map::{Point2, Point3, IdentifierEdgeHalf}, geo::FaceWriter};
use svg::{Document, node::element::{path::Data, Path, Definitions, Marker, Group}};

#[derive(Debug, Default, Clone)]
pub struct ObjFaceWriter {
    output: String,
    vert_count: usize,
}

impl ObjFaceWriter {
    
    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn vert_count(&self) -> usize {
        self.vert_count
    }

}

impl FaceWriter for ObjFaceWriter {
    fn add_surf(&mut self, _part: usize, face: impl IntoIterator<Item = Point3>) {
        self.vert_count = append_ngon_to_obj_str(&mut self.output, self.vert_count, face.into_iter());
    }

    fn add_wall(&mut self, _part: usize, _edge: IdentifierEdgeHalf, face: impl IntoIterator<Item = Point3>) {
        self.vert_count = append_ngon_to_obj_str(&mut self.output, self.vert_count, face.into_iter());
    }
}

pub fn append_ngon_to_obj_str(out: &mut String, vert_count: usize, face: impl Iterator<Item = Point3>) -> usize {
    let mut len = 0;
    for point in face {
        let [x, y, z] = point.to_world();
        writeln!(out, "v {} {} {}", x, y, z).unwrap();
        len += 1;
    }

    write!(out, "f ").unwrap();
    for i in 0..len {
        write!(out, "{} ", vert_count + i + 1).unwrap();
    }
    writeln!(out).unwrap();

    vert_count + len
}

pub fn polys_to_svg<'a>(polygons: impl Iterator<Item = &'a [Point2]>, dest: &str) {
    
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut document = Document::new();
    

    document = document.add(Definitions::new()
        .add(Marker::new()
            .set("id", "triangle")
            .set("viewBox", (0, 0, 10, 10))
            .set("refX", 3.5)
            .set("refY", 2.5)
            .set("markerUnits", "strokeWidth")
            .set("markerWidth",  5)
            .set("markerHeight", 5)
            .set("orient",   "auto")
            .add(Path::new()
                .set("d", Data::new()
                    .move_to((0.0, 0.0))
                    .line_to((5.0, 2.5))
                    .line_to((0.0, 5.0))
                    .close()
                )
                .set("fill", "black")
            )
        )
    );

    let mut group = Group::new()
        .set("transform", "scale(-1 1)");

    for polygon in polygons {
        
        let mut data = Data::new();
        for (i, point) in polygon.iter().enumerate() {
            let [x, y] = point.to_world();
            min = min.min(x).min(y);
            max = max.max(x).max(y);
            if i==0 {
                data = data.move_to((x, y))
            } else {
                data = data.line_to((x, y));
            }
        }

        if polygon.len() > 2 {
            data = data.close();
        }

        group = group.add(Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.1)
            .set("marker-end", "url(#triangle)")
            .set("d", data)
        );

    }

    let size   = max-min;
    let border = size*0.05;
    document = document.set("viewBox", (
        -max-border,     min-border, 
        size+border*2.0, size+border*2.0
    ));

    document = document.add(group);

    std::fs::create_dir_all(FSPath::new(dest).parent().unwrap()).unwrap();
    svg::save(dest, &document).unwrap();
}

fn main() {}