use std::time::Instant;

use sdl2::event::{Event, WindowEvent};

/* 
use ubi::core::math::transform::*;
use ubi::graphics::windsdl::Windsdl;
use ubi::graphics::objects::*;
use ubi::core::custom_error::UbiError;
*/

use ubi::{graphics::window::window_trait::Window, prelude::*};
use ubi::graphics::window::window_trait::WindowData;

fn main() {
    ubi::core::logger::init();
    //test_error();
    engine_loop();
}

fn test_error() -> Result<(), UbiError> {
    appinfo!("rrrrr");
    apperror!("rrrrr");
    return Err(UbiError::Other(String::from("eeeee")))
}


fn engine_loop() {
    let window_data = WindowData {
        name: "rusty",
        width: 800,
        height: 600,
    };
    let mut windsdl = Windsdl::create(window_data).unwrap();
    unsafe { gl::Viewport(0, 0, 800, 600) }
    
    let program = create_program().unwrap();
    program.set();
    
    let (mut vertices, mut indices) = gem_triangle();
    
    let vbo = Vbo::gen();
    vbo.set(&vertices);
    
    let vao = Vao::gen();
    vao.set();
    
    let ibo = Ibo::gen();
    ibo.set(&indices);
    
    let texture: Texture = Texture::gen().unwrap();
    texture.setup("../assets/textures/wall.jpg");
    
    let texture2: Texture = Texture::gen().unwrap();
    texture2.setup("../assets/textures/awesomeface.png");
    
    let mut model_matrix: Mat4 = Mat4::new();
    let mut view_matrix: Mat4 = Mat4::new();
    
    let u_time = Uniform::new(program.id(), "u_time").unwrap();
    let u_model = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_texture = Uniform::new(program.id(), "u_texture").unwrap();
    let u_texture2 = Uniform::new(program.id(), "u_texture2").unwrap();
    
    unsafe { 
        gl::Uniform1f(u_time.id, 0.0);
        gl::UniformMatrix4fv(u_model.id, 1, gl::TRUE, model_matrix.ptr());
        gl::UniformMatrix4fv(u_view.id, 1, gl::TRUE, view_matrix.ptr());
    }
    
    let start = Instant::now();
    
    'running: loop {
        for event in windsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, 
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, width, height);
                        }
                    }
                }
                
                _ => { }, 
            }
        }
        
        unsafe {
            gl::ClearColor(20./255., 20./255., 20./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();
            gl::Uniform1i(u_texture.id, 0);
            gl::ActiveTexture(gl::TEXTURE1);
            texture2.bind();
            gl::Uniform1i(u_texture2.id, 1);
            
            (vertices, indices) = gem_triangle();
            vbo.set(&vertices);
            ibo.set(&indices);
            
            model_matrix = Mat4::new();
            view_matrix = Mat4::new();
            
            gl::Uniform1f(u_time.id, start.elapsed().as_secs_f32());
            gl::UniformMatrix4fv(u_model.id, 1, gl::TRUE, model_matrix.ptr());
            gl::UniformMatrix4fv(u_view.id, 1, gl::TRUE, view_matrix.ptr());
            gl::DrawElements(
                gl::TRIANGLES, 
                indices.len() as i32, 
                gl::UNSIGNED_INT, 
                0 as *const _
            );
        }
        windsdl.swap_buffers();
    }
}


fn gem_triangle() -> (Vec<f32>, Vec<u32>) {
    //vertice: x, y, z,  uv.x, uv.y 
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,   0.0, 0.0, // bottom left
        0.5 , -0.5, 0.0,   1.0, 0.0,  // bottom right
        0.0 ,  0.5, 0.0,   0.5, 1.0,   // top  
         
    ];
    let indices: Vec<u32> = vec![
        0, 1, 2,
    ];

    (vertices, indices)
}

