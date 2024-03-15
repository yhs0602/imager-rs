extern crate dirs;

use home;
use iced;
use iced::{Alignment, Application, clipboard, Command, Element, executor, Length, Renderer, Settings, Theme};
use iced::widget::{button, Column, container};

pub fn main() -> iced::Result {
    // 애플리케이션 상태 및 설정 초기화
    let mut settings: Settings<()> = iced::Settings::default();
    // settings.fonts = vec![Cow::Borrowed("".as_ref())];
    settings.default_font = Font {
        name: "Arial Unicode MS",
        bytes: include_bytes!("/System/Library/Fonts/Supplemental/Arial Unicode.ttf"),
    };
    Imager::run(iced::Settings::default())
}

#[derive(Debug)]
struct Image {
    path: String,
    name: String,
}

#[derive(Debug)]
struct Imager {
    images: Vec<Image>,
}

#[derive(Debug)]
struct ImageItem {
    image: Image,
}


#[derive(Debug, Clone)]
pub enum Message {
    Add,
    ClipboardContent(Option<String>),
}

impl Imager {
    fn new_impl() -> Result<(Self, Command<<Imager as Application>::Message>), String> {
        let mut images = vec![];
        let data_dir = dirs::data_dir().ok_or("데이터 디렉토리를 찾을 수 없습니다.")?;
        let data_dir = data_dir.join("imager");

        if std::fs::create_dir_all(&data_dir).is_err() {
            return Err("데이터 디렉토리를 생성할 수 없습니다.".into());
        }

        let entries = std::fs::read_dir(&data_dir).map_err(|_| "디렉토리를 읽을 수 없습니다.")?;
        for entry in entries {
            let entry = entry.map_err(|_| "디렉토리 항목을 읽을 수 없습니다.")?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                images.push(Image {
                    path: path.to_str().ok_or("파일 경로를 문자열로 변환할 수 없습니다.")?.to_string(),
                    name: name.to_string(),
                });
            }
        }

        Ok((
            Imager {
                images,
            },
            Command::none()
        ))
    }
}

impl Application for Imager {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        if let Ok(res) = Imager::new_impl() {
            res
        } else {
            (Imager {
                images: vec![Image { path: String::from("test"), name: String::from("Error") }],
            }, Command::none())
        }
    }


    fn title(&self) -> String {
        String::from("Imager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Add => {
                self.images.push(Image {
                    path: String::from("test"),
                    name: String::from("test"),
                });
                clipboard::read(|content| Message::ClipboardContent(content))
            }
            Message::ClipboardContent(Some(content)) => {
                println!("Clipboard content: {}", content);
                self.images.push(Image {
                    path: content.clone(),
                    name: content,
                });
                Command::none()
            }
            _ => Command::none()
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let downloads =
            Column::with_children(self.images.iter().map(Image::view))
                .push(
                    button("Add from clipboard")
                        .on_press(Message::Add)
                        .padding(10),
                )
                .spacing(20)
                .align_items(Alignment::End);

        container(downloads)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

impl Image {
    fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        iced::widget::Text::new(&self.name).into()
    }
}
