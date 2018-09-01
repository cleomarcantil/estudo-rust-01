
fn main() {

    println!("Hello World!");

	let x: i8 = 1;

	let a = (1, "aaa");

	let (x, y) = a;

	#[derive(Debug)]
	struct Matrix(f32, f32, f32, f32);

	let z = a.0;

	println!("tupla: {:?}", a);

	let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
	
    println!("{:?}", matrix);
	

	#[derive(Debug)]
	struct Person<'a> {
		name: &'a str,
		age: u8,
	}

	let pes = Person { name: "Fulano", age: 25 };
	println!("{:?}", pes);

	let new_pes = Person { age: 11, ..pes };
	println!("{:?}", new_pes);

	let Person { name: my_name, age: my_age } = pes;
	

	#[derive(Debug)]
	enum WebEvent {
		// An `enum` may either be `unit-like`,
		PageLoad,
		PageUnload,
		// like tuple structs,
		KeyPress(char),
		Paste(String),
		// or like structures.
		Click { x: i64, y: i64 },
	}

	let we = WebEvent::KeyPress;

	//println!("{:?}", we);

	enum Color {
		Red = 0xff0000,
		Green = 0x00ff00,
		Blue = 0x0000ff,
	}

	

}