use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
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

fn brute_force(points: &[Point]) -> Option<Solution> {
    if points.len() < 3 {
        return None;
    }

    let n = points.len();
    let mut best = Solution::new(points[0], points[1], points[2]);

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let current =
                    Solution::new(points[i], points[j], points[k]);

                if current.twice_area > best.twice_area {
                    best = current;
                }
            }
        }
    }

    if best.twice_area == 0 {
        None
    } else {
        Some(best)
    }
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

    let algorithm_start = Instant::now();

    let solution = brute_force(&points).ok_or_else(|| {
        invalid(
            "Los postes no permiten formar un triángulo \
             de área positiva."
        )
    })?;

    let algorithm_time = algorithm_start.elapsed();

    write_solution(solution)?;

    let total_time = total_start.elapsed();

    println!("MÉTODO: FUERZA BRUTA");
    println!("Postes leídos: {}", points.len());
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