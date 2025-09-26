#![windows_subsystem = "windows"]

use std::fs;
use iced::widget::{button, column, container, row, text, Column, Text};
use iced::Length::Fill;
use iced::{Center, Color, Element, Pixels, Renderer, Subscription, Task, Theme};
use iced::time;
use iced::time::every;
use std::time::Duration;



pub fn main() -> iced::Result {
    iced::application("RandomPicker", Randomizer::update, Randomizer::view)
        .subscription(Randomizer::subscription)
        .run_with(|| (Randomizer::new(), iced::Task::none()))
}

const TICK_TIME: usize = 50;

#[derive(Debug, Clone, Copy)]
enum Message {
    Reset,
    Random,
    Tick,
    StartRandom
}

#[derive(Default)]
struct Randomizer {
    name: String,
    names: Vec<(String, bool)>,
    name_size: Pixels,
    animating: bool,
    remaining_ticks: usize
}
impl Randomizer {
    fn new() -> Self {
        Self {
            name: String::new(),
            names: read_names(),
            name_size: 50.into(),    // 👈 default value here
            animating: false,
            remaining_ticks: 0,
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                for name in &mut self.names {
                    name.1 = false;
                } 
                self.name = "".to_string();
            }
            Message::Random => {
                let mut remaining_names = self
                    .names
                    .iter_mut()
                    .filter(|line| !line.1)
                    .collect::<Vec<&mut (String, bool)>>();
                
                let max_range = remaining_names.len();

                if max_range != 0{
                    let name: &mut (String, bool) = remaining_names
                        .get_mut(rand::random_range(0..max_range))
                        .unwrap();

                    name.1 = true;

                    self.name = name.0.to_string();
                }
                else{
                    self.name = "Everyone was selected".to_string();
                }
            },
            Message::StartRandom => {
                // start animation
                self.animating = true;
                self.remaining_ticks = rand::random_range(300 / TICK_TIME.. 1500 / TICK_TIME); // between 2s and 5s
            },
            Message::Tick =>{

                let max_range = self.names.len();
                self.remaining_ticks -= 1;

                if self.remaining_ticks > 0{
                    self.name = self.names.get_mut(rand::random_range(0..max_range)).unwrap().0.to_string();
                    self.name_size = 30.into();
                }
                else{
                    self.animating = false;
                    let _ = &self.update(Message::Random);
                    self.name_size = 50.into();
                }
            }
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        if self.animating {
            iced::time::every(Duration::from_millis(TICK_TIME as u64)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let name_list = self.names
        .iter()
        .map(|(name, flag)| {
            text(name)
                .size(20)
                .color(get_color(*flag))
                .into()
        });
        
        container(
            row![
                //Left Column - Randomizer
                column![
                text(&self.name)
                    .size(self.name_size)
                    .height(200)
                    .color(get_color(self.animating)),
                button("Random")
                    .on_press(Message::StartRandom)
                    .width(140),
                button("Reset")
                    .on_press(Message::Reset)
                    .width(140)
                ].align_x(Center)
                .spacing(10)
                .width(Fill),
                //Right Column - Names
                column(name_list).align_x(Center)
                .width(Fill)
            ]
        )
        .padding(20)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

pub fn read_names() -> Vec<(String, bool)> {
    print!("test");
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<(String, bool)> = input
        .lines()
        .map(|line| (line.to_string(), false))
        .collect();
    return lines;
}

pub fn get_color(was_used:bool) -> Color{
    if was_used{
        Color::from_rgb8(128, 128, 128)
    }
    else{
        Color::WHITE
    }
}