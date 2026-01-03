use raylib::color::Color;
use raylib::prelude::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, BufRead, BufReader, Write};

const WINDOW_HEIGHT: i32 = 480;
const WINDOW_WIDTH: i32 = 640;
enum Screens {
    MainScreen,
    SignupScreen,
    AccountCreatedScreen,
    LoginScreen,
    RetrievePassword,
    AddPassword,
    PasswordScreen,
    WelcomeScreen,
    UserSession,
}

struct State {
    rl: RaylibHandle,
    rl_thread: RaylibThread,
    input: String,
    current_screen: Screens,
    current_user: Option<String>,
    current_key: Option<String>,
    acc_pass: Option<(String, String)>,
    font_size: i32,
    dt: i32,
}

fn main() {
    let (rl, rl_thread) = raylib::init()
        .msaa_4x()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Password Manager")
        .build();
    let current_screen = Screens::MainScreen;
    let mut quit = false;
    let mut state = State {
        rl: rl,
        rl_thread: rl_thread,
        input: String::new(),
        current_screen: current_screen,
        current_user: None,
        current_key: None,
        font_size: 24,
        acc_pass: None,
        dt: 0,
    };
    state.rl.set_target_fps(60);
    while !state.rl.window_should_close() && !quit {
        match state.current_screen {
            Screens::MainScreen => update_main_screen(&mut state, &mut quit),
            Screens::SignupScreen => update_signup_screen(&mut state),
            Screens::AccountCreatedScreen => update_account_created_screen(&mut state),
            Screens::LoginScreen => update_login_screen(&mut state),
            Screens::UserSession => update_usersession_screen(&mut state),
            Screens::WelcomeScreen => update_welcome_screen(&mut state),
            Screens::RetrievePassword => update_retrieve_password(&mut state),
            Screens::AddPassword => update_add_password(&mut state),
            Screens::PasswordScreen => update_password_screen(&mut state),
        }
        state.dt += 1;
    }
}

fn draw_main_screen(
    d: &mut RaylibDrawHandle,
    buttons: Vec<Rectangle>,
    text: Vec<&str>,
    font_size: i32,
) {
    let roundness = 5.0;
    let segments = 12;
    let color = Color::BLACK;

    d.clear_background(Color::ORANGERED);
    for i in 0..buttons.len() {
        d.draw_rectangle_rounded(buttons[i], roundness, segments, color);
        d.draw_text(
            text[i],
            buttons[i].x as i32 + buttons[i].width as i32 / 10,
            buttons[i].y as i32 + buttons[i].height as i32 / 2,
            font_size,
            Color::RAYWHITE,
        );
    }
}

fn draw_signup_screen(
    d: &mut RaylibDrawHandle,
    buttons: Vec<Rectangle>,
    text: Vec<String>,
    label: Vec<&str>,
    font_size: i32,
) {
    let roundness = 5.0;
    let segments = 12;
    let color = Color::BLACK;

    d.clear_background(Color::BLUE);
    for i in 0..buttons.len() {
        if label.len() > 0 {
            d.draw_text(
                label[i],
                buttons[i].x as i32,
                buttons[i].y as i32 - buttons[i].height as i32 / 4,
                font_size,
                Color::RAYWHITE,
            );
        }
        d.draw_rectangle_rounded(buttons[i], roundness, segments, color);
        if i < text.len() {
            d.draw_text(
                text[i].as_str(),
                buttons[i].x as i32 + buttons[i].width as i32 / 10,
                buttons[i].y as i32 + buttons[i].height as i32 / 2,
                font_size,
                Color::RAYWHITE,
            );
        }
    }
}

fn update_main_screen(state: &mut State, quit: &mut bool) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text = vec!["Sign Up", "Login", "Quit"];
    let buttons_width = 1.0;
    let buttons_height = 1.0;

    let no_of_buttons = 3;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * w as f32) * (0.5),
            (buttons_height * h as f32 / no_of_buttons as f32) * (1.0 - 0.4),
        ));
    }

    for i in 0..buttons.len() {
        buttons[i].x = w as f32 / 2.0 - buttons[i].width / 2.0;
        buttons[i].y = (h as f32 / buttons.len() as f32) * i as f32 + buttons[i].height / 2.0;
    }

    if state
        .rl
        .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    {
        for i in 0..buttons.len() {
            if buttons[i].check_collision_point_rec(state.rl.get_mouse_position()) {
                match i {
                    0 => state.current_screen = Screens::SignupScreen,
                    1 => state.current_screen = Screens::LoginScreen,
                    _ => *quit = true,
                }
                println!("Button {i} was clicked");
            }
        }
    }

    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_main_screen(&mut d, buttons, text, state.font_size);
}

fn update_signup_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text: Vec<String> = state.input.lines().map(|x| x.to_string()).collect();
    let label = vec!["Username", "Password", "Confirm Password"];
    let buttons_width = 1.0;
    let buttons_height = 1.0;

    let no_of_buttons = 3;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * w as f32) * (0.5),
            (buttons_height * h as f32 / no_of_buttons as f32) * (1.0 - 0.4),
        ));
    }

    for i in 0..buttons.len() {
        buttons[i].x = w as f32 / 2.0 - buttons[i].width / 2.0;
        buttons[i].y = (h as f32 / buttons.len() as f32) * i as f32 + buttons[i].height / 2.0;
    }

    while let Some(key) = state.rl.get_char_pressed() {
        if key as u8 >= 32 && key as u8 <= 125 {
            state.input.push(key);
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.input.pop() {
            Some('\n') => state.input.push('\n'),
            _ => (),
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        state.input.push('\n');
        if text.len() >= no_of_buttons {
            state.input.clear();
            match signup(&text) {
                Err(e) => println!("Couldn't signup: {}", e),
                Ok(key) => {
                    state.current_key = Some(key);
                    state.current_screen = Screens::AccountCreatedScreen;
                }
            }
        }
    }
    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_signup_screen(&mut d, buttons, text, label, state.font_size);
}

fn update_account_created_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    if let Some(_) = state.rl.get_key_pressed() {
        state.current_screen = Screens::MainScreen;
    }
    let text = "Account Created Press Any key to login";
    let text_offset = state.rl.measure_text(text, 24);

    let mut d = state.rl.begin_drawing(&state.rl_thread);

    d.clear_background(Color::BLACK);

    d.draw_text(
        text,
        w / 2 - text_offset / 2,
        h / 2,
        state.font_size,
        Color::RAYWHITE,
    );
}

fn update_welcome_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    if let Some(_) = state.rl.get_key_pressed() {
        state.current_screen = Screens::UserSession;
    }
    let username = state.current_user.clone();
    let username = username.expect("Should not be none by this point");
    let text = format!("Welcome back {}", username);
    let text = text.as_str();
    let text_offset = state.rl.measure_text(text, 24);

    let mut d = state.rl.begin_drawing(&state.rl_thread);

    d.clear_background(Color::BLACK);

    d.draw_text(
        text,
        w / 2 - text_offset / 2,
        h / 2,
        state.font_size,
        Color::RAYWHITE,
    );
}

fn update_login_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let mut text: Vec<String> = state.input.lines().map(|x| x.to_string()).collect();
    let label = vec!["User name", "Password"];
    let buttons_width = 1.0;
    let buttons_height = 1.0;

    let no_of_buttons = 2;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * w as f32) * (0.5),
            (buttons_height * h as f32 / no_of_buttons as f32) * (1.0 - 0.4),
        ));
    }

    for i in 0..buttons.len() {
        buttons[i].x = w as f32 / 2.0 - buttons[i].width / 2.0;
        buttons[i].y = (h as f32 / buttons.len() as f32) * i as f32 + buttons[i].height / 2.0;
    }

    while let Some(key) = state.rl.get_char_pressed() {
        if key as u8 >= 32 && key as u8 <= 125 {
            state.input.push(key);
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.input.pop() {
            Some('\n') => state.input.push('\n'),
            _ => (),
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        state.input.push('\n');
        if text.len() >= no_of_buttons {
            state.input.clear();
            let login_sesh = login(&mut text);
            match login_sesh {
                Ok((username, key)) => {
                    state.current_user = Some(username);
                    state.current_key = Some(key);
                    state.current_screen = Screens::WelcomeScreen;
                }
                Err(e) => println!("{}", e),
            }
        }
    }
    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_signup_screen(&mut d, buttons, text, label, state.font_size);
}

fn update_usersession_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text = vec!["Add password", "Retrieve Password", "Back"];
    let buttons_width = 1.0;
    let buttons_height = 1.0;

    let no_of_buttons = 3;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * w as f32) * (0.5),
            (buttons_height * h as f32 / no_of_buttons as f32) * (1.0 - 0.4),
        ));
    }

    for i in 0..buttons.len() {
        buttons[i].x = w as f32 / 2.0 - buttons[i].width / 2.0;
        buttons[i].y = (h as f32 / buttons.len() as f32) * i as f32 + buttons[i].height / 2.0;
    }

    if state
        .rl
        .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    {
        for i in 0..buttons.len() {
            if buttons[i].check_collision_point_rec(state.rl.get_mouse_position()) {
                match i {
                    0 => state.current_screen = Screens::AddPassword,
                    1 => state.current_screen = Screens::RetrievePassword,
                    _ => state.current_screen = Screens::MainScreen,
                }
                println!("Button {i} was clicked");
            }
        }
    }

    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_main_screen(&mut d, buttons, text, state.font_size);
}

fn update_retrieve_password(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text: Vec<String> = state.input.lines().map(|x| x.to_string()).collect();
    let label = vec!["Account"];
    let buttons_width = 1.0;
    let buttons_height = 1.0;

    let no_of_buttons = 1;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * w as f32) * (0.5),
            (buttons_height * h as f32 / 3 as f32) * (1.0 - 0.4),
        ));
    }

    for i in 0..buttons.len() {
        buttons[i].x = w as f32 / 2.0 - buttons[i].width / 2.0;
        buttons[i].y = (h as f32 / buttons.len() as f32) * i as f32 + buttons[i].height / 2.0;
    }

    while let Some(key) = state.rl.get_char_pressed() {
        if key as u8 >= 32 && key as u8 <= 125 {
            state.input.push(key);
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.input.pop() {
            Some('\n') => state.input.push('\n'),
            _ => (),
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        state.input.push('\n');
        if text.len() >= no_of_buttons {
            state.input.clear();
            if let Some(key) = state.current_key.clone()
                && let Some(user) = state.current_user.clone()
            {
                match retrieve_password(&key, &user, &text[0]) {
                    Ok((account, password)) => {
                        state.acc_pass = Some((account, password));
                        state.current_screen = Screens::PasswordScreen;
                    }
                    Err(e) => println!("Could not retrieve password because of: {}", e),
                }
            }
        }
    }
    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_signup_screen(&mut d, buttons, text, label, state.font_size);
}

fn update_add_password(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text: Vec<String> = state.input.lines().map(|x| x.to_string()).collect();
    let label = vec!["Account", "Password"];
    let buttons_width = 1.0;
    let buttons_height = 1.0;

    let no_of_buttons = 2;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * w as f32) * (0.5),
            (buttons_height * h as f32 / no_of_buttons as f32) * (1.0 - 0.4),
        ));
    }

    for i in 0..buttons.len() {
        buttons[i].x = w as f32 / 2.0 - buttons[i].width / 2.0;
        buttons[i].y = (h as f32 / buttons.len() as f32) * i as f32 + buttons[i].height / 2.0;
    }

    while let Some(key) = state.rl.get_char_pressed() {
        if key as u8 >= 32 && key as u8 <= 125 {
            state.input.push(key);
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.input.pop() {
            Some('\n') => state.input.push('\n'),
            _ => (),
        }
    }

    if state.rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        state.input.push('\n');
        if text.len() >= no_of_buttons {
            state.input.clear();
            if let Some(username) = state.current_user.clone()
                && let Some(key) = state.current_key.clone()
            {
                match add_password(&key, &username, &text) {
                    Ok(_) => state.current_screen = Screens::UserSession,
                    Err(e) => println!("Could not add password because of: {}", e),
                };
            }
        }
    }

    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_signup_screen(&mut d, buttons, text, label, state.font_size);
}

fn update_password_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    if let Some(_) = state.rl.get_key_pressed() {
        state.acc_pass = None;
        state.current_screen = Screens::UserSession;
    }
    let text = {
        if let Some((account, password)) = state.acc_pass.clone() {
            format!("The password for {} is {}", account, password)
        } else {
            format!("No account or password")
        }
    };
    let text2 = "This is not copyable";
    let text = text.as_str();
    let text_offset = state.rl.measure_text(text, 24);

    let mut d = state.rl.begin_drawing(&state.rl_thread);

    d.clear_background(Color::BLACK);

    d.draw_text(
        text,
        w / 2 - text_offset / 2,
        h / 2,
        state.font_size,
        Color::RAYWHITE,
    );
    d.draw_text(
        text2,
        w / 2 - text_offset / 2,
        h / 2 + state.font_size,
        state.font_size,
        Color::RAYWHITE,
    );
}

fn signup(input: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut key = String::new();
    let username = input[0].clone();

    let mut password = input[1].clone();
    let mut password_hasher = DefaultHasher::new();

    get_symmetric_key(&mut key, &password);
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();
    println!("\nMaster account {username} created");

    let user = username + " " + password.as_str() + "\n";

    let database = "master.db";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(database)?;
    file.write_all(user.as_bytes())?;
    Ok(key)
}

fn login<'a>(input: &mut Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut key = String::new();
    let username = input[0].clone();
    let mut password = input[1].clone();

    get_symmetric_key(&mut key, &password);
    let mut password_hasher = DefaultHasher::new();
    password.hash(&mut password_hasher);
    password = password_hasher.finish().to_string();

    let database = "master.db";
    let file = File::open(database)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let u: Vec<_> = line.split(" ").collect();
        if username == u[0] && password == u[1] {
            return Ok((username, key));
        }
    }

    Err("Can't find your account".into())
}

fn get_symmetric_key(key: &mut String, password: &String) {
    hash_key(key, password)
}

fn hash_key(key: &mut String, password: &String) {
    let mut hash: u128 = 5381;
    let password = password.clone().into_bytes().into_iter();
    for i in password {
        hash = hash.wrapping_mul(33).wrapping_add(i.into());
    }
    *key = hash.to_string();
}

fn add_password(key: &String, user: &String, input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let account = input[0].clone();

    let password = input[1].clone();

    let password = encrypt_password(key, &password);
    let response = account.clone() + " " + password.as_str() + "\n";

    let database = user.clone() + "_database.db";
    let database = database.as_str();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(database)?;
    file.write_all(response.as_bytes())?;

    println!("encryped password is {password}");
    println!("password added to {account} account");
    Ok(())
}

fn retrieve_password(
    key: &String,
    user: &String,
    input: &String,
) -> Result<(String, String), io::Error> {
    let account = input.clone();

    let database = user.clone() + "_database.db";
    let database = database.as_str();

    let file = match File::open(database) {
        Ok(x) => x,
        Err(e) => {
            println!("unable to open file: {}", e);
            return Err(e);
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{line}");
        let u: Vec<_> = line.split(" ").collect();
        if account == u[0] {
            let mut password = u[1].to_string();
            password = decrypt_password(key, &password);
            println!("The password for {account} is {password}");
            return Ok((account, password));
        }
    }
    let e = io::Error::new(io::ErrorKind::Other, "Password not found");
    Err(e)
}

fn encrypt_password(key: &String, password: &String) -> String {
    let mut p: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    let key_char = key.as_bytes().to_vec();
    for i in password.as_bytes() {
        p.push(*i as u32 * key_char[count as usize] as u32);
        count += 1;
        if count >= key_char.len() as u32 {
            count = 0;
        }
    }
    let mut enc_password = String::new();
    for (i, items) in p.iter().enumerate() {
        if i > 0 {
            enc_password.push('-');
        }
        enc_password.push_str(items.to_string().as_str());
    }
    enc_password
}
fn decrypt_password(key: &String, password: &String) -> String {
    let mut p = String::new();
    let mut count: u32 = 0;
    let key_char = key.as_bytes();
    for num_str in password.split('-') {
        println!("{num_str}");
        let num: u32 = num_str.parse().expect("Should be a number");
        let orig_char: u32 = num / key_char[count as usize] as u32;
        p.push(orig_char as u8 as char);
        count += 1;
        if count >= key_char.len() as u32 {
            count = 0;
        }
    }
    p
}
