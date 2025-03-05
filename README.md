# Flood Fill

A small experiment trying simulate floods. It works for Slovenia only, because it's using public geodetic data that
has a format specific to Slovenia.

## Algorithm

The idea behind an algorithm is:

1. User gives a location on map.
2. We find elevation of that location.
3. We find all points that are both:
   - Lower than initial location.
   - Can be gotten to from initial location. This is done by "BFS". We start at initial location,
     jump to near points and repeat for each near point.

Unfortunately the data is way to sparse and the algorithm ignores "small" land features, such as roads,
railways, etc. This means that most of the time the result is half of the country being flooded, because half of
the country has lower elevation that the user input.

## Usage

1. Get data from [here](https://ipi.eprostor.gov.si/jgp/data),
   `Portal Prostor > Data collection > National topographic system > Digital elevation model`. You need all four
   `DEM 0050` files.
2. Unzip the files.
3. Convert into binary format with:
   ```sh
    cargo run --release -- import --input path/to/unzipped/data --output data.bin
   ```
4. Simulate a flood
   ```sh
   cargo run --release -- simulate -v \
     --lat <lat> --lon <lon> \
     --depth 0.2  \
     --data ./data.bin
   ```
5. Open `flood.html` which the program outputs in your browser. You can also view the overlay image
   which is stored at `flood.png`.

> [!NOTE]  
> You should use release mode, otherwise you will wait for a _very_ long time.

See command help `cargo run -- simulate --help` to see what different options do.
