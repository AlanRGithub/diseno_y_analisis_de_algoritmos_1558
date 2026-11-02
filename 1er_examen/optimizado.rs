use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

const BASE_LIMIT: usize = 96;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
struct Solution {
    points: [Point; 3],
    twice_area: i64,
}

impl Solution {
    fn new(a: Point, b: Point, c: Point) -> Self {
        Self {
            points: [a, b, c],
            twice_area: cross(a, b, c).abs(),
        }
    }
}

#[derive(Clone, Copy)]
struct Marker {
    index: usize,
    family: usize,
    label: usize,
}

fn cross(a: Point, b: Point, c: Point) -> i64 {
    (b.x - a.x) * (c.y - a.y)
        - (b.y - a.y) * (c.x - a.x)
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn read_points() -> Result<Vec<Point>, Box<dyn Error>> {
    let reader = BufReader::new(File::open("campo.in")?);
    let mut points = Vec::new();
    let mut found_end = false;

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let values = line
            .split_whitespace()
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()?;

        if values.len() != 2 {
            return Err(
                invalid("Cada línea debe contener dos enteros.").into()
            );
        }

        let x = values[0];
        let y = values[1];

        if x == -1 && y == -1 {
            found_end = true;
            break;
        }

        if !(0..1000).contains(&x) || !(0..1000).contains(&y) {
            return Err(
                invalid("Las coordenadas deben estar entre 0 y 999.")
                    .into()
            );
        }

        points.push(Point { x, y });

        if points.len() > 500 {
            return Err(
                invalid("El archivo contiene más de 500 postes.").into()
            );
        }
    }

    if !found_end {
        return Err(invalid("Falta el terminador -1 -1.").into());
    }

    if points.len() < 3 {
        return Err(
            invalid("Se necesitan al menos tres postes.").into()
        );
    }

    Ok(points)
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_unstable();
    points.dedup();

    if points.len() <= 2 {
        return points;
    }

    let mut lower: Vec<Point> = Vec::with_capacity(points.len());

    for &point in &points {
        while lower.len() >= 2 {
            let size = lower.len();

            if cross(lower[size - 2], lower[size - 1], point) > 0 {
                break;
            }

            lower.pop();
        }

        lower.push(point);
    }

    let mut upper: Vec<Point> = Vec::with_capacity(points.len());

    for &point in points.iter().rev() {
        while upper.len() >= 2 {
            let size = upper.len();

            if cross(upper[size - 2], upper[size - 1], point) > 0 {
                break;
            }

            upper.pop();
        }

        upper.push(point);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);

    lower
}

fn rooted_maximum(
    polygon: &[Point],
    root: usize,
) -> ([usize; 3], Solution) {
    let n = polygon.len();

    let mut best_indices = [
        root,
        (root + 1) % n,
        (root + 2) % n,
    ];

    let mut best = Solution::new(
        polygon[best_indices[0]],
        polygon[best_indices[1]],
        polygon[best_indices[2]],
    );

    let mut third = 2;

    for second in 1..n - 1 {
        third = third.max(second + 1);

        let b = (root + second) % n;

        while third + 1 < n {
            let c = (root + third) % n;
            let next_c = (root + third + 1) % n;

            let current_area =
                cross(polygon[root], polygon[b], polygon[c]);

            let next_area =
                cross(polygon[root], polygon[b], polygon[next_c]);

            if next_area < current_area {
                break;
            }

            third += 1;
        }

        let c = (root + third) % n;

        let current =
            Solution::new(polygon[root], polygon[b], polygon[c]);

        if current.twice_area > best.twice_area {
            best = current;
            best_indices = [root, b, c];
        }
    }

    best_indices.sort_unstable();
    (best_indices, best)
}

fn quadratic_maximum(polygon: &[Point]) -> Solution {
    let (_, mut best) = rooted_maximum(polygon, 0);

    for root in 1..polygon.len() {
        let (_, current) = rooted_maximum(polygon, root);

        if current.twice_area > best.twice_area {
            best = current;
        }
    }

    best
}

fn previous_label(
    markers: &[Marker],
    edge: usize,
    family: usize,
) -> usize {
    for back in 0..markers.len() {
        let position =
            (edge + markers.len() - back) % markers.len();

        if markers[position].family == family {
            return markers[position].label;
        }
    }

    unreachable!("Cada familia contiene tres marcadores.");
}

fn compatible_subpolygons(
    polygon: &[Point],
    first: [usize; 3],
    second: [usize; 3],
) -> Vec<Vec<Point>> {
    let n = polygon.len();

    let shared: Vec<usize> = first
        .iter()
        .copied()
        .filter(|index| second.contains(index))
        .collect();

    let mut children: Vec<Vec<Point>> = Vec::new();

    for ordering in 0..(1usize << shared.len()) {
        let mut markers = Vec::with_capacity(6);

        for (label, &index) in first.iter().enumerate() {
            markers.push(Marker {
                index,
                family: 0,
                label,
            });
        }

        for (label, &index) in second.iter().enumerate() {
            markers.push(Marker {
                index,
                family: 1,
                label,
            });
        }

        markers.sort_unstable_by_key(|marker| {
            let reverse = shared
                .iter()
                .position(|&index| index == marker.index)
                .map(|bit| ordering & (1usize << bit) != 0)
                .unwrap_or(false);

            let family_order = if reverse {
                1 - marker.family
            } else {
                marker.family
            };

            (marker.index, family_order)
        });

        let mut first_labels = [0usize; 6];
        let mut second_labels = [0usize; 6];

        for edge in 0..6 {
            first_labels[edge] =
                previous_label(&markers, edge, 0);

            second_labels[edge] =
                previous_label(&markers, edge, 1);
        }

        for e0 in 0..4 {
            for e1 in e0 + 1..5 {
                for e2 in e1 + 1..6 {
                    let edges = [e0, e1, e2];

                    let mut first_mask = 0usize;
                    let mut second_mask = 0usize;

                    for &edge in &edges {
                        first_mask |=
                            1usize << first_labels[edge];

                        second_mask |=
                            1usize << second_labels[edge];
                    }

                    if first_mask != 0b111
                        || second_mask != 0b111
                    {
                        continue;
                    }

                    let mut keep = vec![false; n];

                    for marker in &markers {
                        keep[marker.index] = true;
                    }

                    for &edge in &edges {
                        let mut index = markers[edge].index;
                        let end = markers[(edge + 1) % 6].index;

                        loop {
                            keep[index] = true;

                            if index == end {
                                break;
                            }

                            index = (index + 1) % n;
                        }
                    }

                    let child: Vec<Point> = polygon
                        .iter()
                        .enumerate()
                        .filter_map(|(index, &point)| {
                            if keep[index] {
                                Some(point)
                            } else {
                                None
                            }
                        })
                        .collect();

                    if !children.iter().any(|old| old == &child) {
                        children.push(child);
                    }
                }
            }
        }
    }

    children
}

fn divide_and_conquer(polygon: &[Point]) -> Solution {
    let n = polygon.len();

    if n <= BASE_LIMIT {
        return quadratic_maximum(polygon);
    }

    let (first, mut best) = rooted_maximum(polygon, 0);

    let mut longest_start = first[0];
    let mut longest_length = 0;

    for position in 0..3 {
        let start = first[position];
        let end = first[(position + 1) % 3];
        let length = (end + n - start) % n;

        if length > longest_length {
            longest_length = length;
            longest_start = start;
        }
    }

    let middle =
        (longest_start + longest_length / 2) % n;

    let (second, second_solution) =
        rooted_maximum(polygon, middle);

    if second_solution.twice_area > best.twice_area {
        best = second_solution;
    }

    let children =
        compatible_subpolygons(polygon, first, second);

    assert!(!children.is_empty() && children.len() <= 2);

    assert!(children.iter().all(|child| {
        child.len() >= 3 && child.len() < n
    }));

    assert!(
        children.iter().map(Vec::len).sum::<usize>() <= n + 12
    );

    for child in children {
        let current = divide_and_conquer(&child);

        if current.twice_area > best.twice_area {
            best = current;
        }
    }

    best
}

fn write_solution(solution: Solution) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create("campo.out")?);

    for point in solution.points {
        writeln!(writer, "{} {}", point.x, point.y)?;
    }

    writer.flush()
}

fn main() -> Result<(), Box<dyn Error>> {
    let total_start = Instant::now();

    let points = read_points()?;
    let point_count = points.len();

    let algorithm_start = Instant::now();

    let hull = convex_hull(points);
    let hull_count = hull.len();

    if hull_count < 3 {
        return Err(
            invalid(
                "Los postes no permiten formar un triángulo \
                 de área positiva."
            )
            .into()
        );
    }

    let solution = divide_and_conquer(&hull);

    let algorithm_time = algorithm_start.elapsed();

    write_solution(solution)?;

    let total_time = total_start.elapsed();

    println!("MÉTODO: ENVOLVENTE CONVEXA + DIVIDE Y VENCERÁS");
    println!("Postes leídos: {}", point_count);
    println!("Vértices de la envolvente: {}", hull_count);
    println!("Coordenadas del triángulo máximo:");

    for point in solution.points {
        println!("{} {}", point.x, point.y);
    }

    println!(
        "Área máxima: {:.1}",
        solution.twice_area as f64 / 2.0
    );

    println!(
        "Tiempo del algoritmo: {:.6} ms",
        algorithm_time.as_secs_f64() * 1000.0
    );

    println!(
        "Tiempo total (lectura + cálculo + escritura): {:.6} ms",
        total_time.as_secs_f64() * 1000.0
    );

    println!("Resultado guardado en campo.out");

    Ok(())
}