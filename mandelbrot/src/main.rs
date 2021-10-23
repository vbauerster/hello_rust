mod plotter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = plotter::parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = plotter::parse_complex(&args[3]).expect("error parsing complex");
    let lower_right = plotter::parse_complex(&args[4]).expect("error parsing complex");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    // plotter::render(&mut pixels, bounds, upper_left, lower_right);

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    let bands = pixels.chunks_mut(rows_per_band * bounds.0);
    crossbeam::scope(|spawner| {
        for (i, band) in bands.enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left =
                plotter::pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_upper_right =
                plotter::pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move |_| {
                plotter::render(band, band_bounds, band_upper_left, band_upper_right);
            });
        }
    })
    .unwrap();

    plotter::write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
