use eframe::egui;
use triple_buffer;

const IMG_X: usize = 512;
const IMG_Y: usize = 512;
const IMGSIZE: usize = IMG_X*IMG_Y;

fn main() {

    let image = Box::new([0u8; IMGSIZE]);
    let (mut buf_input, buf_output) = triple_buffer::triple_buffer(&image);

    // Make a thread. It'll generate some images and write it to the triple_buffer.
    let _producer = std::thread::spawn(move || {
        let mut value: u8 = 0;
        loop {
            let input = buf_input.input_buffer();
            for i in 0..IMGSIZE {
                input[i] = value;
            }
            buf_input.publish();
            value = value.wrapping_add(1);
        }
    });

    // Launch the GUI in the main thread.
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "egui-test", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc, buf_output))));

    /*
        Normally, you'd need to make the producer thread stop when closing out the program. 
        Then you'd throw in a producer.join().unwrap() here. But I didn't want to complicate this 
        example, so we'll just let the producer thread die at the end of the program.
    */
}

struct MyEguiApp {
    imageref: triple_buffer::Output<Box<[u8; IMGSIZE]>>
}

impl MyEguiApp {

    fn new(_cc: &eframe::CreationContext<'_>, 
    imageref: triple_buffer::Output<Box<[u8; IMGSIZE]>>) -> Self {
        Self { imageref }
    }

    fn display_video(&mut self, ui: &mut egui::Ui) {
        self.imageref.update();
        let output = self.imageref.output_buffer();
        let img = egui::ColorImage::from_gray([IMG_X, IMG_Y], &(**output));
        let texture = ui.ctx().load_texture("frame", img, Default::default());
        ui.image(&texture);
    }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_video(ui);
        });
        ctx.request_repaint();
    }
}
