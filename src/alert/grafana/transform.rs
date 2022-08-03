use crate::{alert::grafana::message::AlertState, notify::feishu::card::TemplateColor};

pub fn alert_state_to_feishu_template_color(state: &AlertState) -> TemplateColor {
    match state {
        AlertState::NoData => TemplateColor::Red,
        AlertState::Paused => TemplateColor::Yellow,
        AlertState::Alerting => TemplateColor::Red,
        AlertState::OK => TemplateColor::Green,
        AlertState::Pending => TemplateColor::Yellow,
        AlertState::Unknown => TemplateColor::Yellow,
    }
}
