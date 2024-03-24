mod point_mass;
use cgmath::InnerSpace;
use point_mass::PointMass;

fn main() {
    let mut black_hole = PointMass::new(
        cgmath::Point3::new(0.0, 0.0, 0.0),
        cgmath::Vector3::new(0.0, 0.0, 0.0),
        point_mass::SOLAR_MASS * 20.0,
    );

    let mut space_craft = PointMass::new(
        cgmath::Point3::new(150000000.0, 0.0, 0.0),
        cgmath::Vector3::new(0.0, 1000000.0, 0.0),
        10000.0,
    );

    while (space_craft.position - black_hole.position).magnitude() > 1.0 {
        let force_on_space_craft = black_hole.force_on(&space_craft);
        space_craft.integration_step(force_on_space_craft, 0.01);
        black_hole.integration_step(-force_on_space_craft, 0.01);
        println!("Spacecraft position: {:?}", space_craft.position);
        // println!("Black hole position: {:?}", black_hole.position);
    }
}
