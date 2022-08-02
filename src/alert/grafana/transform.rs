use crate::{alert::grafana::message::GrafanaAlertState, notify::feishu::card::TemplateColor};


pub fn alert_state_to_feishu_template_color(state: &GrafanaAlertState) -> TemplateColor {
    match state {
        GrafanaAlertState::NoData => TemplateColor::Red,
        GrafanaAlertState::Paused => TemplateColor::Yellow,
        GrafanaAlertState::Alerting => TemplateColor::Red,
        GrafanaAlertState::OK => TemplateColor::Green,
        GrafanaAlertState::Pending => TemplateColor::Yellow,
        GrafanaAlertState::Unknown => TemplateColor::Yellow,
    }
}
