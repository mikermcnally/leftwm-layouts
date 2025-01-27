use super::Rect;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents the four different direction where we can search for a neighbor
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Direction {
    #[default]
    /// Search for neighbor starting from the top left of the current rect
    /// This is the default value.
    ///
    /// ```txt
    ///    North
    ///
    /// +---------+
    /// | ^       |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    North,

    /// Search for neighbor starting from the right top of the current rect
    ///
    /// ```txt
    ///    East
    ///
    /// +---------+
    /// |       > |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    East,

    /// Search for neighbor starting from the bottom left of the current rect
    ///
    /// ```txt
    ///    South
    /// +---------+
    /// |         |
    /// |         |
    /// | V       |
    /// +---------+
    ///
    /// ```
    South,

    /// Search for neighbor starting from the left top of the current rect
    ///
    /// ```txt
    ///     West
    ///
    /// +---------+
    /// | <       |
    /// |         |
    /// |         |
    /// +---------+
    /// ```
    West,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "North" => Ok(Direction::North),
            "South" => Ok(Direction::South),
            "East" => Ok(Direction::East),
            "West" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

// Find the north neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_north(rects: &[Rect], current: usize, container_top_edge: i32) -> Option<usize> {
    let current_rect = rects.get(current).or(None)?;

    // We are all the way up, no neighbor available
    if current_rect.top_edge() <= container_top_edge {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
        r.right_edge() - 1 < current_rect.left_edge() || // skip too right
        r.left_edge() + 1 > current_rect.right_edge() || // skip too left
        r.top_edge() + 1 > current_rect.bottom_edge()
        // skip too low
        {
            continue;
        }

        let x_distance = r.left_edge() - current_rect.right_edge();
        let y_distance = current_rect.top_edge() - r.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            true,
        );
    }

    nearest_rect
}

// Find the east neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_east(rects: &[Rect], current: usize, container_right_edge: i32) -> Option<usize> {
    let current_rect = rects.get(current).or(None)?;

    // We are all the way right, no neighbor available
    if current_rect.right_edge() >= container_right_edge {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
        r.right_edge() - 1 < current_rect.right_edge() || // skip too left
        r.bottom_edge() - 1 < current_rect.top_edge() || // skip too high
        r.top_edge() + 1 > current_rect.bottom_edge()
        // skip too low
        {
            continue;
        }

        let x_distance = r.left_edge() - current_rect.right_edge();
        let y_distance = r.top_edge() - current_rect.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            false,
        );
    }

    nearest_rect
}

// Find the south neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_south(rects: &[Rect], current: usize, container_bottom_edge: i32) -> Option<usize> {
    let current_rect = rects.get(current).or(None)?;

    // We are at the bottom, no neighbor available
    if current_rect.y + current_rect.h as i32 >= container_bottom_edge {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
        r.right_edge() - 1 < current_rect.left_edge() || // skip too left
        r.left_edge() + 1 > current_rect.right_edge() || // skip too right
        r.bottom_edge() - 1 < current_rect.top_edge()
        // skip too high
        {
            // skip current rect
            continue;
        }

        let x_distance = r.left_edge() - current_rect.right_edge();
        let y_distance = r.top_edge() - current_rect.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            true,
        );
    }

    nearest_rect
}

// Find the west neighbor starting from a given `Rect` with index `current` in an array of
// [`Rect`].
fn find_west(rects: &[Rect], current: usize, container_left_edge: i32) -> Option<usize> {
    let current_rect = rects.get(current).or(None)?;

    // We are all the way left; no neighbor available
    if current_rect.left_edge() <= container_left_edge {
        return None;
    }

    let mut nearest_rect: Option<usize> = None;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;

    for (i, r) in rects.iter().enumerate() {
        if r == current_rect || // skip current rect
         r.left_edge() + 1 > current_rect.right_edge() || // skip too right
         r.bottom_edge() - 1 < current_rect.top_edge() || // skip too high
         r.top_edge() + 1 > current_rect.bottom_edge()
        // skip too low
        {
            // skip current rect
            continue;
        }

        let x_distance = current_rect.left_edge() - r.right_edge();
        let y_distance = r.top_edge() - current_rect.bottom_edge();

        find_nearest_rect(
            &mut min_x,
            &mut min_y,
            &mut nearest_rect,
            x_distance,
            y_distance,
            i,
            false,
        );
    }

    nearest_rect
}

// Find the nearest `Rect`. If updown is true, evaluate y_distance and then x_distance. If updown
// is false, evaluate x_distance and then y_distance.
fn find_nearest_rect(
    min_x: &mut Option<i32>,
    min_y: &mut Option<i32>,
    nearest_rect: &mut Option<usize>,
    x_distance: i32,
    y_distance: i32,
    index: usize,
    updown: bool,
) {
    if min_x.is_none() {
        *min_x = Some(x_distance);
        *nearest_rect = Some(index);
    }

    if min_y.is_none() {
        *min_y = Some(y_distance);
        *nearest_rect = Some(index);
    }

    if updown {
        if y_distance < min_y.unwrap() {
            // take the nearest up/down
            *min_y = Some(y_distance);
            *nearest_rect = Some(index);
        } else if y_distance == min_y.unwrap() && x_distance < min_x.unwrap() {
            // take the left most
            *min_x = Some(x_distance);
            *nearest_rect = Some(index);
        }
    } else if x_distance < min_x.unwrap() {
        // take the nearest left/right
        *min_x = Some(x_distance);
        *nearest_rect = Some(index);
    } else if x_distance == min_x.unwrap() && y_distance < min_y.unwrap() {
        // take the higher
        *min_y = Some(y_distance);
        *nearest_rect = Some(index);
    }
}

impl Direction {
    /// Find the neighbor in a given direction (`North`, `East`, `South`, `West`) within the currently active container, starting from a
    /// given `Rect` identified by the index `current` in an array of [`Rect`]
    pub fn find_neighbor(
        rects: &[Rect],
        current: usize,
        direction: Direction,
        container: &Rect,
    ) -> Option<usize> {
        if current >= rects.len() {
            return None;
        }
        let container_top_edge = container.y;
        let container_right_edge = container.x + container.w as i32;
        let container_bottom_edge = container.y + container.h as i32;
        let container_left_edge = container.x;

        match direction {
            Direction::North => find_north(rects, current, container_top_edge),
            Direction::East => find_east(rects, current, container_right_edge),
            Direction::South => find_south(rects, current, container_bottom_edge),
            Direction::West => find_west(rects, current, container_left_edge),
        }
    }

    /// Find the neighbor in a given direction (`North`, `East`, `South`, `West`) in any visible container, starting from a
    /// given `Rect` identified by the index `current` in an array of [`Rect`]
    pub fn find_neighbor_any(
        rects: &[Rect],
        current: usize,
        direction: Direction,
        containers: &[Rect],
    ) -> Option<usize> {
        if current >= rects.len() {
            return None;
        }

        let mut display_area_top_edge = 0;
        let mut display_area_right_edge = 0;
        let mut display_area_bottom_edge = 0;
        let mut display_area_left_edge = 0;
        for container in containers.iter() {
            if container.x + container.w as i32 > display_area_right_edge {
                display_area_right_edge = container.x + container.w as i32;
            } else if container.x < display_area_left_edge {
                display_area_left_edge = container.x;
            }
            if container.y + container.h as i32 > display_area_bottom_edge {
                display_area_bottom_edge = container.y + container.h as i32;
            } else if container.y < display_area_top_edge {
                display_area_top_edge = container.y;
            }
        }

        match direction {
            Direction::North => find_north(rects, current, display_area_top_edge),
            Direction::East => find_east(rects, current, display_area_right_edge),
            Direction::South => find_south(rects, current, display_area_bottom_edge),
            Direction::West => find_west(rects, current, display_area_left_edge),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Direction, Rect};

    const MAIN_CONTAINER: Rect = Rect {
        x: 0,
        y: 0,
        w: 600,
        h: 600,
    };
    const NORTH_CONTAINER: Rect = Rect {
        x: 0,
        y: -200,
        w: 600,
        h: 200,
    };
    const EAST_CONTAINER: Rect = Rect {
        x: 600,
        y: 0,
        w: 200,
        h: 600,
    };
    const SOUTH_CONTAINER: Rect = Rect {
        x: 0,
        y: 600,
        w: 600,
        h: 200,
    };
    const WEST_CONTAINER: Rect = Rect {
        x: -200,
        y: 0,
        w: 200,
        h: 600,
    };

    const CONTAINERS: [Rect; 5] = [MAIN_CONTAINER, NORTH_CONTAINER, EAST_CONTAINER, SOUTH_CONTAINER, WEST_CONTAINER];

    //              Test layout
    //          +-----------------+
    //          |+---------------+|
    //          ||       7       ||
    //          |+---------------+|
    //          +-----------------+
    // +------+ +-----------------+ +------+
    // |+----+| |+---+ +---+ +---+| |+----+|
    // ||    || || 0 | | 3 | | 4 || ||    ||
    // ||    || |+---+ +---+ +---+| ||    ||
    // ||    || |+---+ +---+ +---+| ||    ||
    // || 10 || || 1 | |   | |   || ||  8 ||
    // ||    || |+---+ |   | |   || ||    ||
    // ||    || |+---+ | 6 | | 5 || ||    ||
    // ||    || || 2 | |   | |   || ||    ||
    // |+----+| |+---+ +---+ +---+| |+----+|
    // +------+ +-----------------+ +------+
    //          +-----------------+
    //          |+---------------+|
    //          ||       9       ||
    //          |+---------------+|
    //          +-----------------+
    const ARRAY: [Rect; 11] = [
        Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        },
        Rect {
            x: 0,
            y: 200,
            w: 200,
            h: 200,
        },
        Rect {
            x: 0,
            y: 400,
            w: 200,
            h: 200,
        },
        Rect {
            x: 200,
            y: 0,
            w: 200,
            h: 200,
        },
        Rect {
            x: 400,
            y: 0,
            w: 200,
            h: 200,
        },
        Rect {
            x: 400,
            y: 200,
            w: 200,
            h: 400,
        },
        Rect {
            x: 200,
            y: 200,
            w: 200,
            h: 400,
        },
        Rect {
            x: 0,
            y: -200,
            w: 600,
            h: 200,
        },
        Rect {
            x: 600,
            y: 0,
            w: 200,
            h: 600,
        },
        Rect {
            x: 0,
            y: 600,
            w: 600,
            h: 200,
        },
        Rect {
            x: -200,
            y: 0,
            w: 200,
            h: 600,
        },
    ];

    #[test]
    fn north_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::North, &MAIN_CONTAINER);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor(&ARRAY, 7, Direction::North, &NORTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 8, Direction::North, &EAST_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 9, Direction::North, &SOUTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 10, Direction::North, &WEST_CONTAINER);
        assert_eq!(res, None);

        let res = Direction::find_neighbor_any(&ARRAY, 0, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(7));
        let res = Direction::find_neighbor_any(&ARRAY, 1, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor_any(&ARRAY, 2, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor_any(&ARRAY, 3, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(7));
        let res = Direction::find_neighbor_any(&ARRAY, 4, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(7));
        let res = Direction::find_neighbor_any(&ARRAY, 5, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor_any(&ARRAY, 6, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor_any(&ARRAY, 7, Direction::North, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 8, Direction::North, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 9, Direction::North, &CONTAINERS);
        assert_eq!(res, Some(2));
        let res = Direction::find_neighbor_any(&ARRAY, 10, Direction::North, &CONTAINERS);
        assert_eq!(res, None);
    }

    #[test]
    fn east_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::East, &MAIN_CONTAINER);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor(&ARRAY, 7, Direction::East, &NORTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 8, Direction::East, &EAST_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 9, Direction::East, &SOUTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 10, Direction::East, &WEST_CONTAINER);
        assert_eq!(res, None);

        let res = Direction::find_neighbor_any(&ARRAY, 0, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor_any(&ARRAY, 1, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor_any(&ARRAY, 2, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor_any(&ARRAY, 3, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor_any(&ARRAY, 4, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(8));
        let res = Direction::find_neighbor_any(&ARRAY, 5, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(8));
        let res = Direction::find_neighbor_any(&ARRAY, 6, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor_any(&ARRAY, 7, Direction::East, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 8, Direction::East, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 9, Direction::East, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 10, Direction::East, &CONTAINERS);
        assert_eq!(res, Some(0));
    }

    #[test]
    fn south_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, Some(2));
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::South, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 7, Direction::South, &NORTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 8, Direction::South, &EAST_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 9, Direction::South, &SOUTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 10, Direction::South, &WEST_CONTAINER);
        assert_eq!(res, None);

        let res = Direction::find_neighbor_any(&ARRAY, 0, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor_any(&ARRAY, 1, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(2));
        let res = Direction::find_neighbor_any(&ARRAY, 2, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(9));
        let res = Direction::find_neighbor_any(&ARRAY, 3, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor_any(&ARRAY, 4, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(5));
        let res = Direction::find_neighbor_any(&ARRAY, 5, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(9));
        let res = Direction::find_neighbor_any(&ARRAY, 6, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(9));
        let res = Direction::find_neighbor_any(&ARRAY, 7, Direction::South, &CONTAINERS);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor_any(&ARRAY, 8, Direction::South, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 9, Direction::South, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 10, Direction::South, &CONTAINERS);
        assert_eq!(res, None);
    }

    #[test]
    fn west_neighbor() {
        let res = Direction::find_neighbor(&ARRAY, 0, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 1, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 2, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 3, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor(&ARRAY, 4, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor(&ARRAY, 5, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor(&ARRAY, 6, Direction::West, &MAIN_CONTAINER);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor(&ARRAY, 7, Direction::West, &NORTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 8, Direction::West, &EAST_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 9, Direction::West, &SOUTH_CONTAINER);
        assert_eq!(res, None);
        let res = Direction::find_neighbor(&ARRAY, 10, Direction::West, &WEST_CONTAINER);
        assert_eq!(res, None);

        let res = Direction::find_neighbor_any(&ARRAY, 0, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(10));
        let res = Direction::find_neighbor_any(&ARRAY, 1, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(10));
        let res = Direction::find_neighbor_any(&ARRAY, 2, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(10));
        let res = Direction::find_neighbor_any(&ARRAY, 3, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(0));
        let res = Direction::find_neighbor_any(&ARRAY, 4, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(3));
        let res = Direction::find_neighbor_any(&ARRAY, 5, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(6));
        let res = Direction::find_neighbor_any(&ARRAY, 6, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(1));
        let res = Direction::find_neighbor_any(&ARRAY, 7, Direction::West, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 8, Direction::West, &CONTAINERS);
        assert_eq!(res, Some(4));
        let res = Direction::find_neighbor_any(&ARRAY, 9, Direction::West, &CONTAINERS);
        assert_eq!(res, None);
        let res = Direction::find_neighbor_any(&ARRAY, 10, Direction::West, &CONTAINERS);
        assert_eq!(res, None);
    }
}
