NIGHTLY=${1:-nightly}
echo "----- Building fractal zoom SIMD -----"
./tools/build_wasm_simd.sh fractal_zoom "$NIGHTLY"
echo "----- Building fractal zoom threaded -----"   
./tools/build_wasm_thread.sh fractal_zoom "$NIGHTLY"
echo "----- Building makepad studio normal -----"
./tools/build_wasm_normal.sh makepad_studio "$NIGHTLY"
echo "----- Building fun audio threaded -----"
./tools/build_wasm_thread.sh fun_audio "$NIGHTLY"

