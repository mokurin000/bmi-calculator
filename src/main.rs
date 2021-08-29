#![no_std]

extern crate ndless_handler;

use ndless::msg::{msg, msg_2numeric};
use ndless::prelude::*;

fn main() {
    if let Some((weight, height)) = msg_2numeric(
        "身体数据",
        "",
        "体重（公斤）",
        (0, i32::MAX),
        "身高（厘米）",
        (0, i32::MAX),
    ) {
        let height: f64 = height as f64 / 100.0;
        let bmi = weight as f64 / height.powi(2);

        let status = match bmi {
            _ if bmi < 18.5 => "过轻",
            _ if bmi < 24.0 => "正常",
            _ if bmi < 28.0 => "超重（亚肥胖）",
            _ if bmi < 30.0 => "肥胖一级",
            _ if bmi < 40.0 => "肥胖二级",
            _ => "肥胖三级",
        };
        msg("BMI", bmi.to_string().as_str());
        msg("身体状况", status);
    }
}
