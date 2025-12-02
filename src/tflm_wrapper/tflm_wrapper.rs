extern "C" {
    fn tflm_init(model_data: *const u8, model_size: usize) -> i32;
    fn tflm_set_input(data: *const f32, len: i32);
    fn tflm_invoke() -> i32;
    fn tflm_get_output(out: *mut f32, len: i32);
}

pub fn init_model(model: &[u8]) -> Result<(), i32> {
    let res = unsafe { tflm_init(model.as_ptr(), model.len()) };
    if res == 0 {
        Ok(())
    } else {
        Err(res)
    }
}

pub fn set_input(input: &[f32]) {
    unsafe { tflm_set_input(input.as_ptr(), input.len() as i32) };
}

pub fn invoke() -> Result<(), i32> {
    let res = unsafe { tflm_invoke() };
    if res == 0 {
        Ok(())
    } else {
        Err(res)
    }
}

pub fn get_output(output: &mut [f32]) {
    unsafe { tflm_get_output(output.as_mut_ptr(), output.len() as i32) };
}

/*

fn main() {
    // Wczytaj model .tflite jako bytes
    let model_data = include_bytes!("model.tflite");

    init_model(model_data).unwrap();

    let input = [0.1_f32, 0.2, 0.3, 0.4];
    set_input(&input);

    invoke().unwrap();

    let mut output = [0.0_f32; 3];
    get_output(&mut output);

    println!("Output: {:?}", output);
}
 */
