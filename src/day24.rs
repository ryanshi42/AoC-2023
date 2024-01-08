use anyhow::{anyhow, Context as _};
use itertools::Itertools as _;
use nalgebra::{matrix, vector, Matrix, Matrix2, Matrix4, RowVector4, Vector, Vector3};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete,
    error::context,
    multi::separated_list1,
    IResult,
};

type ParseResult<'a, T> = IResult<&'a str, T, nom::error::VerboseError<&'a str>>;

pub fn parse_all(input: &str) -> ParseResult<Vec<Hailstone>> {
    separated_list1(newline, hailstone)(input)
}

fn newline(input: &str) -> ParseResult<&str> {
    context("newline", alt((tag("\n"), tag("\r\n"))))(input)
}

fn whitespace(input: &str) -> ParseResult<&str> {
    context("whitespace", take_while(|c| " \t".contains(c)))(input)
}

fn hailstone(input: &str) -> ParseResult<Hailstone> {
    let (input, xyz) = point(input)?;
    let (input, _) = whitespace(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, v) = point(input)?;

    Ok((
        input,
        Hailstone {
            position: xyz,
            velocity: v,
        },
    ))
}

fn point(input: &str) -> ParseResult<Vector3<f64>> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = whitespace(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, vector![x as f64, y as f64, z as f64]))
}

// https://gitlab.com/silmeth/advent-of-code-2023/-/blob/main/day-24/src/lib.rs

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

pub fn parse(input: &str) -> anyhow::Result<Vec<Hailstone>> {
    let (_, stones) = parse_all(input)
        .map_err(|e| anyhow!("{}", e.map(|e| nom::error::convert_error(input, e))))?;
    Ok(stones)
}

fn solve_linear_system<T, D, S1, S2>(
    mut a: Matrix<T, D, D, S1>,
    mut b: Vector<T, D, S2>,
) -> Option<Vector<T, D, S2>>
where
    T: num::Num
        + num::traits::NumAssign
        + num::traits::Signed
        + PartialOrd
        + Clone
        + nalgebra::Scalar,
    D: nalgebra::Dim,
    S1: nalgebra::RawStorageMut<T, D, D> + std::fmt::Debug,
    S2: nalgebra::RawStorageMut<T, D, nalgebra::U1>,
{
    assert!(a.nrows() == a.ncols());
    assert!(a.nrows() == b.nrows());
    let dim = a.nrows();

    // Guassian elimination with back-substitution
    for i in 0..dim {
        let rows = a.index((i.., i));
        let (pivot_row, _) = rows
            .iter()
            .enumerate()
            // i-based indexing
            .map(|(j, v)| (j + i, v))
            //there’s no .max() for PartialOrd, let’s do this manually
            .fold::<Option<_>, _>(None, |prev, (j, v)| {
                if let Some((j_acc, prev)) = prev {
                    let v = num::abs(v.clone());

                    if v.partial_cmp(&prev)? == std::cmp::Ordering::Greater {
                        Some((j, v))
                    } else {
                        Some((j_acc, prev))
                    }
                } else {
                    Some((j, v.clone()))
                }
            })?;

        let pivot_el = a[(pivot_row, i)].clone();

        if pivot_el == T::zero() {
            return None;
        }

        if pivot_row != i {
            a.swap_rows(pivot_row, i);
            b.swap_rows(pivot_row, i);
        }

        // divide the row by pivot
        if i < dim - 1 {
            for el in a.index_mut((i, i + 1..)).iter_mut() {
                *el /= pivot_el.clone();
            }
        }
        *b.index_mut(i) /= pivot_el.clone();

        // subtract from the rows below
        for j in i + 1..dim {
            let factor = a[(j, i)].clone();
            for k in i + 1..dim {
                let correction = factor.clone() * a[(i, k)].clone();
                a[(j, k)] -= correction;
            }
            let correction = factor.clone() * b[i].clone();
            b[j] -= correction;
        }
    }

    // back-substitution
    for i in (0..dim - 1).rev() {
        for j in i + 1..dim {
            let c = b[j].clone() * a[(i, j)].clone();
            b[i] -= c;
        }
    }

    Some(b)
}

#[aoc(day24, part1)] 
pub fn part1(input: &str) -> u64 {
    part1_solver(&parse(input).unwrap(), (200000000000000, 400000000000000))
}

pub fn part1_solver(hailstones: &[Hailstone], test_area: (i64, i64)) -> u64 {
    let test_area = (test_area.0 as f64, test_area.1 as f64);
    hailstones
        .iter()
        .combinations(2)
        // For every hailstone the position is expressed by:
        //   x(t) = x(0) + v_x * t
        //   y(t) = y(0) + v_y * t.
        //
        // For a pair of hailstones their trajectories intersect at (x, y) if there is a solution
        // for equations of the type:
        //   x_1(t_1) = x_2(t_2)
        //   y_1(t_1) = y_2(t_2),
        // that is:
        //   x_1(0) + v_x1 * t_1 = x_2(0) + v_x2 * t_2
        //   y_1(0) + v_y1 * t_1 = y_2(0) + v_y2 * t_2.
        //
        // The first eq. is equivalent to (t-components on the left, free components on the right):
        //   v_x1 * t_1 - v_x2 * t_2 = x_2(0) - x_1(0),
        // the second can be transformed analogically.
        //
        // Then if we define a 2x2 matrix A:
        //   A = [ v_x1, -v_x2; v_y1, -v_y2 ]
        // and vectors T and X:
        //   T = [ t_1; t_2],
        //   X = [ x_02 - x_01; y_02 - y_01 ],
        // those equations can be expressed in a matrix form as:
        //   A · T = X
        // which can be multiplied by A^(-1) on the left on both sides to solve for T:
        //   T = A^(-1) · X.
        //
        // From T (which is times of the intersection for both hailstones) we easily get the
        // answer.
        .filter(|stones| {
            let stone1 = stones[0];
            let stone2 = stones[1];

            let mat_a = Matrix2::from_columns(&[stone1.velocity.xy(), -stone2.velocity.xy()]);

            let vec_x = stone2.position.xy() - stone1.position.xy();
            // equivalent to: let vec_t = invert(mat_a) * vec_x;
            let Some(vec_t) = solve_linear_system(mat_a, vec_x) else {
                return false;
            };

            if vec_t.x < 0.0 || vec_t.y < 0.0 {
                // intersection in the past
                return false;
            }

            // find the crossing point
            let cross = stone1.position + stone1.velocity * vec_t.x;

            // check if the crossing point is in the test area
            cross.x >= test_area.0
                && cross.y >= test_area.0
                && cross.x <= test_area.1
                && cross.y <= test_area.1
        })
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day24, part2)] 
pub fn part2(input: &str) -> u64 {
    part2_solver(&parse(input).unwrap()).unwrap()
}

// Heavily inspired by ash30342’s solution:
// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kf4jv41/?context=3
pub fn part2_solver(hailstones: &[Hailstone]) -> anyhow::Result<u64> {
    // To solve part 2 we need at least 3 hailstones (then we get a system of 9 non-linear
    // equations with 9 unknowns).
    //
    // If P is a position vector (x, y, z) and V is velocity (v_x, v_y, v_z), then for every
    // hailstone n we have a vector equation of the form which consists of three equations, each
    // for every coordinate:
    //
    //   P_n + (V_n - V_s) · t_n = P_s
    //
    // we need to take hailstones n ∈ [0, 3) – thus having 9 non-linear equations together and solve
    // them for P_s which is the solution position (while V_s – the velocity; and t_n – the times
    // of collision with n-th hailstone, are other unknowns).
    //
    // But if we introduce some redundancy by considering 5 hailstones instead,
    // we can reduce the system of equations to a linear one which is much easier to solve.
    //
    // So let’s assume that we actually have *at least* 5 hailstones in the input.

    if hailstones.len() < 5 {
        return Err(anyhow!(concat!(
            "At least 5 hailstones are required for an easy linear solution! ",
            "If you have three, go use Maxima, or Matlab, or sth else!"
        )));
    }

    let (h0, h1, h2, h3, h4) = (
        hailstones[0],
        hailstones[1],
        hailstones[2],
        hailstones[3],
        hailstones[4],
    );

    // Now, to find the linear equations that will help us solve the problem, let’s consider only
    // the x and y axes for a moment.
    //
    // For the n-th stone we can write the equations (the subscript _s means ‘solution’):
    //   x_n + v_xn · t_n = x_s + v_xs · t_n
    //   y_n + v_yn · t_n = y_s + v_ys · t_n
    // from which follows that the time t_n is:
    //   t_n = (x_s - x_1) / (v_x1 - v_xs)
    //       = (y_s - y_1) / (v_y1 - v_ys)
    // and if we multiply both sides by (v_(xy)1 - v(xy)s) and then multiply all the elements we
    // get:
    //
    //   x_s v_ys - y_s v_xs = x_s v_yn - y_s v_xn + x_n v_ys - y_n v_xs - x_n v_yn + y_n v_xn
    //
    // where the left-hand side (x_s v_ys - y_s v_xs) does not depend on the hailstone – it’s
    // constant. Thus we can now take the right side for n = 0 and n = 1 and set them equal…
    //
    // This gives us a linear equation of the form:
    //
    //   x_s (v_y1 - v_y0) + y_s (v_x0 - v_x1) + v_xs (y_0 - y_1) + v_ys (x_1 - x_0) =
    //     x_1 v_y1 - x_0 v_y0 + y_0 v_x0 - y_1 v_x1
    //
    // it has four unknowns (x_s, y_s, v_xs, v_ys) and so we need 4 equations of this type. So we
    // just pick 4 pairs of hailstones to do that.

    let make_row = |stone0: Hailstone, stone1: Hailstone| {
        RowVector4::new(
            stone1.velocity.y - stone0.velocity.y,
            stone0.velocity.x - stone1.velocity.x,
            stone0.position.y - stone1.position.y,
            stone1.position.x - stone0.position.x,
        )
    };
    let make_rh = |stone0: Hailstone, stone1: Hailstone| {
        stone1.position.x * stone1.velocity.y - stone0.position.x * stone0.velocity.y
            + stone0.position.y * stone0.velocity.x
            - stone1.position.y * stone1.velocity.x
    };

    let lh_coefficients = Matrix4::from_rows(&[
        make_row(h0, h1),
        make_row(h1, h2),
        make_row(h2, h3),
        make_row(h3, h4),
    ]);

    let rh = vector![
        make_rh(h0, h1),
        make_rh(h1, h2),
        make_rh(h2, h3),
        make_rh(h3, h4)
    ];

    // Solving these equations gives us (x_s, y_s, v_xs, v_ys):
    // equivalent to: let xy = invert(lh_coefficients) * rh;
    let xy = solve_linear_system(lh_coefficients, rh).context("singular matrix")?;
    let (x_s, y_s, v_xs) = (xy.x, xy.y, xy[2]);

    // What’s left is calculating z_s. Let’s just compute t_0, t_1, and then use:
    //   z_s + v_zs · t_n = z_n + v_zn · t_n.

    let t0 = (x_s - h0.position.x) / (h0.velocity.x - v_xs);
    let t1 = (x_s - h1.position.x) / (h1.velocity.x - v_xs);

    let z_eq_lh = matrix![1., t0; 1., t1];
    let z_eq_rh = vector![
        h0.position.z + h0.velocity.z * t0,
        h1.position.z + h1.velocity.z * t1,
    ];

    // equivalent to: let z_s = (invert(z_eq_lh) * z_eq_rh).x;
    let z_s = solve_linear_system(z_eq_lh, z_eq_rh)
        .context("singular matrix")?
        .x;

    println!("{:?}", x_s);
    println!("{:?}", y_s);
    println!("{:?}", z_s);
    Ok((x_s + y_s + z_s).round() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "19, 13, 30 @ -2,  1, -2\n",
        "18, 19, 22 @ -1, -1, -2\n",
        "20, 25, 34 @ -2, -2, -4\n",
        "12, 31, 28 @ -1, -2, -1\n",
        "20, 19, 15 @  1, -5, -3\n",
    );

    #[test]
    fn test_part1() {
        let hailstones = parse(INPUT).unwrap();
        let result = part1_solver(&hailstones, (7, 27));
        assert_eq!(2, result);
    }

    #[test]
    fn test_part2() {
        let hailstones = parse(INPUT).unwrap();
        let result = part2_solver(&hailstones).unwrap();
        assert_eq!(47, result);
    }
}
