## Required setup:
    1. Build fishgen with `cargo build -p fishgen-py`
    2. Find the library in the `target/` folder (under either target/release or target/debug)
    3. 
        - on macOS, rename libfishgen.dylib to fishgen.so.
        - on Windows, rename libfishgen.dll to fishgen.pyd.
        - on Linux, rename libfishgen.so to fishgen.so.
    4. Move the library into either the `include` directory of your virtualenv or next to `main.py`.
