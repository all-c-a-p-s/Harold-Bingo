#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::*;

struct Entry {
    label: String,
    ticked: bool,
}

impl Entry {
    fn from_label(label: String) -> Self {
        Entry {
            label,
            ticked: false,
        }
    }
}

struct Board {
    rows: Vec<Vec<Entry>>,
}

const LABEL_LIST: [&'static str; 25] = [
    "STOP RUNNING",
    "dont walk through field",
    "dont climb fence",
    "singing hymns",
    "jamaican drill or \nsimilar on aux",
    "moby dick",
    "are you sure?",
    "walk at the speed \nof the slowest person",
    "metatheatrical",
    "word nobody knows \non eye spy",
    "A level drama",
    "tough life in devon",
    "ghost story",
    "black belt karate",
    "I could beat \ntaylor in a fight",
    "does acting",
    "private land",
    "athlete's foot",
    "A level French",
    "A level greek",
    "story about how \nsome gay guy had \na crush on him",
    "getting navigation \nwrong 10 times",
    "stay closer together",
    "talk about DofE",
    "say it's a metaphor \nfor the most \nrandom thing",
];

impl Default for Board {
    fn default() -> Self {
        Self {
            rows: vec![
                LABEL_LIST[0..=4]
                    .iter()
                    .map(|x| Entry::from_label(String::from(*x)))
                    .collect::<Vec<Entry>>(),
                LABEL_LIST[5..=9]
                    .iter()
                    .map(|x| Entry::from_label(String::from(*x)))
                    .collect::<Vec<Entry>>(),
                LABEL_LIST[10..=14]
                    .iter()
                    .map(|x| Entry::from_label(String::from(*x)))
                    .collect::<Vec<Entry>>(),
                LABEL_LIST[15..=19]
                    .iter()
                    .map(|x| Entry::from_label(String::from(*x)))
                    .collect::<Vec<Entry>>(),
                LABEL_LIST[20..=24]
                    .iter()
                    .map(|x| Entry::from_label(String::from(*x)))
                    .collect::<Vec<Entry>>(),
            ],
        }
    }
}

impl Board {
    fn count_bingos(&self) -> i32 {
        let mut count = 0;
        for row in &self.rows {
            if row[0].ticked && row[1].ticked && row[2].ticked && row[3].ticked && row[4].ticked {
                count += 1;
            }
        }

        for i in 0..5 {
            let mut c = 0;
            for r in &self.rows {
                if r[i].ticked {
                    c += 1;
                }
            }

            if c == 5 {
                count += 1;
            }
        }

        let mut c1 = 0;
        for i in 0..5 {
            if self.rows[i][i].ticked {
                c1 += 1;
            }
        }
        if c1 == 5 {
            count += 1;
        }

        let mut c2 = 0;
        for i in 0..5 {
            if self.rows[i][4 - i].ticked {
                c2 += 1;
            }
        }
        if c2 == 5 {
            count += 1;
        }

        count
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                for r in self.rows.iter_mut() {
                    for e in r.iter_mut() {
                        let s = e.label.clone();

                        let colour = if e.ticked {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        };

                        let response = ui.add(
                            egui::Button::new(
                                egui::RichText::new(s)
                                    .size(14.0)
                                    .color(egui::Color32::BLACK),
                            )
                            .min_size(Vec2::new(150.0, 150.0))
                            .fill(colour),
                        );
                        if response.clicked() {
                            e.ticked = !e.ticked;
                        }
                    }
                    ui.end_row();
                }

                ui.label(
                    egui::RichText::new(format!("BINGOS: {}", self.count_bingos()).as_str())
                        .size(20.0),
                );
            });
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let _canvas = document
            .get_element_by_id("canvas_id")
            .expect("Failed to find canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                "canvas_id",
                web_options,
                Box::new(|_| Ok(Box::new(Board::default()))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Harold Bingo",
        opts,
        Box::new(|_| Ok(Box::<Board>::default())),
    )
}
