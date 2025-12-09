use shared::Problem;

#[derive(Debug)]
struct Coord {
    x: u64,
    y: u64,
}

impl Coord {
    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn parse_input(contents: &str) -> Vec<Coord> {
    contents
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Coord { x, y }
        })
        .collect()
}

fn compute_1(contents: &str) -> u64 {
    let coords = parse_input(contents);
    let mut largest_rectangle_area = 0;

    for (i, coord_0) in coords.iter().enumerate() {
        for coord_1 in coords.iter().skip(i + 1) {
            let area = coord_0.area(&coord_1);
            if area > largest_rectangle_area {
                largest_rectangle_area = area;
            }
        }
    }

    largest_rectangle_area
}

fn compute_2(contents: &str) -> u64 {
    let coords = parse_input(contents);
    for (coord_0, coord_1) in coords.iter().zip(coords.iter().skip(1)) {
        if coord_0.x.abs_diff(coord_1.x) == 1 || coord_0.y.abs_diff(coord_1.y) == 1 {
            panic!("This solution assumes that there are no immediate U-Turns!");
        }
    }

    let mut largest_rectangle_area = 0;
    for (i, coord_0) in coords.iter().enumerate() {
        'coord_1_loop: for coord_1 in coords.iter().skip(i + 1) {
            // Check for any edge that intersects the INTERIOR of the rectangle defined
            // by coord_0 and coord_1. Any such intersection means that part of the
            // rectangle would not be in the interior of the polygon, and thus we can
            // ignore it. (We do so by `continue`ing the 'coord_1_loop for loop.)
            // We've already verified that there are no edges that are immediate U-Turns,
            // i.e. (0,0)->(5,0)->(5,1)->(0,1), which would be the only way this
            // approach would be invalid.
            for (coord_2, coord_3) in coords.iter().zip(coords.iter().cycle().skip(1)) {
                //                    1
                //       +-----------+
                //       |           |
                //       +-----------+
                //      0
                //
                //   Collapse down to x coordinate:
                //      0+-----------+1
                //   2+-------+3
                let min_x_0_1 = std::cmp::min(coord_0.x, coord_1.x);
                let min_x_2_3 = std::cmp::min(coord_2.x, coord_3.x);
                let max_x_0_1 = std::cmp::max(coord_0.x, coord_1.x);
                let max_x_2_3 = std::cmp::max(coord_2.x, coord_3.x);

                let min_y_0_1 = std::cmp::min(coord_0.y, coord_1.y);
                let min_y_2_3 = std::cmp::min(coord_2.y, coord_3.y);
                let max_y_0_1 = std::cmp::max(coord_0.y, coord_1.y);
                let max_y_2_3 = std::cmp::max(coord_2.y, coord_3.y);

                let x_coordinates_overlap_interior = min_x_2_3 < max_x_0_1 && max_x_2_3 > min_x_0_1;
                let y_coordinates_overlap_interior = min_y_2_3 < max_y_0_1 && max_y_2_3 > min_y_0_1;

                if x_coordinates_overlap_interior && y_coordinates_overlap_interior {
                    continue 'coord_1_loop;
                }
            }

            let area = coord_0.area(&coord_1);
            if area > largest_rectangle_area {
                largest_rectangle_area = area;
            }
        }
    }

    largest_rectangle_area
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    fn expected1(&self) -> String {
        "4758121828".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "1577956170".to_string()
    }
}
