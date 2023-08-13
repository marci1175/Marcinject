use rfd::FileDialog;
use windows_sys::{Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK}, w};
use dll_syringe::{Syringe, process::OwnedProcess};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    path: String,
    proc: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            path: String::new(),
            proc: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    
}

impl eframe::App for TemplateApp {
   
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        _frame.set_always_on_top(true);
        _frame.set_window_size(egui::vec2(280.0, 145.0));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.label("Path");
                ui.text_edit_singleline(&mut self.path);
                if ui.button("Browse").clicked(){
                    let files = FileDialog::new()
                            .set_title("Open")
                            .set_directory("/")
                            .add_filter("Dynamic Link Library", &["dll"])
                            .pick_file();
                    if files.clone().is_some(){
                        self.path = files.unwrap().display().to_string();
                    }
                
                }
                ui.separator();
                ui.label("Process");
                ui.text_edit_singleline(&mut self.proc);
                if ui.button("Inject").clicked() {
                        if OwnedProcess::find_first_by_name(self.proc.clone()).is_none() {
                            unsafe{
                                MessageBoxW(0, w!("Process not found, check spelling!"), w!("Error"), MB_ICONERROR | MB_OK);
                            }
                        }
                        else {
                            let target_process = OwnedProcess::find_first_by_name(self.proc.clone()).unwrap();
                            let syringe = Syringe::for_process(target_process);
                            match syringe.inject(self.path.clone().to_owned()){
                                    Err(e) => {
                                    println!("{}", e);
                                    unsafe{
                                        MessageBoxW(0, w!("DLL not found, check path!"), w!("Error"), MB_ICONERROR | MB_OK);
                                    }
                                    
                                },
                                Ok(_) => {},
                            };
                        };
                        
                    }
                
            });
        });
    }
}
