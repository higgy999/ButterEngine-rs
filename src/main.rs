mod window;
use window::Window;
mod shader;
use shader::Shader;
use gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

fn main() {
    //TODO: Load settings from text file
    let mut window = Window::new(1280, 720, "Butter Engine", glfw::WindowMode::Windowed, glfw::SwapInterval::Sync(1));
    window.center();
    window.init_gl();

    let shader: Shader = Shader::new("triangle".to_string());

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
         0.5, -0.5, 0.0, // right
         0.0,  0.5, 0.0  // top
    ];

    let VAO = unsafe {
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        
        let (mut VBO, mut VAO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        VAO
    };

    while !window.should_close() {
        //TODO: Render
        //TODO: ecs/physics/sound/ui/engine updates

        window.process_events();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader.bind();
        unsafe {
            // draw our first triangle
            //gl::UseProgram(shaderProgram);
            gl::BindVertexArray(VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // glBindVertexArray(0); // no need to unbind it every time
        }
        shader.unbind();

        window.swap_buffers();
        window.poll_events();
    }

    shader.cleanup();
}


