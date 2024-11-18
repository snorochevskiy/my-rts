use crate::{terrain::{Plain, Ramp, Side, Tile}, util::MatrixHelper};


#[derive(Debug, PartialEq, Eq)]
enum TextCell {
    Plain(u32),
    Ramp(String),
}

fn parse_text_cells(text: &str) -> Result<Vec<Vec<TextCell>>,String> {
    let mut ter: Vec<Vec<TextCell>> = Vec::new();
    for (row_ind, line) in text.trim().lines().filter(|line| !line.is_empty()).enumerate() {
        let mut cell_row = Vec::new();
        for (col_ind, s) in line.trim().split(" ").into_iter().enumerate() {
            if ["-", "/", "\\", "|"].contains(&s) {
                cell_row.push(TextCell::Ramp(s.to_string()));
            } else if let Ok(level) = s.parse::<u32>() {
                cell_row.push(TextCell::Plain(level));
            } else {
                return Err(format!("Unexpected symbol at row={row_ind}, col={col_ind}: {s}"));
            }
        }
        ter.push(cell_row);
    }
    Ok(ter)
}

pub fn parse(text: &str) -> Result<Vec<Vec<Tile>>,String> {
    let ter: Vec<Vec<TextCell>> = parse_text_cells(text)?;

    let map_width = ter[0].len();
    let map_height = ter.len();

    for (row_ind, row) in ter.iter().enumerate() {
        if row.len() != map_width {
            return Err(format!("Unexpected width {} for row number {}", row.len(), row_ind));
        }
    }

    let mut result: Vec<Vec<Tile>> = Vec::new();
    for y in 0 .. map_height {
        let mut row_tiles = Vec::new();
        for x in 0 .. map_width {
            match &ter[y][x] {
                TextCell::Plain(level) => {
                    let mut cliffs = Vec::new();
                    if ter.cell(y as i32 - 1, x as i32) == Some(&TextCell::Plain(level - 1)) {
                        cliffs.push(Side::Top);
                    }
                    if ter.cell(y as i32 + 1, x as i32) == Some(&TextCell::Plain(level - 1)) {
                        cliffs.push(Side::Bottom);
                    }
                    if ter.cell(y as i32, x as i32 - 1) == Some(&TextCell::Plain(level - 1)) {
                        cliffs.push(Side::Left);
                    }
                    if ter.cell(y as i32, x as i32 +1 ) == Some(&TextCell::Plain(level - 1)) {
                        cliffs.push(Side::Right);
                    }
                    let tile: Tile = Tile::Plain(Plain { level: *level as f32, cliffs });
                    row_tiles.push(tile);
                },
                TextCell::Ramp(c) => {
                    match c.as_str() {
                        "/" => match (ter.cell(y as i32 + 1, x as i32 - 1), ter.cell(y as i32 - 1, x as i32 + 1)) {
                                (Some(&TextCell::Plain(bottom_left)), Some(&TextCell::Plain(top_right))) => {
                                    if bottom_left < top_right {
                                        row_tiles.push(Tile::Ramp(Ramp { bottom_level: bottom_left as f32, top_level: top_right as f32, bottom_side: Side::BottomLeft}));
                                    } else if bottom_left > top_right {
                                        row_tiles.push(Tile::Ramp(Ramp { bottom_level: top_right as f32, top_level: bottom_left as f32, bottom_side: Side::TopRight}));
                                    } else {
                                        return Err(format!("Unexpected neighboring levels for / ramp at ({y},{x})"))
                                    }
                                },
                                _ => return Err(format!("Unexpected location for / ramp at ({y},{x})")),
                            },
                        "\\" => match (ter.cell(y as i32 - 1, x as i32 - 1), ter.cell(y as i32 + 1, x as i32 + 1)) {
                            (Some(&TextCell::Plain(top_left)), Some(&TextCell::Plain(bottom_right))) => {
                                if top_left < bottom_right {
                                    row_tiles.push(Tile::Ramp(Ramp { bottom_level: top_left as f32, top_level: bottom_right as f32, bottom_side: Side::TopLeft }));
                                } else if top_left > bottom_right {
                                    row_tiles.push(Tile::Ramp(Ramp { bottom_level: bottom_right as f32, top_level: top_left as f32, bottom_side: Side::BottomRight }));
                                } else {
                                    return Err(format!("Unexpected neighboring levels for \\ ramp at ({y},{x})"))
                                }
                            },
                            _ => return Err(format!("Unexpected location for \\ ramp at ({y},{x})")),
                        },
                        "-" => match (ter.cell(y as i32, x as i32 - 1), ter.cell(y as i32, x as i32 + 1)) {
                            (Some(&TextCell::Plain(left)), Some(&TextCell::Plain(right))) => {
                                if left < right {
                                    row_tiles.push(Tile::Ramp(Ramp { bottom_level: left as f32, top_level: right as f32, bottom_side: Side::Left }));
                                } else if left > right {
                                    row_tiles.push(Tile::Ramp(Ramp { bottom_level: right as f32, top_level: left as f32, bottom_side: Side::Right }));
                                } else {
                                    return Err(format!("Unexpected neighboring levels for - ramp at ({y},{x}): left={left}, right={right}"))
                                }
                            },
                            _ => return Err(format!("Unexpected location for - ramp at ({y},{x})")),
                        },
                        "|" => match (ter.cell(y as i32 - 1, x as i32), ter.cell(y as i32 + 1, x as i32)) {
                            (Some(&TextCell::Plain(top)), Some(&TextCell::Plain(bottom))) => {
                                if top < bottom {
                                    row_tiles.push(Tile::Ramp(Ramp { bottom_level: top as f32, top_level: bottom as f32, bottom_side: Side::Top}));
                                } else if top > bottom {
                                    row_tiles.push(Tile::Ramp(Ramp { bottom_level: bottom as f32, top_level: top as f32, bottom_side: Side::Bottom}));
                                } else {
                                    return Err(format!("Unexpected neighboring levels for | ramp at ({y},{x})"))
                                }
                            },
                            _ => return Err(format!("Unexpected location for ramp at ({y},{x})")),
                        },
                        _ => return Err(format!("Unexpected symbol: {c}")),
                    }
                },
            }
        }
        result.push(row_tiles);
    }

    Ok(result)
}


#[test]
fn test_parse_text_cells() {
    let terrain: &'static str = r#"
        1 1 1 1 1 1
        1 \ | 1 1 1
        1 - 2 2 1 1
        1 1 2 2 1 1
        1 1 1 1 1 1
        1 1 1 1 1 1
    "#;

    let cells = parse_text_cells(terrain).unwrap();

    let ramp = |s: &str| TextCell::Ramp(s.to_string());

    use TextCell::*;
    assert_eq!(cells,
        vec![
            vec![Plain(1), Plain(1), Plain(1), Plain(1), Plain(1), Plain(1)],
            vec![Plain(1), ramp("\\"), ramp("|"), Plain(1), Plain(1), Plain(1)],
            vec![Plain(1), ramp("-"), Plain(2), Plain(2), Plain(1), Plain(1)],
            vec![Plain(1), Plain(1), Plain(2), Plain(2), Plain(1), Plain(1)],
            vec![Plain(1), Plain(1), Plain(1), Plain(1), Plain(1), Plain(1)],
            vec![Plain(1), Plain(1), Plain(1), Plain(1), Plain(1), Plain(1)],
        ]
    );
}

#[test]
fn test_parse() {
    let terrain: &'static str = r#"
        1 1 1 1 1 1
        1 \ | 1 1 1
        1 - 2 2 1 1
        1 1 2 2 1 1
        1 1 1 1 1 1
        1 1 1 1 1 1
    "#;

    let result = parse(terrain).unwrap();

    println!("{result:?}");
}