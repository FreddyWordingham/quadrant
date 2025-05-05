use nav::Direction;
use ndarray::{Array2, s};

pub trait Mosaic {
    fn copy_region(&self, start: [usize; 2], size: [usize; 2]) -> Self;

    fn copy_border(&self, direction: Direction, size: usize) -> Self;

    fn tiles(&self, tile_size: usize, overlap: usize) -> Array2<Self>
    where
        Self: Sized;
}

impl<T: Clone> Mosaic for Array2<T> {
    fn copy_region(&self, start: [usize; 2], size: [usize; 2]) -> Self {
        let (height, width) = self.dim();
        debug_assert!(start[0] + size[0] <= height);
        debug_assert!(start[1] + size[1] <= width);
        debug_assert!(size.iter().all(|&s| s > 0));

        self.slice(s![
            start[0]..start[0] + size[0],
            start[1]..start[1] + size[1]
        ])
        .to_owned()
    }

    fn copy_border(&self, direction: Direction, size: usize) -> Self {
        let (height, width) = self.dim();
        debug_assert!(size > 0);

        let view = match direction {
            Direction::North => self.slice(s![0..size, ..]),
            Direction::East => self.slice(s![.., (width - size)..]),
            Direction::South => self.slice(s![(height - size).., ..]),
            Direction::West => self.slice(s![.., 0..size]),
        };

        view.to_owned()
    }

    fn tiles(&self, tile_size: usize, overlap: usize) -> Array2<Self> {
        let (height, width) = self.dim();
        debug_assert!(overlap < tile_size);
        debug_assert!(height >= tile_size);
        debug_assert!(width >= tile_size);
        debug_assert_eq!(
            (width - overlap) % (tile_size - overlap),
            0,
            "Image must contain an integer number of tiles"
        );
        debug_assert_eq!(
            (height - overlap) % (tile_size - overlap),
            0,
            "Image must contain an integer number of tiles"
        );

        let num_horizontal_tiles = (width - overlap) / (tile_size - overlap);
        let num_vertical_tiles = (height - overlap) / (tile_size - overlap);

        let step_size = tile_size - overlap;
        Array2::from_shape_fn((num_vertical_tiles, num_horizontal_tiles), |(y, x)| {
            let start_y = y * step_size;
            let start_x = x * step_size;
            self.copy_region([start_y, start_x], [tile_size, tile_size])
        })
    }
}
