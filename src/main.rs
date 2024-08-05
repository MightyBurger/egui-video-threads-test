use eframe::egui;
use triple_buffer;

const IMG_X: usize = 512;
const IMG_Y: usize = 512;
const IMGSIZE: usize = IMG_X*IMG_Y;

fn main() {

    let image = Box::new([0u8; IMGSIZE]);
    let (mut buf_input, buf_output) = triple_buffer::triple_buffer(&image);
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
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "egui-test", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc, buf_output))));

    // real men kill their threads lol
    // producer.join().unwrap();
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
            egui::ScrollArea::both().show(ui, |ui| {
                self.display_video(ui);
            });
        });
        ctx.request_repaint();
    }
}
