use libc::{ECHO, ICANON, TCSANOW, tcgetattr, tcsetattr, termios};
use raylib::color::Color;
use raylib::prelude::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, BufRead, BufReader, Read, Write, stdin};
use std::os::unix::io::AsRawFd;

const WINDOW_HEIGHT: i32 = 480;
const WINDOW_WIDTH: i32 = 640;
enum Screens {
    MainScreen,
    SignupScreen,
    AccountCreatedScreen,
    LoginScreen,
    RetrievePassword,
    AddPassword,
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
    };
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
        }
        // let mut response = String::new();
        //         match get_input(
        //             r#"Welcome to Cruz Password Manager
        // press 1 to create an account
        // press 2 to login to an account
        // press q to quit
        // "#,
        //             &mut response,
        //         ) {
        //             Ok(_) => (),
        //             Err(e) => println!("Could not get input due to: {}", e),
        //         }
        //
        //         let mut key = String::new();
        //         let mut user_name = String::new();
        //         if response == "1" {
        //             match signup(&mut key) {
        //                 Err(e) => println!("Couldn't signup: {}", e),
        //                 _ => (),
        //             }
        //         } else if response == "2" {
        //             let login_sesh = login(&mut key, &mut user_name);
        //             match login_sesh {
        //                 Ok(_) => user_session(&key, &user_name),
        //                 Err(e) => println!("{}", e),
        //             }
        //         } else if response.to_lowercase() == "q" {
        //             println!("Thanks for using the Password Manager");
        //             break;
        //         } else {
        //             println!("{response} is not an option");
        //         }
    }
}

fn draw_main_screen(d: &mut RaylibDrawHandle, buttons: Vec<Rectangle>, text: Vec<&str>) {
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
            24,
            Color::RAYWHITE,
        );
    }
}
fn draw_update_screen(d: &mut RaylibDrawHandle, buttons: Vec<Rectangle>, text: Vec<String>) {
    let roundness = 5.0;
    let segments = 12;
    let color = Color::BLACK;

    d.clear_background(Color::BLUE);
    for i in 0..buttons.len() {
        d.draw_rectangle_rounded(buttons[i], roundness, segments, color);
        if i < text.len() {
            d.draw_text(
                text[i].as_str(),
                buttons[i].x as i32 + buttons[i].width as i32 / 10,
                buttons[i].y as i32 + buttons[i].height as i32 / 2,
                24,
                Color::RAYWHITE,
            );
        }
    }
}

fn draw_signup_screen(d: &mut RaylibDrawHandle, buttons: Vec<Rectangle>, text: Vec<String>) {
    let roundness = 5.0;
    let segments = 12;
    let color = Color::BLACK;

    d.clear_background(Color::BLUE);
    for i in 0..buttons.len() {
        d.draw_rectangle_rounded(buttons[i], roundness, segments, color);
        if i < text.len() {
            d.draw_text(
                text[i].as_str(),
                buttons[i].x as i32 + buttons[i].width as i32 / 10,
                buttons[i].y as i32 + buttons[i].height as i32 / 2,
                24,
                Color::RAYWHITE,
            );
        }
    }
}
fn draw_login_screen() {
    ()
}
fn draw_usersession_screen() {
    ()
}

fn update_main_screen(state: &mut State, quit: &mut bool) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text = vec!["Sign Up", "Login", "Quit"];
    let buttons_width = 2.0;
    let buttons_height = 1.0;

    let no_of_buttons = 3;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * h as f32 / no_of_buttons as f32) * (1.0 - 0.2),
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
    draw_main_screen(&mut d, buttons, text);
}

fn update_signup_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text: Vec<String> = state.input.lines().map(|x| x.to_string()).collect();
    let buttons_width = 2.0;
    let buttons_height = 1.0;

    let no_of_buttons = 3;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * h as f32 / no_of_buttons as f32) * (1.0 - 0.2),
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
    draw_signup_screen(&mut d, buttons, text);
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

    d.draw_text(text, w / 2 - text_offset / 2, h / 2, 24, Color::RAYWHITE);
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

    d.draw_text(text, w / 2 - text_offset / 2, h / 2, 24, Color::RAYWHITE);
}

fn update_login_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let mut text: Vec<String> = state.input.lines().map(|x| x.to_string()).collect();
    let buttons_width = 2.0;
    let buttons_height = 1.0;

    let no_of_buttons = 2;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * h as f32 / no_of_buttons as f32) * (1.0 - 0.2),
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
            let login_sesh = login(state, &mut text);
            match login_sesh {
                Ok((username, key)) => {
                    // user_session(&key, &username);
                    state.current_user = Some(username);
                    state.current_key = Some(key);
                    state.current_screen = Screens::WelcomeScreen;
                }
                Err(e) => println!("{}", e),
            }
        }
    }
    let mut d = state.rl.begin_drawing(&state.rl_thread);
    draw_update_screen(&mut d, buttons, text);
}

fn update_usersession_screen(state: &mut State) {
    let w = state.rl.get_render_width();
    let h = state.rl.get_render_height();
    let mut buttons: Vec<Rectangle> = vec![];
    let text = vec!["Add password", "Retrieve Password", "Back"];
    let buttons_width = 2.0;
    let buttons_height = 1.0;

    let no_of_buttons = 3;

    for _i in 0..no_of_buttons {
        buttons.push(Rectangle::new(
            0.0,
            0.0,
            (buttons_width * h as f32 / no_of_buttons as f32) * (1.0 - 0.2),
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
    draw_main_screen(&mut d, buttons, text);
}

fn update_retrieve_password(state: &mut State) {
    ()
}

fn update_add_password(state: &mut State) {
    ()
}

fn signup(input: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut key = String::new();
    let username = input[0].clone();

    let mut password = input[1].clone();
    let mut password_hasher = DefaultHasher::new();
    // get_input("Input your username", &mut username)?;
    // get_masked_input("Please input a password", &mut password)?;

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

fn login<'a>(
    state: &mut State,
    input: &mut Vec<String>,
) -> Result<(String, String), Box<dyn Error>> {
    let mut key = String::new();
    let username = input[0].clone();
    let mut password = input[1].clone();

    // get_input("Input your username", &mut username)?;
    // get_masked_input("Input your password", &mut password)?;

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
            // *user_name = username.clone();
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

fn user_session(key: &String, user: &String) {
    loop {
        let mut response = String::new();
        match get_input(
            "press 1 to add a password\npress 2 to retrieve a password\npress b to go back",
            &mut response,
        ) {
            Ok(_) => (),
            Err(e) => {
                println!("error getting input: {}", e);
                continue;
            }
        }

        if response == "1" {
            match add_password(key, user) {
                Ok(_) => (),
                Err(e) => println!("Could not add password because of: {}", e),
            };
        } else if response == "2" {
            match retrieve_password(key, user) {
                Ok(_) => (),
                Err(e) => println!("Could not retrieve password because of: {}", e),
            }
        } else if response == "b" {
            break;
        } else {
            println!("{response} is not an option");
        }
    }
}

fn add_password(key: &String, user: &String) -> Result<(), Box<dyn Error>> {
    let mut account = String::new();
    get_input(
        "\nWhat is the name of the account associated with the password",
        &mut account,
    )?;

    let mut password = String::new();
    println!("Type in the passowrd");
    get_masked_input("Type in the password", &mut password)?;

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

fn retrieve_password(key: &String, user: &String) -> Result<(), io::Error> {
    let mut account = String::new();
    get_input(
        "\nWhat is the name of the account associated with the password",
        &mut account,
    )?;

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
        }
    }
    Ok(())
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

fn set_raw_mode(enable: bool) {
    unsafe {
        let fd = io::stdin().as_raw_fd();
        let mut term: termios = std::mem::zeroed();
        tcgetattr(fd, &mut term);

        if enable {
            term.c_lflag &= !(ICANON | ECHO);
        } else {
            term.c_lflag |= ICANON | ECHO;
        }

        tcsetattr(fd, TCSANOW, &term);
    }
}

fn get_masked_input(query: &str, input: &mut String) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    *input = String::new();

    println!("{query}");
    stdout.flush()?;

    set_raw_mode(true);

    let mut buf = [0u8; 1];
    loop {
        stdin.read_exact(&mut buf)?;
        let ch = buf[0] as char;

        if ch == '\n' || ch == '\r' {
            break;
        } else if ch == '\x08' || ch == '\x7f' {
            if !input.is_empty() {
                input.pop();
                print!("\x08 \x20 \x08");
                stdout.flush()?;
            }
        } else {
            input.push(ch);
            print!("*");
            stdout.flush()?;
        }
    }

    set_raw_mode(false);
    println!("\nthe input typed is {input}");
    Ok(())
}

fn get_input(query: &str, output: &mut String) -> io::Result<()> {
    println!("{query}");
    stdin().read_line(output)?;
    *output = output.trim().to_string();
    Ok(())
}
