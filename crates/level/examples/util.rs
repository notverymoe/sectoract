use sectoract_level::map::Sector;

use svg::{Document, node::element::{path::Data, Path}};

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

    document = document.set("viewBox", (min-2.0, min-2.0, max+2.0, max+2.0));
    svg::save(out, &document).unwrap();
}
