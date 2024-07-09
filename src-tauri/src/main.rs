// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;
//旋转方向
struct  Revolve{
    rotating:f64,
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
            },
            _=>{
                self.scaling=Self::_Scale;
                self.scaling
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
    const _Angle: f64 = 90.0; //默认旋转角度
    //初始化的时候，默认给的值，为0
     fn new()->Self{
        Self{
            rotating:Self::_Angle,
        }
     }
    fn init(&mut self){
        self.rotating=Self::_Angle;
    }
    fn set_angle_deg(&mut self,direction:RevolveDirection)->f64 {
        let temp=match direction{
            RevolveDirection::Left => {
                self.rotating-=Self::_Angle;
                self.rotating
            },
            RevolveDirection::Right =>{
                self.rotating+=Self::_Angle;
                self.rotating
            },
            _=>{
                self.rotating=Self::_Angle;
                self.rotating
            }
        };
        self.rotating+=temp;
        println!("{}-{temp}",self.rotating);
        self.rotating
    }
    fn get_angle_deg(&self)->f64{
        self.rotating
    }

}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

//保存文件基本功能操作了
#[tauri::command]
fn savePathUrl(file_name:String,content:String)->std::io::Result<()> {
    if file_name.is_empty(){
        let from=std::string::String::from("文件不存在,无法保存");
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound,from));
    }
    if content.is_empty(){
        let from=std::string::String::from("路径不存在,无法保存");
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound,from));
    }
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let full_file_name = format!("{}-{}", file_name, timestamp);
    let mut file = File::create(full_file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
