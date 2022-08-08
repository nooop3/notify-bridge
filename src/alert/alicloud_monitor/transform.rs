use crate::{
    alert::alicloud_monitor::message::{EventLevel, ThresholdAlertState},
    notify::feishu::card::TemplateColor,
};

pub fn threshold_alert_state_to_feishu_template_color(
    state: &ThresholdAlertState,
) -> TemplateColor {
    match state {
        ThresholdAlertState::OK => TemplateColor::Green,
        ThresholdAlertState::Alert => TemplateColor::Red,
        ThresholdAlertState::InsufficientData => TemplateColor::Grey,
    }
}

pub fn event_level_to_feishu_template_color(level: &EventLevel) -> TemplateColor {
    match level {
        EventLevel::Critical => TemplateColor::Red,
        EventLevel::Warning => TemplateColor::Yellow,
        EventLevel::Info => TemplateColor::Orange,
    }
}
