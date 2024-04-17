## Building

Run the below commands. The built static libraries and header files will be in the `/out` folder
```bash
mkdir -p build
rm -rf out
mkdir -p out
cd build
cmake -DCMAKE_TOOLCHAIN_FILE=../aarch64-linux-gnu.toolchain.cmake -DCMAKE_BUILD_TYPE=Release ..       
make
cd ../out
cp ../build/libzenohlib.a ./
cp ../build/libzenohrs.a ./
cp -r ../build/corrosion_generated/cxxbridge/zenohlib/include/ ./include/
```