use fishgen;

fn main() {
    let random = fishgen::random_fish(1024, 1024);
    random.image.save("/tmp/fish.png").unwrap();
}
