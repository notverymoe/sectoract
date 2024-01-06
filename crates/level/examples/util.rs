// Copyright 2023 Natalie Baker // AGPLv3 //

#![allow(dead_code)]

use std::{fmt::Write, path::Path as FSPath};

use sectoract_level::map::Point2;
use svg::{Document, node::element::{path::Data, Path, Definitions, Marker, Group}};

pub fn face_to_obj_ngon(vert_count: usize, out: &mut String, face: &[[f32; 3]]) -> usize {

    for [x, y, z] in face {
        writeln!(out, "v {} {} {}", x, y, z).unwrap();
    }

    write!(out, "f ").unwrap();
    for i in 0..face.len() {
        write!(out, "{} ", vert_count + i + 1).unwrap();
    }
    writeln!(out).unwrap();

    vert_count + face.len()
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