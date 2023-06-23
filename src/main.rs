use derive_macro::EnumMap;

#[derive(EnumMap)]
enum Size {
	Small,
	Medium,
	Large
}

fn main() {
	let mut map = SizeMap {
		small: 0u8,
		medium: 1u8,
		large: 2u8
	};

	map.set(Size::Medium, 4);

	println!("{}", map.get(Size::Medium));

	map.small = 5;

	println!("{}", map.small);
}
