use std::error::Error;

#[derive(Default, Clone)]
pub struct PointCloud {
    pub points: Vec<Point>,
}

impl PointCloud {
    pub fn add_str(&mut self, val: &str) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = val.split(',').collect();

        if parts.len() == 3 {
            let mut p = Point::default();
            p.set_x(parts[0].parse()?);
            p.set_y(parts[1].parse()?);
            p.set_z(parts[2].parse()?);

            self.points.push(p);
        } else {
            println!("String does not contain exactly 3 comma-separated values");
        }

        Ok(())
    }
}

#[derive(Hash, Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Point(pub i64, pub i64, pub i64);

impl Point {
    pub fn set_x(&mut self, val: i64) {
        self.0 = val;
    }
    pub fn set_y(&mut self, val: i64) {
        self.1 = val;
    }
    pub fn set_z(&mut self, val: i64) {
        self.2 = val;
    }

    pub fn distance2(&self, other: &Point) -> i64 {
        (other.0 - self.0).pow(2) + (other.1 - self.1).pow(2) + (other.2 - self.2).pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_cloud() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let mut p = PointCloud::default();
        for line in input.lines() {
            p.add_str(line).unwrap();
        }

        assert!(p.points.contains(&Point(984, 92, 344)));

        assert_eq!(p.points.len(), 20);
    }

    #[test]
    fn distance() {
        let distance2 = Point(162, 817, 812).distance2(&Point(425, 690, 689));
        assert_eq!(distance2, 100427);
    }
}
