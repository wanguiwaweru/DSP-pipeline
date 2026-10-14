use dsp::framing::frames;

fn main() {
    println!("Hello, world!");
    let samples: Vec<f32> = (0..2048).map(|x| x as f32).collect();

    for (i, frame) in frames(&samples, 1024, 256).enumerate() {
        println!(
            "Frame {i}: first={}, last={}, len={}",
            frame[0],
            frame[frame.len() - 1],
            frame.len(),
        );
    }
}
