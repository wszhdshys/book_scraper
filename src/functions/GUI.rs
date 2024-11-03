use crate::functions::gui::Windows::{Data, Thanks, Update};
use crate::functions::select_position::select_position;
use crate::functions::setfont;
use crate::functions::sort::data_conclusion;
use crate::functions::sort::sort;
use crate::functions::update::date;
use crate::functions::update::update;
use crate::functions::update::House;
use crate::MY_FONTS_BYTES;
use eframe::egui;
use egui::{vec2, CentralPanel, Color32, Frame, RichText, ScrollArea, Ui, Window};
use std::thread;
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

pub fn crate_gui() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ZH二手房数据查询",
        options,
        //Box::new(|_cc| Ok(Box::<MyApp>::default())),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

#[derive(Default)]
enum Windows {
    #[default]
    Main,
    Search,
    Update,
    Data,
    Thanks,
}

#[derive(Default)]
pub enum Sort {
    #[default]
    Default,
    Ppm,
    Tp,
    Size,
    Rb,
}

#[derive(Default)]
enum State {
    #[default]
    Yes,
    Stay,
    No,
}

#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    lbltext: String,
    position: String,
    selection: Windows,
    sort: Sort,
    state: State,
    u_d: u8,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setfont::setup_custom_fonts(&cc.egui_ctx, MY_FONTS_BYTES);
        Self {
            show_confirmation_dialog: false,
            allowed_to_close: false,
            lbltext: "".to_string(),
            position: "".to_string(),
            selection: Windows::Main,
            sort: Sort::Default,
            state: State::No,
            u_d: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            Window::new("你想要关闭吗?")
                .fixed_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("否").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("是").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
        let _screen = match self.selection {
            Windows::Main => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(135.0);
                        ui.heading(
                            RichText::new("欢迎来到ZH二手房数据查看系统")
                                .color(Color32::from_rgb(255, 0, 0))
                                .size(64.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(90.0);
                        ui.label(RichText::new("请输入你想要查询的城市：").size(36.0));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.lbltext)
                                .desired_width(480.0) // 设置期望宽度
                                .font(egui::FontId::new(30.0, Default::default()))
                                .hint_text(RichText::new("在此输入").size(27.0)), // 提示文本
                        );
                        let enter = ui.add(
                            egui::Button::new(RichText::new("确定").size(30.0))
                                .min_size(vec2(108.0, 36.0)),
                        );
                        if enter.clicked() {
                            self.selection = Windows::Search;
                        }
                    });
                    ui.vertical(|ui| {
                        ui.add_space(330.0);
                        ui.horizontal(|ui| {
                            ui.add_space(26.0);
                            let data_update = ui.add(
                                egui::Button::new(RichText::new("数据更新").size(36.0))
                                    .stroke(egui::Stroke::new(2.0, Color32::RED))
                                    .min_size(vec2(360.0, 120.0)),
                            );
                            if data_update.clicked() {
                                self.selection = Update;
                            }
                            ui.add_space(57.0);
                            let data = ui.add(
                                egui::Button::new(RichText::new("数据总览").size(36.0))
                                    .stroke(egui::Stroke::new(2.0, Color32::RED))
                                    .min_size(vec2(360.0, 120.0)),
                            );
                            if data.clicked() {
                                self.selection = Data;
                            }
                            ui.add_space(57.0);
                            let name = ui.add(
                                egui::Button::new(RichText::new("制作名单").size(36.0))
                                    .stroke(egui::Stroke::new(2.0, Color32::RED))
                                    .min_size(vec2(360.0, 120.0)),
                            );
                            if name.clicked() {
                                self.selection = Thanks;
                            }
                        })
                    })
                });
            }
            Windows::Search => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("您现在正在浏览{}的二手房：", &self.lbltext));
                        let _sort_update = ui.menu_button("排序", |ui| {
                            if ui.button("默认").clicked() {
                                self.sort = Sort::Default;
                            };
                            ui.menu_button("平方价", |ui| {
                                if ui.button("升序").clicked() {
                                    self.sort = Sort::Ppm;
                                };
                                if ui.button("降序").clicked() {
                                    self.u_d = 1;
                                    self.sort = Sort::Ppm;
                                };
                            });
                            ui.menu_button("总价", |ui| {
                                if ui.button("升序").clicked() {
                                    self.sort = Sort::Tp;
                                };
                                if ui.button("降序").clicked() {
                                    self.u_d = 1;
                                    self.sort = Sort::Tp;
                                };
                            });
                            ui.menu_button("大小", |ui| {
                                if ui.button("升序").clicked() {
                                    self.sort = Sort::Size;
                                };
                                if ui.button("降序").clicked() {
                                    self.u_d = 1;
                                    self.sort = Sort::Size;
                                };
                            });
                            ui.menu_button("首付", |ui| {
                                if ui.button("升序").clicked() {
                                    self.sort = Sort::Rb;
                                };
                                if ui.button("降序").clicked() {
                                    self.u_d = 1;
                                    self.sort = Sort::Rb;
                                };
                            });
                        });
                        if ui.button("关闭").clicked() {
                            self.selection = Windows::Main; // 关闭新窗口
                        } // 显示传入的字符串
                    });
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.add_space(600.0);
                        });
                        match self.sort {
                            Sort::Default => {
                                let temp = sort(select_position(&self.lbltext), Sort::Default, self.u_d);
                                create_scroll(ui, temp)
                            }
                            Sort::Ppm => {
                                let temp =
                                    sort(select_position(&self.lbltext), Sort::Ppm, self.u_d);
                                create_scroll(ui, temp)
                            }
                            Sort::Tp => {
                                let temp = sort(select_position(&self.lbltext), Sort::Tp, self.u_d);
                                create_scroll(ui, temp)
                            }
                            Sort::Size => {
                                let temp =
                                    sort(select_position(&self.lbltext), Sort::Size, self.u_d);
                                create_scroll(ui, temp)
                            }
                            Sort::Rb => {
                                let temp = sort(select_position(&self.lbltext), Sort::Rb, self.u_d);
                                create_scroll(ui, temp)
                            }
                        }
                    });
                    ui.horizontal(|ui| {})
                });
            }
            Update => {
                let mut handle = thread::spawn(|| ());
                let app = self.lbltext.clone();
                CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(135.0);
                        ui.heading(
                            RichText::new("数据更新系统")
                                .color(Color32::from_rgb(255, 0, 0))
                                .size(64.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(90.0);
                        ui.label(RichText::new("请输入你想要更新或获取的城市：").size(36.0));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.lbltext)
                                .desired_width(480.0) // 设置期望宽度
                                .font(egui::FontId::new(30.0, Default::default()))
                                .hint_text(RichText::new("在此输入").size(27.0)), // 提示文本
                        );
                        let enter = ui.add(
                            egui::Button::new(RichText::new("确定").size(30.0))
                                .min_size(vec2(108.0, 36.0)),
                        );
                        if enter.clicked() {
                            self.state = State::Stay;
                            handle = thread::spawn(move || {
                                update(select_position(&app)).expect("update error");
                            });
                        };
                    });
                    ui.vertical(|ui| {
                        ui.add_space(40.0);
                        ui.label(
                            RichText::new("目前已有的城市及其更新时间：")
                                .color(Color32::BLUE)
                                .size(36.0),
                        );
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.add_space(450.0);
                            });
                            ui.vertical(|ui| {
                                ScrollArea::both()
                                    .enable_scrolling(true)
                                    .scroll_bar_visibility(
                                        egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                    )
                                    .show_rows(ui, 0.0, 20, |ui, rowrange| {
                                        for city in date().iter() {
                                            ui.label(RichText::new(city).size(20.0));
                                        }
                                    });
                            });
                            let retun = ui.add(
                                egui::Button::new(RichText::new("返回").size(36.0))
                                    .min_size(vec2(360.0, 120.0)),
                            );
                            match self.state {
                                State::Yes => ui.label(RichText::new("update已完成").size(36.0)),
                                State::Stay => ui.label(RichText::new("正在update").size(36.0)),
                                State::No => ui.label(RichText::new("update未开始").size(36.0)),
                            };
                            if retun.clicked() {
                                self.selection = Windows::Main;
                            }
                        });
                    })
                });
            }
            Data => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(135.0);
                        ui.heading(
                            RichText::new("数据总览")
                                .color(Color32::from_rgb(255, 0, 0))
                                .size(64.0),
                        );
                        ui.label("收集一些杂七杂八的东西")
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("本数据库已经收集数据条数：")
                                .color(Color32::BLUE)
                                .size(36.0),
                        );
                        ui.label(
                            RichText::new(&data_conclusion[4])
                                .color(Color32::GREEN)
                                .size(36.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("在收集的数据之中，每平方价格最高的是：")
                                .color(Color32::GREEN)
                                .size(36.0),
                        );
                        ui.label(RichText::new(&data_conclusion[0]).color(Color32::GREEN).size(36.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("总价格最高的是：")
                                .color(Color32::BLUE)
                                .size(36.0),
                        );
                        ui.label(RichText::new(&data_conclusion[1]).color(Color32::GREEN).size(36.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("最宽敞的屋子是：")
                                .color(Color32::GREEN)
                                .size(36.0),
                        );
                        ui.label(RichText::new(&data_conclusion[2]).color(Color32::GREEN).size(36.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("首付最高的屋子是：")
                                .color(Color32::BLUE)
                                .size(36.0),
                        );
                        ui.label(RichText::new(&data_conclusion[3]).color(Color32::GREEN).size(36.0));
                    });
                    if ui.button("返回").clicked() {
                        self.selection = Windows::Main;
                    }
                });
            }
            Thanks => {
                CentralPanel::default().show(ctx, |ui| {
                    if ui.button("返回").clicked() {
                        self.selection = Windows::Main;
                    }
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("制作：").color(Color32::BLUE).size(36.0));
                            ui.label(
                                RichText::new("20232131095张颢")
                                    .color(Color32::RED)
                                    .size(36.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("特别鸣谢：").color(Color32::GREEN).size(36.0));
                            ui.label(RichText::new("KKould").color(Color32::YELLOW).size(36.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("特别鸣谢：").color(Color32::GOLD).size(36.0));
                            ui.label(RichText::new("FnckSQL").color(Color32::GRAY).size(36.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("特别鸣谢：").color(Color32::BROWN).size(36.0));
                            ui.label(RichText::new("Chatgpt").color(Color32::BLACK).size(36.0));
                        });
                    });
                });
            }
        };
        ()
    }
}

fn pad_string(s: &str, total_width: usize) -> String {
    let width = UnicodeWidthStr::width(s);
    if width >= total_width {
        s.to_string()
    } else {
        let padding = total_width - width;
        format!("{}{}", s, " ".repeat(padding))
    }
}

fn create_scroll(ui: &mut Ui, temp: Vec<House>) {
    ScrollArea::both()
        .enable_scrolling(true)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show_rows(ui, 0.0, 20, |ui, rowrange| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    for k in 0..18 {
                        let t = match k {
                            16 => RichText::new(pad_string("简述", 95))
                                .color(Color32::BLUE)
                                .size(15.0),
                            0 => RichText::new(pad_string("小区名", 25))
                                .color(Color32::BLUE)
                                .size(15.0),
                            1 => RichText::new(pad_string("地区", 12))
                                .color(Color32::BLUE)
                                .size(15.0),
                            2 => RichText::new(pad_string("价格每平方", 10))
                                .color(Color32::BLUE)
                                .size(15.0),
                            3 => RichText::new(pad_string("总价", 12))
                                .color(Color32::BLUE)
                                .size(15.0),
                            4 => RichText::new(pad_string("布局", 15))
                                .color(Color32::BLUE)
                                .size(15.0),
                            5 => RichText::new(pad_string("楼层", 15))
                                .color(Color32::BLUE)
                                .size(15.0),
                            6 => RichText::new(pad_string("大小", 6))
                                .color(Color32::BLUE)
                                .size(15.0),
                            7 => RichText::new(pad_string("装修情况", 10))
                                .color(Color32::BLUE)
                                .size(15.0),
                            8 => RichText::new(pad_string("方位", 6))
                                .color(Color32::BLUE)
                                .size(15.0),
                            9 => RichText::new(pad_string("竣工", 6))
                                .color(Color32::BLUE)
                                .size(15.0),
                            10 => RichText::new(pad_string("产权性质", 12))
                                .color(Color32::BLUE)
                                .size(15.0),
                            11 => RichText::new(pad_string("物业类型", 12))
                                .color(Color32::BLUE)
                                .size(15.0),
                            12 => RichText::new(pad_string("参考首付", 9))
                                .color(Color32::BLUE)
                                .size(15.0),
                            13 => RichText::new(pad_string("发布公司", 12))
                                .color(Color32::BLUE)
                                .size(15.0),
                            14 => RichText::new(pad_string("营业执照", 30))
                                .color(Color32::BLUE)
                                .size(15.0),
                            15 => RichText::new(pad_string("数据更新时间", 12))
                                .color(Color32::BLUE)
                                .size(15.0),
                            17 => RichText::new(pad_string("链接", 60))
                                .color(Color32::BLUE)
                                .size(15.0),
                            _ => continue,
                        };
                        Frame::none()
                            .stroke(egui::Stroke::new(1.0, Color32::BLACK))
                            .show(ui, |ui| {
                                ui.label(t);
                            });
                    }
                });
                for j in temp.iter() {
                    ui.horizontal(|ui| {
                        for k in 0..18 {
                            let t = match k {
                                16 => RichText::new(pad_string(&j.title, 65))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                0 => RichText::new(pad_string(&j.community, 22))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                1 => RichText::new(pad_string(&j.area, 12))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                2 => RichText::new(format!(
                                    "{}{}",
                                    pad_string(&j.price_per_meter.to_string(), 6),
                                    "元/㎡ "
                                ))
                                .color(Color32::BLACK)
                                .size(15.0),
                                3 => RichText::new(format!(
                                    "{}{}",
                                    pad_string(&j.total_price.to_string(), 4),
                                    "万元 "
                                ))
                                .color(Color32::BLACK)
                                .size(15.0),
                                4 => RichText::new(pad_string(&j.layout, 12))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                5 => RichText::new(pad_string(&j.floor, 15))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                6 => RichText::new(format!(
                                    "{}{}",
                                    pad_string(&j.size.to_string(), 4),
                                    "㎡ "
                                ))
                                .color(Color32::BLACK)
                                .size(15.0),
                                7 => RichText::new(pad_string(&j.decoration, 10))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                8 => RichText::new(pad_string(&j.orientation, 6))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                9 => RichText::new(pad_string(&j.time, 6))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                10 => RichText::new(pad_string(&j.property_ownership_type, 12))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                11 => RichText::new(pad_string(&j.property_type, 12))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                12 => RichText::new(format!(
                                    "{}{}",
                                    pad_string(&j.reference_budget.to_string(), 4),
                                    "万元 "
                                ))
                                .color(Color32::BLACK)
                                .size(15.0),
                                13 => RichText::new(pad_string(&j.publishing_company, 12))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                14 => RichText::new(pad_string(&j.business_license, 20))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                15 => RichText::new(pad_string(&j.update_date, 12))
                                    .color(Color32::BLACK)
                                    .size(15.0),
                                17 => {
                                    Frame::none()
                                        .stroke(egui::Stroke::new(1.0, Color32::BLACK)) // 设置描边，2.0是线条宽度
                                        .show(ui, |ui| {
                                            ui.hyperlink(&j.link);
                                        });
                                    continue;
                                }
                                _ => continue,
                            };
                            Frame::none()
                                .stroke(egui::Stroke::new(1.0, Color32::BLACK))
                                .show(ui, |ui| {
                                    ui.label(t);
                                });
                        }
                    });
                }
            });
            drop(temp);
        });
}

// fn sorted_data(my_app: &MyApp) -> Vec<House> {
//     let temp = sort(select_position(&my_app.lbltext), Sort::Tp, my_app.u_d);
//     temp
// }
