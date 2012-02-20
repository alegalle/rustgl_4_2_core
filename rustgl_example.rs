/************************************************************************
 * Example program for rustgl_4_2_core bindings
 * 
 * Not great code, but gives a good example of how to set up a 
 * gl 3.2 onwards context with rust.
 * 
 * You need the glfw bindings from cargo central, the glfw library
 * and a opengl 3.2 or greater driver and library.
 * 
 * 
 *************************************************************************/


use std;
use glfw;
use rustgl_4_2_core;

import glfw::*;
import rustgl_4_2_core::*;
import rustgl_4_2_core::GL::*;
import tuple::*;

fn readFile (file : str) -> result::t<[u8], str>
{
    let r = std::io::file_reader(file);
    let out : [u8] = [];
    
    alt r {
        result::err(e) { 
            ret result::err(#fmt("%s\n", e));
        }
        result::ok(f) { 
            while !f.eof() 
            { 
                out += f.read_bytes(2048u); 
            } 
            out += [0u8];
            ret result::ok(out); 
        }
    }      
}

fn loadShaders() -> result::t<uint, str>
{    
    let vert_shader = 0u;
    let frag_shader = 0u;
    let program = 0u;
        
    program = glCreateProgram() as uint;
    
    let vert = readFile("test.vs");
    
    alt vert {
        result::err(e) {  std::io::print(e); ret result::err(#fmt("%s\n", e)) }
        result::ok(s) 
        {
            unsafe {                
                vert_shader = glCreateShader(GL_VERTEX_SHADER) as uint;
                let length = vec::len(s) as int;
                let source = vec::unsafe::to_ptr(s);
                glShaderSource(vert_shader, 1, ptr::addr_of(source), ptr::addr_of(length));
                glCompileShader(vert_shader);
                
                let result = 0;
                glGetShaderiv(vert_shader, GL_COMPILE_STATUS, ptr::addr_of(result));
                
                if(result as GLboolean == GL_FALSE) {
                    let length = 0;
                    glGetShaderiv(vert_shader, GL_INFO_LOG_LENGTH, ptr::addr_of(length));
                    let mes : [u8] = [];
                    vec::grow(mes, length as uint, 0u8);
                    glGetShaderInfoLog(vert_shader as GLuint, length as GLsizei, ptr::addr_of(length) as *GLsizei, vec::unsafe::to_ptr(mes));
                    glDeleteShader(vert_shader);
                    glDeleteProgram(program);
                    
                    ret result::err(#fmt("Could not compile vertex shader\n%s\n", str::from_bytes(mes)))
                }
                
                glAttachShader(program, vert_shader);
                glDeleteShader(vert_shader);
            }
        }
    }
    
    let frag = readFile("test.fs");
    
    alt frag {
        result::err(e) {  std::io::print(e); ret result::err(#fmt("%s\n", e)) }
        result::ok(s)
        {
            unsafe {
                frag_shader = glCreateShader(GL_FRAGMENT_SHADER) as uint;
                let length = vec::len(s) as int;
                let source = vec::unsafe::to_ptr(s);
                glShaderSource(frag_shader, 1, ptr::addr_of(source), ptr::addr_of(length));
                glCompileShader(frag_shader);
                
                let result = 0;
                glGetShaderiv(frag_shader, GL_COMPILE_STATUS, ptr::addr_of(result));
                
                if(result as GLboolean == GL_FALSE) {
                    let length = 0;
                    glGetShaderiv(frag_shader, GL_INFO_LOG_LENGTH, ptr::addr_of(length));
                    let mes : [u8] = [];
                    vec::grow(mes, length as uint, 0u8);
                    glGetShaderInfoLog(vert_shader as GLuint, length as GLsizei, ptr::addr_of(length) as *GLsizei, vec::unsafe::to_ptr(mes));
                    glDeleteShader(vert_shader);
                    glDeleteShader(frag_shader);
                    glDeleteProgram(program);
                    
                    ret result::err(#fmt("Could not compile fragment shader\n%s\n", str::from_bytes(mes)))
                }
                
                glAttachShader(program, frag_shader);
                glDeleteShader(frag_shader);
            }
        }
    }
    
    glLinkProgram(program);
    
    let result = 0;
    glGetProgramiv(program, GL_LINK_STATUS, ptr::addr_of(result));
    
    if(result as GLboolean == GL_FALSE) {
        unsafe {
            let length = 0;
            glGetProgramiv(program, GL_INFO_LOG_LENGTH, ptr::addr_of(length));
            let mes : [u8] = [];
            vec::grow(mes, length as uint, 0u8);
            glGetShaderInfoLog(vert_shader as GLuint, length as GLsizei, ptr::addr_of(length) as *GLsizei, vec::unsafe::to_ptr(mes));
            glDeleteShader(vert_shader);
            glDeleteShader(frag_shader);
            glDeleteProgram(program);
            
            ret result::err(#fmt("Could not link program\n%s\n", str::from_bytes(mes)));
        }
    }
    
    ret result::ok(program);
}

fn setPerspective (w : int, h : int, fovy : f32, zNear : f32, zFar : f32) -> [f32]
{
    let aspect = (w as f32) / (h as f32);
    let ymax = zNear * f32::tan(fovy * f32::consts::pi / 360.0f32);
    let ymin = -ymax;
    let xmin = ymin * aspect;
    let xmax = ymax * aspect;

    let r = [ 1.0f32, 0.0f32, 0.0f32, 0.0f32,
              0.0f32, 1.0f32, 0.0f32, 0.0f32,
              0.0f32, 0.0f32, 1.0f32, 0.0f32,
              0.0f32, 0.0f32, 0.0f32, 1.0f32 ];
    let r = vec::to_mut(r);

    r[0] = 2.0f32 * zNear / (xmax - xmin);
    r[1] = 0.0f32;
    r[2] = 0.0f32;
    r[3] = 0.0f32;
    r[4] = 0.0f32;
    r[5] = 2.0f32 * zNear / (ymax - ymin);
    r[6] = 0.0f32;
    r[7] = 0.0f32;
    r[8] = (xmax + xmin) / (xmax - xmin);
    r[9] = (ymax + ymin) / (ymax - ymin);
    r[10] = -(zFar + zNear) / (zFar - zNear);
    r[11] = -1.0f32;
    r[12] = 0.0f32;
    r[13] = 0.0f32;
    r[14] = -(2.0f32 * zFar * zNear) / (zFar - zNear);
    r[15] = 0.0f32;

    ret vec::from_mut(r);
}

fn checkSize(w : @mutable int, h : @mutable int) -> bool
{    
    let w1 = @0;
    let h1 = @0;
    glfwGetWindowSize(w1,h1);
    
    if (*w!=*w1 || *h!=*h1)
    {
        *w = *w1;
        *h = *h1;
        ret true;
    }
    
    ret false;
}

fn initGL ()
{
    glClearColor(0.0f32, 0.6f32, 0.9f32, 1.0f32);
    glClearDepth (1.0f);									
	glDepthFunc (GL_LEQUAL);									
	glEnable (GL_DEPTH_TEST);									
	glFrontFace(GL_CCW);
	glEnable(GL_CULL_FACE);	
}

impl c_str for str {
    fn c_str() -> (@[u8], *u8)
    {
        let bytes = str::bytes(self);
        bytes += [0u8];
        let pbytes = @bytes;
        unsafe { ret (pbytes, vec::unsafe::to_ptr(*pbytes)); }
    }
}

fn main ()
{
    /**** Init window ****/
	if (glfwInit() == 0)
	{
		fail("glfwInit() failed\n");
	}
	
	if (glfwOpenWindow(800, 600, 5, 6, 5, 0, 0, 0, GLFW_WINDOW) == 0)
	{
		fail("glfwOpenWindow() failed\n");
	}
    /****/
    
    
	
    /**** Get OpenGL version info ****/
	let major : @int = @0;
	let minor : @int = @0;
	let rev   : @int = @0;
	glfwGetGLVersion(major, minor, rev);
	
	let title = #fmt("Opengl version - %d.%d rev %d", *major, *minor, *rev);
	glfwSetWindowTitle(title);
    /****/
    
    
    
    /**** Load shaders ****/
    let rprogram = loadShaders();
    let program : uint;
    
    alt rprogram {
        result::err(e) {  std::io::print(e); glfwTerminate(); ret }
        result::ok(s) { program = s; glUseProgram(program); }
    }
    /****/
    
    
    /**** Set Shader Uniforms ****/
    let proj : [f32] = [];
    let uProj = glGetUniformLocation( program, second("projectionMatrix".c_str()) ) as uint;
    let uVert = glGetAttribLocation ( program, second("vertex".c_str()) ) as uint;
    let uCol  = glGetAttribLocation ( program, second("colour".c_str()) ) as uint;
    /****/
    
    
    /**** Make a vbo ****/
    let tris = [-2.0f32, -1.0f32, -4.0f32, 1.0f32, 0.0f32, 0.0f32,
                 0.0f32, -1.0f32, -4.0f32, 1.0f32, 0.0f32, 0.0f32,
                -1.0f32,  1.0f32, -4.0f32, 1.0f32, 0.0f32, 0.0f32,
                 0.0f32, -1.0f32, -4.0f32, 0.0f32, 1.0f32, 0.0f32,
                 2.0f32, -1.0f32, -4.0f32, 0.0f32, 1.0f32, 0.0f32,
                 1.0f32,  1.0f32, -4.0f32, 0.0f32, 1.0f32, 0.0f32 ];
    let buf : GLuint = 0u;
    unsafe {        
        glGenBuffers(1, ptr::addr_of(buf));
        glBindBuffer(GL_ARRAY_BUFFER, buf);
        glBufferData(GL_ARRAY_BUFFER, (sys::size_of::<f32>()*vec::len(tris)) as int, vec::unsafe::to_ptr(tris) as *uint, GL_STATIC_DRAW);
        glBindBuffer(GL_ARRAY_BUFFER, buf);
        glVertexAttribPointer(uVert, 3, GL_FLOAT, GL_FALSE, (sys::size_of::<f32>()*6u) as int, 0 as *uint);
        glEnableVertexAttribArray(uVert);
        glVertexAttribPointer(uCol, 3, GL_FLOAT, GL_FALSE, (sys::size_of::<f32>()*6u) as int, (sys::size_of::<f32>()*3u) as *uint);
        glEnableVertexAttribArray(uCol);
    }
    /****/


    /**** Setup opengl ****/
    initGL();
    /****/
    
    
    /**** Setup and run loop ****/
    let w = @mutable 0;
    let h = @mutable 0;     

    let done = false; 
    
    let then = std::time::precise_time_s();
    let fps = 0;
    
	while (!done)
	{
        /**** Check for exit conditions ****/
		if (glfwGetKey(GLFW_KEY_ESC) == GLFW_PRESS  || !glfwGetWindowParam(GLFW_OPENED) as bool)
		{
            done = true;
		}
        /****/
        
        
        /**** Check time ****/
        let now = std::time::precise_time_s();
        if ( now - then >= 0.1 )
        {
            let title = #fmt("Opengl version - %d.%d rev %d - FPS = %d", *major, *minor, *rev, fps*10);
            glfwSetWindowTitle(title);
            fps = 0;
            then = now;
        }
        fps += 1;
        /****/
        
        
        /**** Check for changes in window size ****/
        if (checkSize(w,h))
        {
            proj = setPerspective(*w, *h, 40.0f32, 1.0f32, 100.0f32);
            glViewport(0, 0, *w, *h);  
            unsafe { glUniformMatrix4fv(uProj as int, 1, GL_FALSE, vec::unsafe::to_ptr(proj)); }
        }
        /****/
        
        
        
        /**** Render ****/
        glClear( GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT ); 
        
        glDrawArrays(GL_TRIANGLES, 0, 6 ); 
        
		glfwSwapBuffers();
        /****/
        
        
    }  
    /****/
    
    
    glDeleteBuffers(1, ptr::addr_of(buf));

	glfwTerminate();
}
