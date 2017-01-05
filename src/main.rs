#![feature(test)]

#![feature(alloc_system)]
extern crate alloc_system;

extern crate simd;

extern crate test;


mod baseline;

fn main() {
    let size = 10;
    let mut costs = baseline::CostField::new(size, size);
    costs.set(1, 2, 5);
    costs.set(2, 2, 5);
    costs.set(7, 7, 5);
    costs.set(7, 8, 5);
    costs.set(8, 7, 5);
    costs.set(8, 8, 5);
    println!("{:?}", &costs);
    let mut field = baseline::IntegrationField::new(size, size);
	field.calculate(0, 0, &costs); 
//	field.reset();
    println!("{:?}", &field);
    let flow = baseline::FlowField::from_integration_field(&field);
    println!("{:?}", &flow);    
}
