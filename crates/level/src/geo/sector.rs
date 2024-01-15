// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::IdentifierEdgeHalf;

pub fn extract_sector_contour(bounds: &mut Vec<IdentifierEdgeHalf>, start: IdentifierEdgeHalf, keys: &[IdentifierEdgeHalf]) {
    bounds.push(start);
    loop {
        let test_key = bounds[bounds.len() - 1];
        if let Some(next_key) = keys.iter().map(|v| v.with_reverse()).find(|&v| 
                start    != v &&                     // Exclude the start key
                test_key != v &&                     // Exclude the key we're looking for
                test_key.connects_to(v) &&           // Check that we have connectivity
                !keys.contains(&v) &&                // Check the graph doesn't contain the key (is orphan edge)
                (start == v || !bounds.contains(&v)) // Check that we haven't already added it, unless it's the first key
            ) {

            bounds.push(next_key);

            // We found the end!
            // - A polygon must have at least 3 points
            // - Only two edges should connect to the first point
            if bounds.len() >= 3 && next_key.connects_to(start) {
                break;
            }
        } else {
            unreachable!("Incomplete or duplicate sector boundry detected. This shouldn't be possible.");
        }
    }
}