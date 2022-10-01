#![windows_subsystem = "windows"]

mod dragon_view;
use dragon_view::DragonView;

use egui::{Context, Color32, TextStyle, FontId};
use glutin::event_loop::{EventLoop, ControlFlow};
mod graphics;

pub struct UiState {
    request_redraw: i8, //draw 2 frames per request to fix some things not getting redrawn
}

fn main() {
    let mut dragon_view = DragonView::default();

    let el = EventLoop::new();
    let mut graphics_state = graphics::Graphics::setup(&el, (680, 700));
    set_egui_visuals(&mut graphics_state.egui_state.ctx);

    let mut ui_state = UiState{ request_redraw: 1 };

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + std::time::Duration::from_millis(20));

        graphics::event_handling(event, control_flow, &mut graphics_state.egui_state, &mut ui_state);

        if ui_state.request_redraw > 0 {
            ui_state.request_redraw -= 1;

            graphics_state.egui_state.ctx.begin_frame(graphics_state.egui_state.raw_input.take());
            create_ui(&mut graphics_state.egui_state.ctx, &mut dragon_view);
            graphics_state.paint();
        }
    });
}

fn create_ui(ctx: &mut Context, dragon_view: &mut DragonView) {
    top_panel(ctx, dragon_view);
    side_panel(ctx, dragon_view);
    central_panel(ctx, dragon_view);
}

fn top_panel(ctx: &mut Context, dragon_view: &mut DragonView) {
    let top_frame = egui::containers::Frame {
        inner_margin: egui::style::Margin { left: 10.0, right: 10.0, top: 6.0, bottom: 6.0 },
        outer_margin: egui::style::Margin { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
        rounding: egui::Rounding::default(),
        shadow: egui::epaint::Shadow::default(),
        fill: Color32::BLACK,
        stroke: egui::Stroke::default(),
    };

    egui::TopBottomPanel::top("top_panel")
    .frame(top_frame)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            let mut temp_level = dragon_view.player.level + 1;

            ui.add(
                egui::DragValue::new(&mut temp_level)
                .speed(0.2)
                .clamp_range(1 ..= 46)
                .prefix("Level: ")
                .custom_formatter(|n, _|
                    if n == 46.0 {
                        String::from("Master")
                    } else {
                        n.to_string()
                    }
                )
            );

            dragon_view.player.level = temp_level - 1;
        });
    });
}

fn side_panel(ctx: &mut Context, dragon_view: &mut DragonView) {
    egui::SidePanel::left("my_left_panel")
    .min_width(210.0)
    .resizable(false)
    .show(ctx, |ui| {
        egui::ScrollArea::vertical()
        .max_height(670.0)
        .show(ui, |ui| {
            for enemy in dragon_view::enemy::Enemy::list() {
                let name = enemy.get_enemy().name;
    
                if ui.add(egui::SelectableLabel::new(dragon_view.current_enemy == enemy, name)).clicked() {
                    dragon_view.is_boss = false;
                    dragon_view.current_enemy = enemy;
                }
            }

            ui.separator();

            for boss in dragon_view::enemy::Enemy::list_boss() {
                let name = boss.get_enemy().name;
    
                if ui.add(egui::SelectableLabel::new(dragon_view.current_enemy == boss, name)).clicked() {
                    dragon_view.is_boss = true;
                    dragon_view.current_enemy = boss;
                }
            }
        });
    });
}

fn central_panel(ctx: &mut Context, dragon_view: &mut DragonView) {
    let central_frame = egui::containers::Frame {
        inner_margin: egui::style::Margin { left: 10.0, right: 10.0, top: 10.0, bottom: 10.0 },
        outer_margin: egui::style::Margin { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
        rounding: egui::Rounding::default(),
        shadow: egui::epaint::Shadow::default(),
        fill: Color32::from_rgb(0x30, 0x80, 0xA0),
        stroke: egui::Stroke::default(),
    };

    egui::CentralPanel::default()
    .frame(central_frame)
    .show(ctx, |ui| {
        egui::Grid::new("dmg_grid").show(ui, |ui| {
            for (idx, (name, damage)) in dragon_view.damage_dealt().iter().enumerate() {
                ui.colored_label(Color32::from_rgb(240, 240, 240), name);

                let hp = match dragon_view.is_boss {
                    true => 184,
                    false => 40,
                };

                for &level in damage {
                    let color = if level < 1 {
                        Color32::LIGHT_RED
                    } else if level >= hp {
                        Color32::LIGHT_GREEN
                    } else {
                        Color32::from_rgb(240, 240, 240)
                    };

                    ui.colored_label(color, format!("{:3}", level.clamp(1, hp)));
                }

                ui.end_row();

                if idx == 1 || idx == 5 || idx == 8 {
                    ui.end_row();
                }
            }
        });

        ui.add_space(20.0);
        ui.colored_label(Color32::from_rgb(240, 240, 240), format!("defense: {}", dragon_view.current_enemy.get_enemy().defense));
        ui.colored_label(Color32::from_rgb(240, 240, 240), format!("xp: {}", dragon_view.current_enemy.get_enemy().xp));
    });
}

fn set_egui_visuals(ctx: &mut Context) {
    use egui::FontFamily::Proportional;

    let mut visuals = egui::Visuals::default();

    visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(240, 240, 240);
    visuals.widgets.hovered.fg_stroke.color = Color32::from_rgb(240, 240, 240);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (TextStyle::Heading, FontId::new(16.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, Proportional)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(14.0, Proportional)),
    ].into();

    ctx.set_style(style);
}
