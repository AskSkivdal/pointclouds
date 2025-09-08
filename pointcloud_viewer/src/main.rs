use std::path::PathBuf;

fn main() {
    let mut pc = pointcloud::Pointcloud::read_from_las(PathBuf::from("./locals/cloud.las"));
    println!("Pointcloud read");

    println!("Vectored: {}", pc.points.count());

    println!("Building rtree");
    pc.points = pc.points.initialize_index();
    println!("Rtree built");

    println!("Rtree ed: {}", pc.points.count());
}
