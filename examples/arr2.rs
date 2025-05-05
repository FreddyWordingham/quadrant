use nav::ALL_DIRECTIONS;
use ndarray::Array2;
use quadrant::Mosaic;

fn main() {
    let arr = Array2::from_shape_vec((4, 4), (0..16).collect()).unwrap();
    println!("Original Array:");
    println!("{}", arr);

    let tile_size = 2;
    let overlap = 1;
    let tiles = arr.tiles(tile_size, overlap);

    println!("Tiles:");
    for tile in tiles.iter() {
        println!("{}", tile);
    }

    for d in ALL_DIRECTIONS {
        let border = arr.copy_border(d, 2);
        println!("Border {:?}:", d);
        println!("{}", border);
    }
}
