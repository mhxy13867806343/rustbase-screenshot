// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::ptr::read;
use chrono::prelude::*;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;


//旋转方向
struct  Revolve{
    rotating:f64,
}
lazy_static! {
    static ref GLOBAL_REVOLVE_Direction: Mutex<Revolve> = Mutex::new(Revolve::new());
}
//放大缩小
struct  Scale{
    scaling:f64,
    disabled:ScaleDisabled,
}
#[derive(Serialize, Deserialize, Debug)]
enum RevolveDirection{
    Left,
    Right,
}
#[derive(Serialize, Deserialize, Debug)]
enum ScaleDirection{
    Left,
    Right,
}
struct  ScaleDisabled(bool);
//放大缩小
impl  Scale {
    const _Scale: f64 = 1_00.00; //默认放大缩小倍数
    const _Max: f64 = 500.0; //最大放大倍数
    const _Min: f64 = 25.0; //最小放大倍数
    const _temp200: f64 = 200.0; // 200%
    const _temp675: f64 = 67.5; // 67.5%
    const _1: f64 = 1.0; // 1%
    const _01: f64 =0.1; // 15%
    const _015: f64 =0.15; // 15%
    const _030: f64 = 0.30; // 30%
    fn new ()->Self{
        Self{
            scaling:Self::_Scale,
            disabled:ScaleDisabled(false),
        }
    }
    fn init(&mut self){
        self.scaling=if Self::_Scale<Self::_Min {
            Self::_Min
        }else{
            Self::_Scale
        };
    }
    fn set_scale_deg(&mut self,direction:ScaleDirection)->f64 {
        let temp=match direction{
            ScaleDirection::Left => {
               if self.scaling>Self::_temp675{
                   (self.scaling-(self.scaling*Self::_01)).max(Self::_Min)
               }else if self.scaling> Self::_Min{
                    (self.scaling-(self.scaling*Self::_015)).max(Self::_Min)
                }else{
                   if self.scaling<=Self::_Min{
                       self.disabled=ScaleDisabled(false);
                   }
                   self.scaling
               }
            },
            ScaleDirection::Right =>{
                if self.scaling < Self::_Max {
                    // 如果当前缩放比例小于500%
                    if self.scaling > Self::_temp200 {
                        // 如果大于200%，每次增加30%
                        (self.scaling + (self.scaling *Self::_030)).min(Self::_Max)  // 不允许超过500%
                    } else {
                        if self.scaling>=Self::_Max{
                            self.disabled=ScaleDisabled(true);
                        }
                        // 如果小于或等于200%，正常增加10%
                        (self.scaling + (self.scaling * Self::_01)).min(Self::_Max)  // 不允许超过500%
                    }
                } else {
                    if self.scaling>=Self::_Max{
                        self.disabled=ScaleDisabled(true);
                    }
                    // 如果已经是500%，则保持不变
                    self.scaling
                }
            }
        };
        self.scaling+=temp;
        println!("{}-{temp}",self.scaling);
        self.scaling
    }
    fn get_scale_deg(&self)->f64{
        self.scaling
    }
}
//旋转
impl Revolve {
    const _Angle: f64 = 90.0; // 默认旋转角度

    fn new() -> Self {
        Self {
            rotating: 0.0, // 初始化时从0开始
        }
    }
    fn init(&mut self) {
        self.rotating = Self::_Angle; // 重置为0
    }
    // 这个方法现在看起来是正确的，它应该能持续更新角度
    fn set_angle_deg(&mut self, direction: RevolveDirection) -> f64 {
        match direction {
            RevolveDirection::Left => {
                self.rotating -= Self::_Angle; // 向左旋转，角度递减
            },
            RevolveDirection::Right => {
                self.rotating += Self::_Angle; // 向右旋转，角度递增
            },
        };
        self.rotating
    }

    fn get_angle_deg(&self) -> f64 {
        self.rotating
    }
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn setAngleDeg(direction:RevolveDirection)->tauri::Result<f64>{
    let mut revolve = GLOBAL_REVOLVE_Direction.lock().unwrap(); // 锁定全局变量
    revolve.set_angle_deg(direction); // 调整角度
    let result = revolve.get_angle_deg(); // 获取当前角度
    println!("{:?}", result); // 打印结果
    Ok(result) // 返回结果

}
#[tauri::command]
fn getAngleDeg()->tauri::Result<f64>{
    let mut revolve = GLOBAL_REVOLVE_Direction.lock().unwrap();
    let result=revolve.get_angle_deg();
    println!("{:?}",result);
    Ok(result)
}
#[derive(Serialize, Deserialize)]
struct FileInfo {
    file_name: String,
    timestamp: String,
    full_file_name: String,
}
//保存文件基本功能操作了
#[tauri::command]
fn savePathUrl(file_name: String) -> tauri::Result<(FileInfo)> {
    if file_name.is_empty() {
        // 直接使用 std::io::Error
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "不能为空").into());
    }
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let full_file_name = format!("{}-{}", file_name, timestamp);
    let file_info = FileInfo {
        file_name,
        timestamp,
        full_file_name,
    };
    Ok(file_info)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,savePathUrl,setAngleDeg,getAngleDeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
