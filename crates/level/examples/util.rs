use std::fmt::Write;

use sectoract_level::map::{Sector, SectorPoint2, SectionSlope, IdentifierPoint};

use svg::{Document, node::element::{path::Data, Path}};

pub fn sector_to_obj(sector: &Sector, dest: &str) {

    let mut vertex_count = 0;
    let mut out = String::new();

    for section in sector.sections.iter() {
        if let Some([lower, upper]) = &section.surfaces {
            let lower_pnts: Vec<_> = get_surface_from_section(true,  &sector.points, &section.edges, lower);
            let upper_pnts: Vec<_> = get_surface_from_section(false, &sector.points, &section.edges, upper);

            vertex_count = face_to_ngon(vertex_count, &mut out, &lower_pnts);
            vertex_count = face_to_ngon(vertex_count, &mut out, &upper_pnts);
        }
    }

    std::fs::write(dest, out).unwrap();

}

fn face_to_ngon(vert_count: usize, out: &mut String, face: &[[f32; 3]]) -> usize {

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

fn get_surface_from_section(flip: bool, points: &[SectorPoint2], edges: &[IdentifierPoint], slope: &SectionSlope) -> Vec<[f32; 3]> {
    let map_fn = |(j, &i)| points[usize::from(i)].extend(slope.slice_height_for_point(j)).to_world();
    if flip {
        edges.iter().enumerate().map(map_fn).collect()
    } else {
        edges.iter().rev().enumerate().map(map_fn).collect()
    }
}

pub fn sector_to_svg(sector: &Sector, out: &str) {
    polys_to_svg(
        &sector.sections.iter().map(|section| section.edges.iter().map(|&i| sector.points[usize::from(i)].to_world()).collect()).collect(),
        out
    );
}

pub fn polys_to_svg(polygons: &Vec<Vec<[f32; 2]>>, out: &str) {
    
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut document = Document::new();
    for polygon in polygons {
        
        let mut data = Data::new();
        for (i, &[x, y]) in polygon.iter().enumerate() {
            min = min.min(x).min(y);
            max = max.max(x).max(y);
            if i==0 {
                data = data.move_to((x, y))
            } else {
                data = data.line_to((x, y));
            }
        }
        data = data.close();

        document = document.add(Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.1)
            .set("d", data)
        );

    }

    let border = (max - min)*0.05;
    document = document.set("viewBox", (min-border, min-border, max+border, max+border));
    svg::save(out, &document).unwrap();
}
