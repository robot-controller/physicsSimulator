mod wgpu_window;

use wgpu_window::run_wgpu_window;


fn main() {
    pollster::block_on(run_wgpu_window());
}
